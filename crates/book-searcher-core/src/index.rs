use crate::{always_merge_policy::AlwaysMergePolicy, Book, Searcher};
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use log::info;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};
use sysinfo::{System, SystemExt};
use tantivy::doc;

fn get_memory_arena_num_bytes() -> usize {
    let sys = System::new_all();
    let available_memory = sys.available_memory() as usize;
    let cpu_num = sys.cpus().len();
    info!("Your system has cpu {cpu_num} cores and {available_memory} Bytes available");

    let chunk_size = 1024 * 1024 * 1024; // 1GB
    let total_num_chunk = available_memory / chunk_size;

    let s = if total_num_chunk < 2 {
        // <2G
        available_memory - 100 * 1024 * 1024 // available_memory-100MB
    } else {
        // >2G
        available_memory * (total_num_chunk - 1) // available_memory-1GB
    };

    let num_threads = std::cmp::min(cpu_num, 8);
    let s = std::cmp::min(s, num_threads * 4293967294);

    info!("Using {num_threads} threads and {s} Bytes to do index");
    s
}

impl Searcher {
    pub fn index(&mut self, csv_file: impl AsRef<Path>) {
        let mut writer = self.index.writer(get_memory_arena_num_bytes()).unwrap();
        writer.set_merge_policy(Box::new(AlwaysMergePolicy));

        let file = File::open(&csv_file).unwrap();
        let reader = BufReader::new(file);

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(reader);

        let line_count = BufReader::new(File::open(&csv_file).unwrap())
            .lines()
            .count();

        let style = ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap();
        let bar = ProgressBar::new(line_count as u64)
            .with_message(format!("Indexing {}", csv_file.as_ref().to_str().unwrap()))
            .with_style(style);

        for result in rdr.deserialize::<Book>().progress_with(bar) {
            match result {
                Ok(item) => {
                    if let Err(err) = writer.add_document(doc!(
                        self.id => item.id,
                        self.title => item.title,
                        self.author => item.author,
                        self.publisher_exist => !item.publisher.is_empty(),
                        self.publisher => item.publisher,
                        self.extension => item.extension,
                        self.filesize => item.filesize,
                        self.language => item.language,
                        self.year => item.year,
                        self.pages => item.pages,
                        self.isbn => item.isbn,
                        self.ipfs_cid => item.ipfs_cid,
                    )) {
                        println!("{err}");
                    }
                }
                Err(err) => {
                    println!("{err}");
                }
            }
        }

        writer.commit().unwrap();
        writer.wait_merging_threads().expect("merge complete");
    }

    pub fn index_background(&mut self, csv_file: impl AsRef<Path>) -> ProgressBar {
        let searcher = self.to_owned();

        let mut writer = self.index.writer(get_memory_arena_num_bytes()).unwrap();

        let file = File::open(&csv_file).unwrap();
        let reader = BufReader::new(file);

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(reader);

        let line_count = BufReader::new(File::open(&csv_file).unwrap())
            .lines()
            .count();

        let style = ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap();
        let bar = ProgressBar::new(line_count as u64)
            .with_message(format!("Indexing {}", csv_file.as_ref().to_str().unwrap()))
            .with_style(style);

        let bar_background = bar.clone();
        std::thread::spawn(move || {
            for result in rdr.deserialize::<Book>().progress_with(bar_background) {
                match result {
                    Ok(item) => {
                        if let Err(err) = writer.add_document(doc!(
                            searcher.id => item.id,
                            searcher.title => item.title,
                            searcher.author => item.author,
                            searcher.publisher_exist => !item.publisher.is_empty(),
                            searcher.publisher => item.publisher,
                            searcher.extension => item.extension,
                            searcher.filesize => item.filesize,
                            searcher.language => item.language,
                            searcher.year => item.year,
                            searcher.pages => item.pages,
                            searcher.isbn => item.isbn,
                            searcher.ipfs_cid => item.ipfs_cid,
                        )) {
                            println!("{err}");
                        }
                    }
                    Err(err) => {
                        println!("{err}");
                    }
                }
            }

            writer.commit().unwrap();
            writer.wait_merging_threads().expect("merge complete");
        });

        bar
    }
}

#[test]
fn test_csv_der() {
    let file = File::open("books.csv").unwrap();
    let reader = BufReader::new(file);

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(reader);
    for result in rdr.records() {
        if let Err(err) = result {
            println!("{err:?}");
            break;
        }
    }
    println!("{:?}", rdr.position());
}
