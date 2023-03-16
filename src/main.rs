use std::env;

mod cache;
mod fileio;

use sim::{ process_args };

use crate::fileio::process_input_file;
use crate::cache::Cache;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    /* let flags = match process_args(&args[..]) {
        Ok(f) => f,
        Err(_e) => ,
    }; */

    let flags = process_args(&args[..])?;

    let mut cache = Cache::new(flags.s, flags.b, flags.e);

    process_input_file(&flags.t, &mut cache, flags.v)?;

    println!("{}", cache.cache_results());

    Ok(())
}

