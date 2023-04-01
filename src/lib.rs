use getopts::{ Options };
use std::{ fmt, error::{Error, self}, fs, io:: {BufRead, BufReader}};

#[derive(Debug)]
pub struct ArgFlags {
    pub v: bool,
    pub s: u64,
    pub b: u64,
    pub e: u32,
    pub t: String,
}
#[derive(Debug, Clone, Copy)]
pub struct Cmd {
    pub inst: CacheInstruction,
    pub address: u64,
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
        if cmd.is_none() || cmd.unwrap().inst == CacheInstruction::Instruction { continue }
        let result = cache.run_command(cmd.unwrap());
        if verbose {
            println!("{} {}", &line[1..], result.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
        }
    }
    Ok(())
}

pub fn line_to_command(line: &str) -> Option<Cmd> {
    let item: Vec<&str> = line.split([' ', ',']).filter(|&x| x != "").collect();
    if item.len() == 0 {return None}
    let inst = match str_to_inst(item[0]) {
        Some(i) => i,
        None => return None 
    };
    let address = u64::from_str_radix(item[1], 16).unwrap();
    Some(Cmd { inst, address })
}

fn str_to_inst(c: &str) -> Option<CacheInstruction> {
    match c {
        "I" => Some(CacheInstruction::Instruction),
        "L" => Some(CacheInstruction::Load),
        "S" => Some(CacheInstruction::Store),
        "M" => Some(CacheInstruction::Modify),
        _ => None
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
        print_usage(&program, &opts);
        std::process::exit(1);
    }
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => fail_with_message(&format!("Error: {}", e.to_string()), &program, &opts), 
    };
    let v_flag =  matches.opt_present("v");

    let s_flag:u64 = match matches.opt_get("s"){
        Ok(f) => f.unwrap(),
        Err(_e) => fail_with_message("Missing required command line argument -s", &program, &opts)
    };
    let b_flag:u64 = match matches.opt_get("b"){
        Ok(f) => f.unwrap(),
        Err(_e) => fail_with_message("Missing required command line argument -b", &program, &opts)
    };
    let e_flag:u32 = match matches.opt_get("E"){
        Ok(f) => f.unwrap(),
        Err(_e) => fail_with_message("Missing required command line argument -E", &program, &opts)
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

pub fn fail_with_message(message: &str, program: &str, opts: &Options) -> ! {
    eprintln!("{}", message);
    print_usage(&program, &opts);
    std::process::exit(1);
}

fn print_usage(program: &str, opts: &Options) {
    print!("{}", opts.usage(&format!("Usage: {} [-hv] -s <num> -E <num> -b <num> -t <file>", program)));
}

/* CACHE */
#[derive(PartialEq, Debug)]
pub enum CacheResult {
    Hit,
    Miss,
    Eviction,
}

impl fmt::Display for CacheResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CacheResult::Hit => write!(f, "hit"),
            CacheResult::Miss => write!(f, "miss"),
            CacheResult::Eviction => write!(f, "eviction"),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum CacheInstruction {
    Load,
    Store,
    Modify,
    Instruction
}

struct Address { //we ignore address and block in this simulation
    tag: u64,
    set: u64,
}

#[derive(Debug)]
pub struct Cache {
    pub sets: Vec<Vec<u64>>, 
    set_bits: u64,  
    block_bits: u64, 
    miss: u32,
    hit: u32,
    evict: u32,
}

impl Cache {
    /* Create a new cache from parameters */
    pub fn new(set_bits: u64, block_bits: u64, num_lines: u32) -> Cache {
        let set_num = 1 << set_bits;
        let mut new_sets = Vec::with_capacity(set_num);
        for _ in 0..set_num {
            new_sets.push(Vec::with_capacity(num_lines as usize));
        }

        Self {
            sets: new_sets,
            set_bits,
            block_bits,
            miss: 0,
            hit: 0,
            evict: 0,
        }
    }

    /* Perform single cache access */
    pub fn operate(&mut self, addr: u64) -> Vec<CacheResult> {
        let address = self.process_address(addr);
        let mut result = Vec::new();
        if self.check_hit(&address) {
            result.push(CacheResult::Hit);
            self.update(&address);
        } else {
            result.push(CacheResult::Miss);
            if self.check_free(&address) {
                self.insert(&address);
            } else {
                result.push(CacheResult::Eviction);
                self.evict(&address);
            }
        }
        result
    }

    /* Execute an instruction on the cache, return a vector containing the results */
    pub fn run_command(&mut self, command: Cmd) -> Vec<CacheResult> {
        if command.inst == CacheInstruction::Load || command.inst == CacheInstruction::Store {
            self.operate(command.address)
        } else if command.inst == CacheInstruction::Modify { //modify must operate cache twice
            let mut x = self.operate(command.address);
            x.extend(self.operate(command.address));
            x
        } else {
            vec!() //nothing happened so return an empty vector 
        }
    }

    /* Process a raw address integer into an Address enum */
    fn process_address(&self, addr: u64) -> Address {
        let setmask: u64 = (1 << self.set_bits + self.block_bits) - 1;
        Address {
            tag : addr >> (self.block_bits + self.set_bits),
            set : (addr & setmask) >> self.block_bits,
        }
    }

    /* Check if the tag is in cache */
    fn check_hit(&mut self, addr: &Address) -> bool {
        if let Some(set) = self.sets.get(addr.set as usize) {
            if set.contains(&addr.tag) {
                self.hit += 1;
                return true;
            } else {
                self.miss += 1;
                return false;
            }
        }
        panic!("Problem checking for hit");
    }
    /* Re-insert tag to update the LRU */
    fn update(&mut self, addr: &Address) {
        if let Some(set) = self.sets.get_mut(addr.set as usize) {
            let index = set.iter().position(|&x| x == addr.tag);
            set.remove(index.unwrap());
            set.push(addr.tag);
            return;
        }
        panic!("Cache does not contain address.");
    }

    /* Check for a free space in the set */
    fn check_free(&self, addr: &Address) -> bool {
        if let Some(set) = self.sets.get(addr.set as usize) {
            return &set.len() < &set.capacity();
        }
        panic!("Problem checking for a free space");
    }

    /* Insert the tag into set */
    fn insert(&mut self, addr: &Address) {
        if let Some(set) = self.sets.get_mut(addr.set as usize) {
            set.push(addr.tag);
            return;
        }
        panic!("Cannot insert into cache");
    }

    /* Evict the LRU tag from a test */
    fn evict(&mut self, addr: &Address) {
        if let Some(set) = self.sets.get_mut(addr.set as usize) {
            if &set.len() == &set.capacity() {
                set.remove(0);
                set.push(addr.tag);
                self.evict += 1;
                return;
            }
        }
        panic!("Cannot evict from cache");
    }

    pub fn cache_results(&self) -> String {
        format!( "hits:{} misses:{} evictions:{}", self.hit, self.miss, self.evict)
    }
}