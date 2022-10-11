use canto::overlap;
use std::env;
use std::fs;

pub fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        let input: Vec<String> = fs::read_to_string(&args[1])
            .expect("Should be able to open file")
            .split("\n")
            .filter(|s| s.len() > 0)
            .map(|s| s.to_string())
            .collect();
        let mut input_one = input.clone();
        input_one.truncate(input_one.len() - 1);
        for (i, sound_one) in input_one.iter().enumerate() {
            for sound_two in input.iter().skip(i + 1) {
                let sound_one_o = overlap::open_sound(sound_one);
                let sound_two_o = overlap::open_sound(sound_two);
                let result = overlap::find_overlap(&sound_one_o, &sound_two_o);
                if result.len() > 0 {
                    println!("Equalities between {} and {}", sound_one, sound_two);
                    println!("{:?}", result);
                }
            }
        }

        // TODO: Read all files in
        // Compare to each other and find similarities
        // Print similarities and also create files holding the similarities so we can verify.
        //
        // 8 KHz and 16 KHz. Can we compare these at all?
    }
}
