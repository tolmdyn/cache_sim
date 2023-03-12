//use std::{fs, io::BufReader, error};

//use crate::cache::{Cache, CacheInstruction, self};

//use std::process;


use std::fs;
use std::error;
use std::io::{BufReader, BufRead};

use crate::cache;
#[derive(Debug)]
pub struct Cmd{
    inst: cache::CacheInstruction,
    address: u64
}

//maybe this should take a mutable ref to the cache
pub fn process_input_file(filepath: &str, set_bits: u64, block_bits: u64, num_lines:u32) -> Result<cache::Cache, Box<dyn error::Error>> {
    let file = fs::File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut cache = cache::Cache::new(set_bits, block_bits, num_lines);

    for line in reader.lines(){
        let fline = line.unwrap(); //hmm

        

        let cmd = line_to_command(&fline);

        if cmd.inst == cache::CacheInstruction::Instruction {
            continue;
        }

        let result = cache.instruction(&cmd.inst, &cmd.address); //do cache stuff
        //println!("{} {:?} {:x}", fline, cmd.inst, cmd.address);
        println!("{} {:?}", &fline[1..], result);
        

    }

    Ok(cache) //return the "result"
}

fn line_to_command(line: &str) -> Cmd {
    let item: Vec<&str> = line.split([' ',',']).filter(|&x| x != "").collect();
    
    //println!("{:?}", item);

    let inst = str_to_inst(item[0]);
    let address = u64::from_str_radix(item[1], 16).unwrap();
    
    Cmd {
        inst,
        address
    }
}

fn str_to_inst(c: &str) -> cache::CacheInstruction {
    //println!("{}", c);

    match c {
        "I" => cache::CacheInstruction::Instruction,
        "L" => cache::CacheInstruction::Load,
        "S" => cache::CacheInstruction::Store,
        "M" => cache::CacheInstruction::Modify,
        _ => panic!("Bad instruction")
    }
}

#[cfg(test)]
mod tests {
    use crate::{cache::{CacheInstruction}, fileio::process_input_file};

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

        let cache = process_input_file(FILENAME, 4, 4, 2);

        println!("{}", cache.unwrap().cache_results());


    }
}