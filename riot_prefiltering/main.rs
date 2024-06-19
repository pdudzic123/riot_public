mod io;
mod model;
mod prefiltering;
mod tasks;

use std::{
    fs::{create_dir_all, File},
    io::{BufRead, BufReader},
    path::Path,
};

use itertools::Itertools;
use tasks::{run_grid_search_prefiltering, run_prefiltering, GridSearchParams};

#[macro_use]
extern crate itertools;

fn main() {
    let number_threads: usize = std::env::args()
        .nth(1)
        .unwrap_or("0".to_string())
        .parse()
        .unwrap();

    rayon::ThreadPoolBuilder::new()
        .num_threads(number_threads)
        .build_global()
        .unwrap();

    let gene = "v";
    for locus in ["igh"] {
        let path: String = format!("./data/genes/{}_genes/{}_genes.fasta", gene, gene);
        let genes_fasta_path = Path::new(&path[..]);
        let path: String = format!("./data/ngs_stratified/ngs_sample_clean_no_trash.fasta");
        let input_path = Path::new(&path[..]);
        let path = format!("./results/skbio_stratified_ngs_alignment_grid_search_v/");
        let output_dir = Path::new(&path[..]);
        create_dir_all(output_dir).unwrap();

        let num_seq;
        {
            let file = BufReader::new(File::open(input_path).expect("Unable to open file"));
            num_seq = file.lines().count() / 2
        }
        // let duration = run_prefiltering(
        //     genes_fasta_path,
        //     input_path,
        //     output_dir,
        //     11,
        //     19,
        //     2,
        //     10,
        //     num_seq,
        // );

        // println!("{}", duration.as_secs().to_string());

        let grid_params: GridSearchParams = GridSearchParams {
            kmer_size: (7..=15).step_by(2).collect_vec(),
            distance_threshold: (3..=19).step_by(2).collect_vec(),
            modulo_n: (1..=5).collect_vec(),
            top_n: vec![20],
        };

        run_grid_search_prefiltering(
            genes_fasta_path,
            input_path,
            output_dir,
            grid_params,
            num_seq as u64,
        );
    }
}
