use std::{
    error::Error,
    fs::OpenOptions,
    path::{Path, PathBuf},
};

pub fn to_csv_record<T>(
    id: String,
    query_str: String,
    rev_comp_query_str: String,
    top_genes: T,
) -> Vec<String>
where
    T: serde::Serialize,
{
    let top_matches_str = serde_json::to_string(&top_genes).unwrap();
    vec![id, query_str, rev_comp_query_str, top_matches_str]
}

pub fn write_to_csv<I>(
    path: PathBuf,
    header: Vec<&str>,
    records: I,
    append: bool,
) -> Result<(), Box<dyn Error>>
where
    I: Iterator<Item = Vec<String>>,
{
    let file_existed = path.exists();
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(append)
        .truncate(!append)
        .open(path)
        .unwrap();
    let mut wtr = csv::Writer::from_writer(file);
    if !file_existed | !append {
        wtr.write_record(header)?;
    }
    for record in records {
        wtr.write_record(record)?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn write_time_measurement(output_dir: &Path, header: Vec<&str>, record: Vec<String>) {
    write_to_csv(
        output_dir.join("time_elapsed.csv"),
        header,
        vec![record].into_iter(),
        true,
    )
    .unwrap();
}
