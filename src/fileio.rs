//use std::{fs, io::BufReader, error};

//use crate::cache::{Cache, CacheInstruction, self};

//use std::process;

use std::error;
use std::fs;
use std::io::{BufRead, BufReader};

use crate::cache;
#[derive(Debug)]
pub struct Cmd {
    inst: cache::CacheInstruction,
    address: u64,
}

//maybe this should take a mutable ref to a cache created elsewhere
//pub fn process_input_file(filepath: &str, set_bits: u64, block_bits: u64, num_lines:u32) -> Result<cache::Cache, Box<dyn error::Error>> {
pub fn process_input_file(
    filepath: &str,
    cache: &mut cache::Cache,
    verbose: bool,
) -> Result<(), Box<dyn error::Error>> {
    let file = fs::File::open(filepath)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let fline = line.unwrap(); //hmm

        let cmd = line_to_command(&fline);

        if cmd.inst == cache::CacheInstruction::Instruction {
            continue;
        }

        let result = cache.run_instruction(&cmd.inst, &cmd.address);
        //println!("{} {:?} {:x}", fline, cmd.inst, cmd.address);

        if verbose {
            let result_string: Vec<String> = result.iter().map(|x| x.to_string()).collect();
            println!("{} {}", &fline[1..], result_string.join(" "));
        }
    }

    Ok(())
}

fn line_to_command(line: &str) -> Cmd {
    let item: Vec<&str> = line.split([' ', ',']).filter(|&x| x != "").collect();

    //println!("{:?}", item);

    let inst = str_to_inst(item[0]);
    let address = u64::from_str_radix(item[1], 16).unwrap();

    Cmd { inst, address }
}

fn str_to_inst(c: &str) -> cache::CacheInstruction {
    match c {
        "I" => cache::CacheInstruction::Instruction,
        "L" => cache::CacheInstruction::Load,
        "S" => cache::CacheInstruction::Store,
        "M" => cache::CacheInstruction::Modify,
        _ => panic!("Unrecognised instruction"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cache::{self, CacheInstruction},
        fileio::process_input_file,
    };

    #[test]
    fn line_to_command_test() {
        let cmd_string = "L  ffff,2";

        let cmd = crate::fileio::line_to_command(cmd_string);

        println!("{:?} {}", cmd.inst, cmd.address);

        assert_eq!(CacheInstruction::Load, cmd.inst);
        assert_eq!(0xffff, cmd.address);
    }

    #[test]
    fn process_input_file_test() {
        const FILENAME: &str = "../traces/yi.trace";

        let mut cache = cache::Cache::new(4, 4, 2);

        process_input_file(FILENAME, &mut cache, true).unwrap();

        println!("{}", cache.cache_results());
    }

    #[test]
    fn process_input_file_test_long() {
        const FILENAME: &str = "../traces/long.trace";

        let mut cache = cache::Cache::new(4, 4, 10);

        process_input_file(FILENAME, &mut cache, false).unwrap();

        println!("{}", cache.cache_results());
    }
}
