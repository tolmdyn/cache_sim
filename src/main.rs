mod cache;

pub fn main() {
    //sim::process_address(0x7ff0005c8, 4, 8, 1);
    //sim::process_address(26, 4, 4, 1);

    /*
    let mut cache = sim::Cache::new(4, 4, 4);

    println!("{:?}", cache);

    cache.operate_cache(26);
    cache.operate_cache(526);
    cache.operate_cache(26);
    //cache.get_stats();

    println!("{:?}", cache);
    */

    let mut direct_cache = cache::Cache::new(2, 4, 2);

    direct_cache.operate(0b00001111); //miss
    direct_cache.operate(0b01011111); //miss
    direct_cache.operate(0b10101111); //miss
    direct_cache.operate(0b11111111); //miss
    direct_cache.operate(0b100001111); //miss

    direct_cache.operate(0b11111111); //hit

    direct_cache.operate(0b101100001111); //full

    //println!("{:?}", direct_cache);

    println!("{:?}", direct_cache.operate(0b00001111).unwrap());
    println!("{}", direct_cache);
}









