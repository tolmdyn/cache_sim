use std::error::Error;

use getopt::{Parser, Opt};

#[derive(Debug)]
pub struct ArgFlags {
    pub h:bool,
    pub v:bool,
    pub s:u64,
    pub b:u64,
    pub e:u32,
    pub t:String,
}

pub fn process_args(args: &[String]) -> Result<ArgFlags, Box<dyn Error>> { 
    let mut opts = Parser::new(args, "hvs:E:b:t:");

    let mut h_flag = false;
    let mut v_flag = false;
    let mut s_flag = String::new();
    let mut e_flag = String::new();
    let mut b_flag = String::new();
    let mut t_flag = String::new();

    loop {
        match opts.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('h', None) => h_flag = true,
                Opt('v', None) => v_flag = true,
                Opt('s', Some(arg)) => s_flag = arg.clone(),
                Opt('E', Some(arg)) => e_flag = arg.clone(),
                Opt('b', Some(arg)) => b_flag = arg.clone(),
                Opt('t', Some(arg)) => t_flag = arg.clone(),
                _=>unreachable!(),
            },
        }
    }

    if h_flag {
        print_usage();
        std::process::exit(1);
    }

    if t_flag.is_empty() {
        fail_with_message("Missing filename (-t)");
    }

    Ok(ArgFlags{
        h: h_flag,
        v: v_flag, 
        s: parse_arg_to_integer(s_flag)?,      
        b: parse_arg_to_integer(b_flag)?,
        e: parse_arg_to_integer(e_flag)?.try_into()?,
        t: t_flag,
    })
    
}

 

fn parse_arg_to_integer(s:String) -> Result<u64, Box<dyn Error>>{
    
        match s.parse::<u64>(){
            Ok(s) => Ok(s),
            Err(e) => Err(Box::new(e)),
        }
    
}


pub fn fail_with_message(message:&str) -> !{
    eprintln!("{}", message); 
    print_usage();
    std::process::exit(1);
}

pub fn print_usage() {
    println!("Usage: ../sim [-hv] -s <num> -E <num> -b <num> -t <file>
    Options:
      -h         Print this help message.
      -v         Optional verbose flag.
      -s <num>   Number of set index bits.
      -E <num>   Number of lines per set.
      -b <num>   Number of block offset bits.
      -t <file>  Trace file.
    
    Examples:
      linux>  ../sim -s 4 -E 1 -b 4 -t traces/yi.trace
      linux>  ../sim -v -s 8 -E 2 -b 4 -t traces/yi.trace")
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_args() {
        //"-s <num> -E <num> -b <num> -t <file>"
    }
}