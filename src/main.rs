use std::env;
use sim::{ process_args, process_input_file, Cache };

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let flags = process_args(&args[..])?;
    let mut cache = Cache::new(flags.s, flags.b, flags.e);

    process_input_file(&flags.t, &mut cache, flags.v)?;
    
    println!("{}", cache.cache_results());
    Ok(())
}