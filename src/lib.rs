pub mod cache;
use cache::Cache;
use getopts::{Options};
use std::{ error::{Error, self}, fs, io:: {BufRead, BufReader}};

pub struct ArgFlags {
    pub v: bool,
    pub s: u64,
    pub b: u64,
    pub e: u32,
    pub t: String,
}
pub struct Cmd {
    inst: cache::CacheInstruction,
    address: u64,
}

pub fn process_input_file(filepath: &str, cache: &mut Cache, verbose: bool) -> Result<(), Box<dyn error::Error>> {
    let file = match fs::File::open(filepath) {
        Ok(r) => r,
        Err(_) => {
            eprintln!("{}: No such file or directory ", filepath);
            std::process::exit(1);
        }
    };

    for line in BufReader::new(file).lines() {
        let line = line?;
        let cmd = line_to_command(&line);

        if cmd.inst == cache::CacheInstruction::Instruction { continue; }

        let result = cache.run_instruction(&cmd.inst, &cmd.address);

        if verbose {
            let result_string: Vec<String> = result.iter().map(|x| x.to_string()).collect();
            println!("{} {}", &line[1..], result_string.join(" "));
        }
    }
    Ok(())
}

fn line_to_command(line: &str) -> Cmd {
    let item: Vec<&str> = line.split([' ', ',']).filter(|&x| x != "").collect();
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
        _ => panic!("Unrecognised instruction in file"),
    }
}

pub fn process_args(args: &[String]) -> Result<ArgFlags, Box<dyn Error>> {
    let program = args[0].clone();
    let mut opts = Options::new();

    opts.optflag("h", "help", "Print this help message.");
    opts.optflag("v", "verbose", "Optional verbose flag.");
    opts.reqopt("s", "", "Number of set index bits.", "num");
    opts.reqopt("E", "", "Number of lines per set.", "num");
    opts.reqopt("b", "", "Number of block offset bits.", "num");
    opts.reqopt("t", "", "Trace file.", "file");

    if args.contains(&"-h".to_string()){
        print_usage(&program, opts);
        std::process::exit(1);
    }

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => fail_with_message(&format!("Error: {}", e.to_string()), &program, opts), 
    };

    let v_flag =  matches.opt_present("v");

    let s_flag:u64 = match matches.opt_get("s"){
        Ok(f) => f.unwrap(),
        Err(_e) => fail_with_message("Missing required command line argument -s", &program, opts)
    };
    
    let b_flag:u64 = match matches.opt_get("b"){
        Ok(f) => f.unwrap(),
        Err(_e) => fail_with_message("Missing required command line argument -b", &program, opts)
    };

    let e_flag:u32 = match matches.opt_get("E"){
        Ok(f) => f.unwrap(),
        Err(_e) => fail_with_message("Missing required command line argument -E", &program, opts)
    };
    
    let t_flag = matches.opt_str("t").unwrap();

    Ok(ArgFlags {
        v: v_flag,
        s: s_flag,
        b: b_flag,
        e: e_flag,
        t: t_flag,
    })
}

pub fn fail_with_message(message: &str, program: &str, opts: Options) -> ! {
    eprintln!("{}", message);
    print_usage(program, opts);
    std::process::exit(1);
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [-hv] -s <num> -E <num> -b <num> -t <file>", program);
    print!("{}", opts.usage(&brief));
}

#[cfg(test)]
mod tests {
    use crate::{
        cache::{self, CacheInstruction}, process_input_file,
    };

    #[test]
    fn test_args() {
        //"-s <num> -E <num> -b <num> -t <file>"
    }

    #[test]
    fn line_to_command_test() {
        let cmd_string = "L  ffff,2";

        let cmd = crate::line_to_command(cmd_string);

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
