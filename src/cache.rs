use std::collections::VecDeque;
use std::fmt;

/*struct Line {
    //addr: u64,
    block: u64,
    valid: bool,
    tag: u64,
    //last_access
}*/

#[derive(PartialEq)]
pub enum CacheResult {
    Hit,
    Miss,
    Eviction,
}
#[derive(PartialEq)]
#[allow(dead_code)]
pub enum CacheInstruction {
    LOAD,
    STORE,
    MODIFY,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Address {
    addr: u64,
    tag: u64,
    set: u64,
    block: u64,
}

#[derive(Debug)]
struct Set {
    lines: VecDeque<u64>, //just the tag or full Address?
}

impl Set {
    pub fn new(num_lines: u32) -> Set {
        Self {
            lines: VecDeque::with_capacity(num_lines.try_into().unwrap()),
        }
    }
}

#[derive(Debug)]
pub struct Cache {
    //sets: Vec<Set>,
    sets: Box<[Set]>,

    set_bits: u64,   //(s)
    block_bits: u64, //(b)
    num_lines: u32,  //(E)

    miss: u32,
    hit: u32,
    evict: u32,
}

impl Cache {
    pub fn new(set_bits: u64, block_bits: u64, num_lines: u32) -> Cache {
        let set_num = 1 << set_bits;

        let mut new_sets = Vec::with_capacity(set_num);

        for _ in 0..set_num {
            new_sets.push(Set::new(num_lines));
        }

        //println!("{} {}", set_num, new_sets.len());
        let set_box = new_sets.into_boxed_slice(); //cast it into a boxed slice as we don't need to resize it anymore

        Self {
            sets: set_box,
            set_bits,
            block_bits,
            num_lines,
            miss: 0,
            hit: 0,
            evict: 0,
        }
    }

    pub fn operate(&mut self, addr: u64) -> Result<CacheResult, String> {
        let address = self.process_address(addr);
        //println!("{:?}", address);

        if self.check_hit(&address).unwrap() {
            //check if address is in cache and if it is move to back of queue and update "hit"
            Ok(self.update(&address)?) //return CacheResult::Hit;
        } else if self.check_free(&address).unwrap() {
            //if there is space in the cache then add it to cache, put in back of queue and update "miss"
            Ok(self.insert(&address)?) ////return CacheResult::Miss;
            
        } else {
            //if cache is full then evict, then add it to cache and update "full"
            Ok(self.evict(&address)?) //return CacheResult::Eviction;
        }

        
    }
    /*
    pub fn instruction(&mut self, addr: u64, inst: CacheInstruction) -> &[CacheResult] {
        /*match inst {
            CacheInstruction::LOAD=> self.operate(addr),
            CacheInstruction::STORE=> self.operate(addr),
            CacheInstruction::MOD=> self.operate(addr) //and then go again...
        }*/

        if inst == CacheInstruction::MODIFY {
            &[self.operate(addr), self.operate(addr)]
        } else {
            &[self.operate(addr)]
        }

    }*/

    fn process_address(&self, addr: u64) -> Address {
        //println!("0x{:x} b{:0>64b}", addr, addr);

        let blockmask: u64 = (1 << self.block_bits) - 1;
        let block: u64 = addr & blockmask;

        let setmask: u64 = (1 << self.set_bits + self.block_bits) - 1;
        let set: u64 = (addr & setmask) >> self.block_bits;

        let tag: u64 = addr >> (self.block_bits + self.set_bits);

        Address {
            addr,
            tag,
            block,
            set,
        }
    }

    fn check_hit(&self, addr: &Address) -> Result<bool, String> {
        //check if item is in cache
        if let Some(set) = self.sets.get(addr.set as usize) {
            return Ok(set.lines.contains(&addr.addr))
        }
        Err("Problem checking for hit".to_string())
    }

    fn update(&mut self, addr: &Address) -> Result<CacheResult, String> {
        //if the item is in cache then update the LRU

        if let Some(set) = self.sets.get_mut(addr.set as usize) {
            self.hit += 1;
            let index = set.lines.iter().position(|&x| x == addr.addr);
            set.lines.remove(index.unwrap());
            set.lines.push_back(addr.addr);
            return Ok(CacheResult::Hit);
        }

        Err("Cache does not contain address.".to_string())
    }

    fn check_free(&self, addr: &Address) -> Result<bool, String> {
        if let Some(set) = self.sets.get(addr.set as usize) {
            return Ok(&set.lines.len() < &set.lines.capacity())
        }
        Err("Problem checking for a free space".to_string())
    }

    fn insert(&mut self, addr: &Address) -> Result<CacheResult, String> {
        if let Some(set) = self.sets.get_mut(addr.set as usize) {
            set.lines.push_back(addr.addr);
            self.miss += 1;
            return Ok(CacheResult::Miss)
        }
        Err("Cannot insert into cache".to_string())
    }

    fn evict(&mut self, addr: &Address) -> Result<CacheResult, String> {
        if let Some(set) = self.sets.get_mut(addr.set as usize) {
            if &set.lines.len() == &set.lines.capacity() {
                set.lines.pop_front();
                set.lines.push_back(addr.addr);
                self.evict += 1;

                return Ok(CacheResult::Eviction)
            }
        }
        Err("Cannot evict from cache".to_string())
    }
}

impl fmt::Display for Cache {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Cache : (s:{}, b:{}, e:{})",
            self.set_bits, self.block_bits, self.num_lines
        )?;

        for s in 0..self.sets.len() {
            writeln!(
                f,
                "{} ({}/{}): {:?}",
                s,
                &self.sets[s].lines.len(),
                &self.sets[s].lines.capacity(),
                &self.sets[s]
            )?;
        }

        writeln!(
            f,
            "hits:{} misses:{} evictions:{}",
            self.miss, self.hit, self.evict
        )?;
        Ok(())
    }
}

/*impl fmt::Display for Vec<Set> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.set_bits, self.block_bits, self.num_lines)
    }
}*/

/*
pub fn process_address_info(addr: u64, set_bits: u64, block_bits: u64, num_lines: u32) {
    //->line {}
    println!("s:{set_bits}, b:{block_bits}, E:{num_lines}");
    println!(
        "S (set size):{}, B (block size):{}, E:{num_lines}",
        1 << set_bits,
        1 << block_bits
    );
    println!("0x{:x} b{:0>64b}", addr, addr);

    //let blockmask:u64 = u64::pow(2, block_bits.try_into().unwrap())-1;
    let blockmask: u64 = (1 << block_bits) - 1;
    let block: u64 = addr & blockmask;

    //let setmask:u64 = u64::pow(2, (set_bits + block_bits).try_into().unwrap())-1;
    let setmask: u64 = (1 << set_bits + block_bits) - 1;
    let set: u64 = (addr & setmask) >> block_bits;

    println!(
        "blockmask:{:b}, block:{:b}\nsetmask:{:b}, set:{:b}\n",
        blockmask, block, setmask, set
    );
}
*/
