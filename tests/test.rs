mod tests {
    use sim::{Cache, CacheInstruction, process_input_file, Cmd, process_args};

    /* ARG TESTS */
    #[test]
    fn test_args_success() {
        //"-s <num> -E <num> -b <num> -t <file>"
        let args = ["test", "-s", "1", "-E", "2", "-b", "3", "-t", "filename"];
        let flags = process_args(&args.map(|s| s.to_string())).unwrap();

        //println!("{:?}", flags);
        assert_eq!(flags.v, false);
        assert_eq!(flags.s, 1);
        assert_eq!(flags.e, 2);
        assert_eq!(flags.b, 3);
        assert_eq!(flags.t, "filename");
    }
    /*
    Because of call process::exit, running these tests will kill the test program, this 
    could be fixed by returning a custom error message to main instead, but the max code 
    line limit for the assignment is already reached. (300)
    #[test]
    fn test_bad_args_missing_s(){
        let args = ["test", "-E", "2", "-b", "3", "-t", "filename"];
        let flags = process_args(&args.map(|s| s.to_string())).unwrap();
    
    fn test_bad_args_missing_b(){
        let args = ["test", "-s", "2", "-E", "3", "-t", "filename"];
        let flags = process_args(&args.map(|s| s.to_string())).unwrap();

    fn test_bad_args_missing_e(){
        let args = ["test", "-s", 5, "-b", "3", "-t", "filename"];
        let flags = process_args(&args.map(|s| s.to_string())).unwrap();

    fn test_bad_args_missing_t(){
        let args = ["test", "-E", "2", "-b", "3"];
        let flags = process_args(&args.map(|s| s.to_string())).unwrap();
    }
    #[test]
    fn test_show_usage(){
        let args = ["test", "-h", "-E", "2", "-b", "3"];
        let flags = process_args(&args.map(|s| s.to_string())).unwrap();
    }*/

    /* INPUT FILE TESTS */
    #[test]
    fn line_to_command_test() {
        /* A normal Load instruction with a hexa address */
        let cmd_string = "L  ffff,2";
        let cmd = sim::line_to_command(cmd_string).unwrap();

        assert_eq!(CacheInstruction::Load, cmd.inst);
        assert_eq!(0xffff, cmd.address);

        /* Instructions should work too even though we dont use them */
        let cmd_string = " I deadbeef,2";
        let cmd = sim::line_to_command(cmd_string).unwrap();

        assert_eq!(CacheInstruction::Instruction, cmd.inst);
        assert_eq!(0xdeadbeef, cmd.address);

        /* Modify works, as and the 0x10 is interpreted as 16 not deicmal 10 */
        let cmd_string = "M 10,2";
        let cmd = sim::line_to_command(cmd_string).unwrap();

        assert_eq!(CacheInstruction::Modify, cmd.inst);
        assert_eq!(16, cmd.address);
    }

    #[test]
    fn line_to_command_none() {
        /* Invalid operations do not crash program but just return nothing */
        let cmd_string = "J  ffff,2";
        let cmd = sim::line_to_command(cmd_string);
        assert!(cmd.is_none());
        /* Sometimes the trace files contain comments or non-parseable text */
        let cmd_string = "--- ----";
        let cmd = sim::line_to_command(cmd_string);
        assert!(cmd.is_none());
        /* We should anticipate empty lines */
        let cmd_string = "";
        let cmd = sim::line_to_command(cmd_string);
        assert!(cmd.is_none());
    }


    #[test]
    #[should_panic(expected = "InvalidDigit")]
    fn line_to_command_panic() {
        /* A non hexadecimal address will cause an exception */
        let cmd_string = "S garbage,5";
        let cmd = sim::line_to_command(cmd_string);
        println!("{:?}", cmd);

    }
    
    /* CACHE TESTS */
    #[test]
    fn test_direct_cache() {
        let addrs = [
            0b00001111, //miss (set 0)
            0b01011111, //miss (set 1)
            0b10101111, //miss (set 2)
            0b11111111, //miss (set 3)
            0b100001111,    //miss (set 0)
            0b11111111,     //hit (set 3)
            0b101100001111, //miss, eviction (set 0)
        ];

        let mut direct_cache = Cache::new(2, 4, 2);

        addrs.map(|address| direct_cache.operate(address));

        assert_eq!(direct_cache.sets.len(), 4);
        assert_eq!(direct_cache.sets[0].len(), 2);
        assert_eq!(direct_cache.cache_results(), "hits:1 misses:6 evictions:1");
    }

    #[test]
    fn test_tiny_cache() {
        let addrs = [ 1,2,3,4,5,6,7,8,9,10 ];

        let mut cache = Cache::new(0, 0, 1);

        addrs.map(|address| cache.operate(address));

        assert_eq!(cache.sets.len(), 1);
        assert!(cache.sets[0].contains(&10));
        assert_eq!(cache.cache_results(), "hits:0 misses:10 evictions:9");
    }

    #[test]
    fn test_tiny_cache_2() {
        let addrs = [ 1,1,0,0,1,1,0,0,1,1,0,0,1,1,0,0,1,1,0,0 ];

        let mut cache = Cache::new(0, 0, 1);

        addrs.map(|address| cache.operate(address));

        //println!("{:?}", cache);
        assert_eq!(cache.cache_results(), "hits:10 misses:10 evictions:9");
    }

    #[test]
    fn test_operation_insert() {
        //add address to different sets and assert set bits == correct set
        let mut cache = Cache::new(2, 4, 4);

        cache.run_command(Cmd {inst:CacheInstruction::Load, address:0b1000000});
        cache.run_command(Cmd {inst:CacheInstruction::Load, address:0b10010000});
        cache.run_command(Cmd {inst:CacheInstruction::Load, address:0b11100000});
        cache.run_command(Cmd {inst:CacheInstruction::Load, address:0b100110000});

        let mut i = cache.sets.into_iter();

        assert_eq!(1, *i.next().unwrap().get(0).unwrap());
        assert_eq!(2, *i.next().unwrap().get(0).unwrap());
        assert_eq!(3, *i.next().unwrap().get(0).unwrap());
        assert_eq!(4, *i.next().unwrap().get(0).unwrap());
    }

    #[test]
    fn test_lru() {
        let mut cache = Cache::new(0, 0, 4);

        cache.run_command(Cmd {inst:CacheInstruction::Load, address:0b11});
        cache.run_command(Cmd {inst:CacheInstruction::Load, address:0b10});
        cache.run_command(Cmd {inst:CacheInstruction::Load, address:0b01});
        cache.run_command(Cmd {inst:CacheInstruction::Load, address:0b00});

        assert_eq!(vec![3,2,1,0], *cache.sets.get(0).unwrap());

        cache.run_command(Cmd {inst:CacheInstruction::Load, address:0b100});
        cache.run_command(Cmd {inst:CacheInstruction::Load, address:0b101});

        assert_eq!(vec![1,0,4,5], *cache.sets.get(0).unwrap());

        cache.run_command(Cmd {inst:CacheInstruction::Load, address:0b00});
        cache.run_command(Cmd {inst:CacheInstruction::Load, address:0b1000});

        assert_eq!(vec![4,5,0,8], *cache.sets.get(0).unwrap());
    }

    #[test]
    fn test_yi_example_inst() {
        let commands: [Cmd; 7] = [
            Cmd {inst:CacheInstruction::Load, address:0x10},
            Cmd {inst:CacheInstruction::Modify, address:0x20},
            Cmd {inst:CacheInstruction::Load, address:0x22},
            Cmd {inst:CacheInstruction::Store, address:0x18},
            Cmd {inst:CacheInstruction::Load, address:0x110},
            Cmd {inst:CacheInstruction::Load, address:0x210},
            Cmd {inst:CacheInstruction::Modify, address:0x12},
        ];

        /* EXAMPLE 1 */
        /* .sim-ref -s 4 -E 1 -b 4 -t ../traces/yi.trace */
        let mut cache = Cache::new(4, 4, 1);
        commands.map(|cmd| cache.run_command(cmd));
        assert_eq!(cache.cache_results(), "hits:4 misses:5 evictions:3");
        
        /* EXAMPLE 2 */
        /* .sim-ref -s 8 -E 2 -b 4 -t ../traces/yi.trace*/
        let mut cache = Cache::new(8, 4, 2);
        commands.map(|cmd| cache.run_command(cmd));
        assert_eq!(cache.cache_results(), "hits:5 misses:4 evictions:0");
    }

    /* INTEGRATION (FULL) TESTS*/
    #[test]
    fn run_cache_yi() {
        const FILENAME: &str = "../traces/yi.trace";

        let mut cache = Cache::new(4, 4, 2);
        process_input_file(FILENAME, &mut cache, true).unwrap();
        assert_eq!("hits:4 misses:5 evictions:2", cache.cache_results());
    }

    #[test]
    fn run_cache_ibm() {
        const FILENAME: &str = "../traces/ibm.trace";

        let mut cache = Cache::new(4, 4, 2);
        process_input_file(FILENAME, &mut cache, false).unwrap();
        assert_eq!("hits:2 misses:3 evictions:0", cache.cache_results());
    }

    #[test]
    fn run_cache_trans() {
        const FILENAME: &str = "../traces/trans.trace";

        let mut cache = Cache::new(8, 4, 2);
        process_input_file(FILENAME, &mut cache, false).unwrap();
        assert_eq!("hits:226 misses:12 evictions:0", cache.cache_results());

        let mut cache = Cache::new(4, 2, 2);
        process_input_file(FILENAME, &mut cache, false).unwrap();
        assert_eq!("hits:195 misses:43 evictions:11", cache.cache_results());
    }

    /* This one takes a while. */
    #[test]
    fn run_cache_long() { //2.59 w vec / 2.73
        const FILENAME: &str = "../traces/long.trace";

        //-s 4 -E 10 -b 4
        //hits:278655 misses:8309 evictions:8149 (sim-ref)
        let mut cache = Cache::new(4, 4, 10);
        process_input_file(FILENAME, &mut cache, false).unwrap();
        assert_eq!(cache.cache_results(), "hits:278655 misses:8309 evictions:8149");

        //-s 1 -E 1 -b 1
        //hits:54369 misses:232595 evictions:232594 (sim-ref)
        let mut cache = Cache::new(1, 1, 1);
        process_input_file(FILENAME, &mut cache, false).unwrap();
        assert_eq!(cache.cache_results(), "hits:54369 misses:232595 evictions:232594");

        //-s 4 -E 8 -b 8
        //hits:282564 misses:4400 evictions:4272 (sim-ref)
        let mut cache = Cache::new(4, 8, 8);
        process_input_file(FILENAME, &mut cache, false).unwrap();
        assert_eq!(cache.cache_results(), "hits:282564 misses:4400 evictions:4272");
    }
}