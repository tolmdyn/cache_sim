use std::env;

mod cache;
mod fileio;

use sim::{ process_args, fail_with_message };

use crate::fileio::process_input_file;
use crate::cache::Cache;

//const FILENAME: &str = "../traces/yi.trace";

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    //let flag_result = process_args(&args[1..]);

     let flags = match process_args(&args[..]) {
        Ok(f) => f,
        Err(_e) => fail_with_message(&format!("{} Missing required command line argument\nError: {:?}", args[0], _e)),
    }; 

    //ParseIntError
    //UnknownOption
    //MissingArgument


    let mut cache = Cache::new(flags.s, flags.b, flags.e);

    process_input_file(&flags.t, &mut cache, flags.v)?;

    println!("{}", cache.cache_results());

    Ok(())
}

