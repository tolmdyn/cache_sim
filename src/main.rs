pub fn main() {
    process_address(0x7ff0005c8, 4, 8, 1);
    process_address(26, 4, 4, 1);

}


struct Line {
    addr: u64,
    block: u64,
    valid: bool,
    tag: u64,
    //last_access
}




fn process_address(addr:u64, set_bits:u64, block_bits:u64, num_lines:u32) { //->line {}
    println!("S:{set_bits}, b:{block_bits}, E:{num_lines}");
    println!("0x{:x} b{:0>64b}", addr, addr);

    //let blockmask:u64 = u64::pow(2, block_bits.try_into().unwrap())-1;

    let blockmask:u64 = (1 << block_bits) -1;
    let block:u64 = addr & blockmask;

    //let setmask:u64 = u64::pow(2, (set_bits + block_bits).try_into().unwrap())-1;
    let setmask:u64 = (1 << set_bits + block_bits) -1;
    let set:u64 = (addr & setmask) >> block_bits;

    println!("blockmask:{:b}, block:{:b}\nsetmask:{:b}, set:{:b}", blockmask, block, setmask, set);
}

