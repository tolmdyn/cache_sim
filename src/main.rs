use crate::fileio::process_input_file;

mod cache;
mod fileio;

const FILENAME: &str = "../traces/yi.trace";

pub fn main() {
    let cache = process_input_file(FILENAME, 4, 4, 2);

    println!("{}", cache.unwrap().cache_results());
}









