use std::collections::VecDeque;
use std::fmt;
use crate::Cmd;

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
    pub sets: Box<[VecDeque<u64>]>, //sets: Vec<Set>,
    set_bits: u64,   //(s)
    block_bits: u64, //(b)
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
            new_sets.push(VecDeque::with_capacity(num_lines as usize));
        }

        Self {
            sets: new_sets.into_boxed_slice(), //cast it into a boxed slice as we don't need to resize it anymore,
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
        } else if command.inst == CacheInstruction::Modify {
            let mut x = self.operate(command.address);
            x.extend(self.operate(command.address));
            x
        } else {
            vec!()
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
            set.push_back(addr.tag);
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
            set.push_back(addr.tag);
            return;
        }
        panic!("Cannot insert into cache");
    }

    /* Evict the LRU tag from a test */
    fn evict(&mut self, addr: &Address) {
        if let Some(set) = self.sets.get_mut(addr.set as usize) {
            if &set.len() == &set.capacity() {
                set.pop_front();
                set.push_back(addr.tag);
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