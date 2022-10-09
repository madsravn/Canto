use std::env;
use std::fs;

pub fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        let input: Vec<String> = fs::read_to_string(&args[1]).expect("Should be able to open file").split("\n").filter(|s| s.len() > 0).map(|s| s.to_string()).collect();



    // TODO: Read all files in
    // Compare to each other and find similarities
    // Print similarities and also create files holding the similarities so we can verify.
    //
    // 8 KHz and 16 KHz. Can we compare these at all? 

    }


}
