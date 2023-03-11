use std::collections::VecDeque;
use std::fmt;

/*struct Line {
    //addr: u64,
    block: u64,
    valid: bool,
    tag: u64,
    //last_access
}*/
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

#[derive(PartialEq)]
#[allow(dead_code)]
pub enum CacheInstruction {
    Load,
    Store,
    Modify,
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

    pub fn operate(&mut self, addr: u64) -> Result<Vec<CacheResult>, String> {
        let address = self.process_address(addr);
        //println!("{:?}", address);
        let mut result = Vec::new();
        /*
        if self.check_hit(&address).unwrap() { //check if address is in cache and if it is move to back of queue and update "hit"
            let res = self.update(&address)?;
            result.push(res);
            //Ok(self.update(&address)?) //return CacheResult::Hit;
        } else {
            if self.check_free(&address)? { //if there is space in the cache then add it to cache, put in back of queue and update "miss"
            //result.push(Cache::Result)
            Ok(self.insert(&address)?) ////return CacheResult::Miss;
            
        } else {
            //if cache is full then evict, then add it to cache and update "full"
            Ok(self.evict(&address)?) //return CacheResult::Eviction;
        }*/

        if self.check_hit(&address)? {
            result.push(CacheResult::Hit);
            self.update(&address)?;
        } else {
            result.push(CacheResult::Miss);
            if self.check_free(&address)? {
                self.insert(&address)?;
            } else {
                result.push(CacheResult::Eviction);
                self.evict(&address)?;
            }
        }

        /*
        let s: Vec<String> = e.iter().map(|x| x.to_string()).collect();
        s.join(" ");
        */

        Ok(result)

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

    fn update(&mut self, addr: &Address) -> Result<(), String> { //if the item is in cache then update the LRU
        if let Some(set) = self.sets.get_mut(addr.set as usize) {
            self.hit += 1;
            let index = set.lines.iter().position(|&x| x == addr.addr);
            set.lines.remove(index.unwrap());
            set.lines.push_back(addr.addr);
            //return Ok(CacheResult::Hit);
            return Ok(())
        }

        Err("Cache does not contain address.".to_string())
    }

    fn check_free(&self, addr: &Address) -> Result<bool, String> {
        if let Some(set) = self.sets.get(addr.set as usize) {
            return Ok(&set.lines.len() < &set.lines.capacity())
        }
        Err("Problem checking for a free space".to_string())
    }

    fn insert(&mut self, addr: &Address) -> Result<(), String> {
        if let Some(set) = self.sets.get_mut(addr.set as usize) {
            set.lines.push_back(addr.addr);
            self.miss += 1;
            return Ok(())
        }
        Err("Cannot insert into cache".to_string())
    }

    fn evict(&mut self, addr: &Address) -> Result<(), String> {
        if let Some(set) = self.sets.get_mut(addr.set as usize) {
            if &set.lines.len() == &set.lines.capacity() {
                set.lines.pop_front();
                set.lines.push_back(addr.addr);
                self.evict += 1;

                return Ok(())
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



