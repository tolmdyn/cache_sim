use crate::fileio::process_input_file;

mod cache;
mod fileio;

const FILENAME: &str = "../traces/yi.trace";

pub fn main() {
    let mut cache = cache::Cache::new(4, 4, 2);

    process_input_file(FILENAME, &mut cache, true).unwrap();

    println!("{}", cache.cache_results());
}
