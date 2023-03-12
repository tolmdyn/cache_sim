use std::collections::VecDeque;
use std::fmt;

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

#[derive(PartialEq, Debug)]
#[allow(dead_code)]
pub enum CacheInstruction {
    Load,
    Store,
    Modify,
}

#[derive(Debug)]
#[allow(dead_code)]
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
            lines: VecDeque::with_capacity(num_lines as usize),
        }
    }
}

#[derive(Debug)]
pub struct Cache {
    sets: Box<[Set]>, //sets: Vec<Set>,

    set_bits: u64,   //(s)
    block_bits: u64, //(b)
    num_lines: u32,  //(E)

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
            new_sets.push(Set::new(num_lines));
        }
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

    /* Perform atomic cache operation */
    pub fn operate(&mut self, addr: u64) -> Result<Vec<CacheResult>, String> {
        let address = self.process_address(addr);
        let mut result = Vec::new();

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
        How to turn this into a string....

        let s: Vec<String> = e.iter().map(|x| x.to_string()).collect();
        s.join(" ");
        */

        Ok(result)
    }

    /* Execute an instruction on the cache */
    pub fn instruction(&mut self, inst: &CacheInstruction, addr: &u64) -> Vec<CacheResult> {
        //Re-write to not eat possible result err

        if *inst == CacheInstruction::Modify {
            let mut x = self.operate(*addr).unwrap();
            x.extend(self.operate(*addr).unwrap());
            x
        } else {
            self.operate(*addr).unwrap()
        }
    }

    /* Process a raw address integer into an Address enum */
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

    /* Check if the tag is in cache */
    fn check_hit(&mut self, addr: &Address) -> Result<bool, String> {
        if let Some(set) = self.sets.get(addr.set as usize) {
            if set.lines.contains(&addr.tag) {
                self.hit += 1;
                return Ok(true);
            } else {
                self.miss += 1;
                return Ok(false);
            }
        }
        Err("Problem checking for hit".to_string())
    }
    /* Re-insert tag to update the LRU */
    fn update(&mut self, addr: &Address) -> Result<(), String> {
        if let Some(set) = self.sets.get_mut(addr.set as usize) {
            let index = set.lines.iter().position(|&x| x == addr.tag);
            set.lines.remove(index.unwrap());
            set.lines.push_back(addr.tag);
            return Ok(());
        }
        Err("Cache does not contain address.".to_string())
    }

    /* Check for a free space in the set */
    fn check_free(&self, addr: &Address) -> Result<bool, String> {
        if let Some(set) = self.sets.get(addr.set as usize) {
            return Ok(&set.lines.len() < &set.lines.capacity());
        }
        Err("Problem checking for a free space".to_string())
    }

    /* Insert the tag into set */
    fn insert(&mut self, addr: &Address) -> Result<(), String> {
        if let Some(set) = self.sets.get_mut(addr.set as usize) {
            set.lines.push_back(addr.tag);
            return Ok(());
        }
        Err("Cannot insert into cache".to_string())
    }

    /* Evict the LRU tag from a test */
    fn evict(&mut self, addr: &Address) -> Result<(), String> {
        if let Some(set) = self.sets.get_mut(addr.set as usize) {
            if &set.lines.len() == &set.lines.capacity() {
                set.lines.pop_front();
                set.lines.push_back(addr.tag);
                self.evict += 1;
                return Ok(());
            }
        }
        Err("Cannot evict from cache".to_string())
    }

    pub fn cache_results(&self) -> String {
        format!(
            "hits:{} misses:{} evictions:{}",
            self.hit, self.miss, self.evict
        )
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
            self.hit, self.miss, self.evict
        )?;
        Ok(())
    }
}

/*impl fmt::Display for Vec<Set> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.set_bits, self.block_bits, self.num_lines)
    }
}*/

#[allow(dead_code)]
fn process_address_static(addr: u64, set_bits: u64, block_bits: u64) -> Address {
    //println!("0x{:x} b{:0>64b}", addr, addr);

    let blockmask: u64 = (1 << block_bits) - 1;
    let block: u64 = addr & blockmask;

    let setmask: u64 = (1 << set_bits + block_bits) - 1;
    let set: u64 = (addr & setmask) >> block_bits;

    let tag: u64 = addr >> (block_bits + set_bits);

    Address {
        addr,
        tag,
        block,
        set,
    }
}

#[cfg(test)]
mod tests {
    use crate::cache::{process_address_static, Cache, CacheInstruction};

    #[test]
    fn test_address_process() {
        let addrs = [0x10, 0x20, 0x20, 0x22, 0x18, 0x110, 0x210, 0x12, 0x12];

        for addr in addrs {
            let addr = process_address_static(addr, 4, 4);
            //let result = direct_cache.operate(addr).unwrap();
            println!(
                "addr: {:>10b} tag: {:b} set: {:>4b} block: {:>4b}",
                addr.addr, addr.tag, addr.set, addr.block
            );
        }
    }

    #[test]
    fn test_direct_cache() {
        let addrs = [
            0b00001111,
            0b01011111,
            0b10101111,
            0b11111111,
            0b100001111,
            0b11111111,
            0b101100001111,
        ];

        let mut direct_cache = Cache::new(2, 4, 2);

        for addr in addrs {
            let result = direct_cache.operate(addr).unwrap();
            println!("{} {:?}", addr, result);
        }

        println!("{}", direct_cache);
    }

    #[test]
    fn test_yi_example_addr() {
        let addrs = [0x10, 0x20, 0x20, 0x22, 0x18, 0x110, 0x210, 0x12, 0x12];

        let mut cache = Cache::new(4, 4, 2);

        for addr in addrs {
            let result = cache.operate(addr).unwrap();
            println!("{:>10b} {:?}", addr, result);
        }
    }

    #[test]
    fn test_yi_example_inst() {
        let addrs = [
            (CacheInstruction::Load, 0x10),
            (CacheInstruction::Modify, 0x20),
            (CacheInstruction::Load, 0x22),
            (CacheInstruction::Store, 0x18),
            (CacheInstruction::Load, 0x110),
            (CacheInstruction::Load, 0x210),
            (CacheInstruction::Modify, 0x12),
        ];

        let mut cache = Cache::new(4, 4, 2);

        for addr in addrs {
            let result = cache.instruction(&addr.0, &addr.1);
            println!("{:?} {:x} {:?}", &addr.0, &addr.1, result);
        }

        //println!("{}", cache);

        println!("{}", cache.cache_results());

        assert_eq!(cache.hit, 4);
        assert_eq!(cache.miss, 5);
        assert_eq!(cache.evict, 2);
    }
}
