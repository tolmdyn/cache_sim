use getopts::Options;
use std::error::Error;

#[derive(Debug)]
pub struct ArgFlags {
    //pub h:bool,
    pub v: bool,
    pub s: u64,
    pub b: u64,
    pub e: u32,
    pub t: String,
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
        Err(e) => fail_with_message(&format!("Error: {}", e.to_string()), &program, opts), //edit this
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        std::process::exit(1);
    }

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

    #[test]
    fn test_args() {
        //"-s <num> -E <num> -b <num> -t <file>"
    }
}
