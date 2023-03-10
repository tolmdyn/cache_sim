use std::collections::VecDeque;
use std::fmt;

/*struct Line {
    //addr: u64,
    block: u64,
    valid: bool,
    tag: u64,
    //last_access
}*/

/*enum Result {
    Hit,
    Miss,
    Evict
}*/

#[derive(Debug)]
struct Address {
    addr: u64,
    _tag: u64,
    set: u64,
    _block: u64,
}

#[derive(Debug)]
struct Set {
    lines: VecDeque<u64>, //just the tag or full Address?
                          //lines: HashMap<u64, &Line>,
                          //queue: VecDeque<Line>
                          //other things
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
    sets: Vec<Set>,

    set_bits: u64,   //(s)
    block_bits: u64, //(b)
    num_lines: u32,  //(E)

    miss: u32,
    hit: u32,
    full: u32,
}

impl Cache {
    pub fn new(set_bits: u64, block_bits: u64, num_lines: u32) -> Cache {
        let set_num = 1 << set_bits;

        let mut new_sets = Vec::with_capacity(set_num);

        for _ in 0..set_num {
            new_sets.push(Set::new(num_lines));
        }

        //println!("{} {}", set_num, new_sets.len());

        Self {
            sets: new_sets,
            set_bits,
            block_bits,
            num_lines,
            miss: 0,
            hit: 0,
            full: 0,
        }
    }

    pub fn operate(&mut self, addr: u64) -> bool {
        let address = self.process_address(addr);
        //println!("{:?}", address);

        if self.check_hit(&address) {
            //check if address is in cache and if it is move to back of queue and update "hit"
            return true;
        } else if self.check_miss(&address) {
            //if there is space in the cache then add it to cache, put in back of queue and update "miss"
            return true;
        } else if self.check_evict(&address) {
            //if cache is full then evict, then add it to cache and update "full"
            return true;
        } else {
            return false; //something bad has happened
        }
    }

    fn process_address(&self, addr: u64) -> Address {
        //println!("0x{:x} b{:0>64b}", addr, addr);

        //let blockmask:u64 = u64::pow(2, block_bits.try_into().unwrap())-1;
        let blockmask: u64 = (1 << self.block_bits) - 1;
        let block: u64 = addr & blockmask;

        //let setmask:u64 = u64::pow(2, (set_bits + block_bits).try_into().unwrap())-1;
        let setmask: u64 = (1 << self.set_bits + self.block_bits) - 1;
        let set: u64 = (addr & setmask) >> self.block_bits;

        let tag: u64 = addr >> (self.block_bits + self.set_bits);
        //println!("blockmask:{:b}, block:{:b}\nsetmask:{:b}, set:{:b}, tag{:b}\n", blockmask, block, setmask, set, tag);

        Address {
            addr,
            _tag : tag,
            _block : block,
            set,
        }
    }

    fn check_hit(&mut self, addr: &Address) -> bool {
        if let Some(set) = self.sets.get_mut(addr.set as usize) {
            if set.lines.contains(&addr.addr) {
                self.hit += 1;
                //update access
                let i = set.lines.iter().position(|&x| x == addr.addr);
                set.lines.remove(i.unwrap());
                set.lines.push_back(addr.addr);
                return true;
            }
        }
        false
    }

    fn check_miss(&mut self, addr: &Address) -> bool {
        if let Some(set) = self.sets.get_mut(addr.set as usize) {
            if &set.lines.len() < &set.lines.capacity() {
                set.lines.push_back(addr.addr);
                self.miss += 1;
                return true;
            }
        }
        false
    }

    fn check_evict(&mut self, addr: &Address) -> bool {
        if let Some(set) = self.sets.get_mut(addr.set as usize) {
            if &set.lines.len() == &set.lines.capacity() {
                set.lines.pop_front();
                set.lines.push_back(addr.addr);
                self.full += 1;

                return true;
            }
        }
        false
    }
}

impl fmt::Display for Cache {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Cache : (s:{}, b:{}, e:{})", self.set_bits, self.block_bits, self.num_lines)?;

        for s in 0..self.sets.len() {
            writeln!(f, "{} ({}/{}): {:?}", s, &self.sets[s].lines.len(), &self.sets[s].lines.capacity(), &self.sets[s])?;
        }

        writeln!(f, "Miss:{}, Hit:{}, Evictions:{}", self.miss, self.hit, self.full)?;
        Ok(())
    }
}

/*impl fmt::Display for Vec<Set> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.set_bits, self.block_bits, self.num_lines)
    }
}*/


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
