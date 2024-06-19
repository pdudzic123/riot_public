use bio::io::fasta::{self, Record};
use indicatif::ProgressBar;
use rayon::prelude::*;
use std::io::Error;
use std::path::Path;
use std::sync::mpsc::sync_channel;

use crate::io::write_time_measurement;
use crate::io::{to_csv_record, write_to_csv};
use crate::model::GeneMatch;
use crate::prefiltering::Prefiltering;
use std::time::{Duration, Instant};

fn parse_record(record_result: Result<Record, Error>) -> (String, String) {
    let record = record_result.unwrap();
    let query_id: String = record.id().to_string();
    let query: String = String::from_utf8(record.seq().to_vec()).unwrap();
    (query_id, query)
}

pub fn run_prefiltering(
    genes_fasta_path: &Path,
    input_path: &Path,
    output_dir: &Path,
    kmer_size: usize,
    distance_threshold: i32,
    modulo_n: usize,
    top_n: usize,
    num_seq: u64,
) -> Duration {
    let output_path = output_dir.join(format!(
        "top_{}_kmer_size_{}_distance_threshold_{}_modulo_{}.csv",
        top_n, kmer_size, distance_threshold, modulo_n,
    ));

    let prefiltering: Prefiltering = Prefiltering::new(
        genes_fasta_path.to_str().unwrap(),
        kmer_size,
        distance_threshold,
        top_n,
        modulo_n,
    );

    let reader = fasta::Reader::from_file(input_path);
    let progress_bar = &ProgressBar::new(num_seq);
    let (sender, receiver) = sync_channel(1024 * 1024);

    let sender_function = || {
        reader
            .unwrap()
            .records()
            .par_bridge()
            .map(parse_record)
            .map(|(query_id, query)| {
                if query.len() < kmer_size {
                    return (query_id, query, "-".to_string(), vec![]);
                }
                let prefiltering_result = prefiltering.calculate_top_matches_with_rev_comp(query);
                (
                    query_id,
                    prefiltering_result.query,
                    prefiltering_result.rev_comp_query,
                    prefiltering_result.top_matches,
                )
            })
            .for_each_with(
                sender,
                |sender, record: (String, String, String, Vec<GeneMatch>)| {
                    let (id, query, rev_comp_query, top_matches) = record;
                    let csv_record = to_csv_record(id, query, rev_comp_query, top_matches);
                    sender.send(csv_record).unwrap();
                    progress_bar.inc(1);
                },
            );
    };

    let receiver_function = move || {
        write_to_csv(
            output_path,
            vec!["sequence_id", "sequence", "rev_comp_sequence", "best_genes"],
            receiver.iter(),
            false,
        )
        .unwrap();
    };
    let start = Instant::now();

    rayon::join(sender_function, receiver_function);

    let duration = start.elapsed();

    progress_bar.finish();
    println!(
        "top_n: {}, kmer_size: {}, distance_threshold: {}, modulo: {}, time elapsed: {:?}",
        top_n, kmer_size, distance_threshold, modulo_n, duration
    );
    duration
}

pub struct GridSearchParams {
    pub kmer_size: Vec<usize>,
    pub distance_threshold: Vec<i32>,
    pub modulo_n: Vec<usize>,
    pub top_n: Vec<usize>,
}

pub fn run_grid_search_prefiltering(
    genes_fasta_path: &Path,
    input_path: &Path,
    output_dir: &Path,
    params: GridSearchParams,
    num_seq: u64,
) {
    for (kmer_size, distance_threshold, modulo_n, top_n) in iproduct!(
        params.kmer_size,
        params.distance_threshold,
        params.modulo_n,
        params.top_n
    ) {
        let duration = run_prefiltering(
            genes_fasta_path,
            input_path,
            output_dir,
            kmer_size,
            distance_threshold,
            modulo_n,
            top_n,
            num_seq,
        );

        write_time_measurement(
            output_dir,
            vec!["kmer_size", "distance_threshold", "modulo", "time_elapsed"],
            vec![
                kmer_size.to_string(),
                distance_threshold.to_string(),
                modulo_n.to_string(),
                duration.as_secs().to_string(),
            ],
        )
    }
}
