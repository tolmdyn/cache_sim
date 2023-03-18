mod tests {
    use sim::{cache::{Cache, CacheInstruction}, process_input_file, Cmd, process_args};

    #[test]
    fn dummy() {
        println!("TEST ");
    }
    /* ARG TESTS */
    #[test]
    fn test_args_success() {
        //"-s <num> -E <num> -b <num> -t <file>"
        let args = ["test", "-s", "1", "-E", "2", "-b", "3", "-t", "filename"];
        let flags = process_args(&args.map(|s| s.to_string())).unwrap();

        println!("{:?}", flags);
        assert_eq!(flags.v, false);
        assert_eq!(flags.s, 1);
        assert_eq!(flags.e, 2);
        assert_eq!(flags.b, 3);
        assert_eq!(flags.t, "filename");
    }
    /*
    Because we call process::exit, running these tests will kill the test program, this 
    could be solved by returning a custom error message to main instead, but the max code 
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
        let cmd_string = "L  ffff,2";
        let cmd = sim::line_to_command(cmd_string).unwrap();

        assert_eq!(CacheInstruction::Load, cmd.inst);
        assert_eq!(0xffff, cmd.address);

        let cmd_string = " I deadbeef,2";
        let cmd = sim::line_to_command(cmd_string).unwrap();

        assert_eq!(CacheInstruction::Instruction, cmd.inst);
        assert_eq!(0xdeadbeef, cmd.address);

        let cmd_string = "M 10,2";
        let cmd = sim::line_to_command(cmd_string).unwrap();

        assert_eq!(CacheInstruction::Modify, cmd.inst);
        assert_eq!(0x10, cmd.address);
    }

    #[test]
    fn line_to_command_none() {
        let cmd_string = "J  ffff,2";
        let cmd = sim::line_to_command(cmd_string);
        assert!(cmd.is_none());

        let cmd_string = "--- ----";
        let cmd = sim::line_to_command(cmd_string);
        assert!(cmd.is_none());

        let cmd_string = "";
        let cmd = sim::line_to_command(cmd_string);
        assert!(cmd.is_none());
    }

    #[test]
    fn process_input_file_test() {
        const FILENAME: &str = "../traces/yi.trace";

        let mut cache = Cache::new(4, 4, 2);

        process_input_file(FILENAME, &mut cache, true).unwrap();

        println!("{}", cache.cache_results());
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

        //println!("{:?}", direct_cache);
        assert_eq!(direct_cache.sets.len(), 4);
        assert_eq!(direct_cache.sets[0].len(), 2);
        assert_eq!(direct_cache.cache_results(), "hits:1 misses:6 evictions:1");
    }

    #[test]
    fn test_operation_insert() {
        //add address to different sets and assert set bits == correct set
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
    fn run_cache_long() {
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