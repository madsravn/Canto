use hound;
use std::collections::HashMap;

pub struct Sound {
    sound: Vec<i16>,
    index: HashMap<i16, Vec<usize>>,
    sample_rate: u32,
}


pub fn open_sound(filename: &str) -> Sound {
    let reader = hound::WavReader::open(filename).expect("Should be able to open file");
    let sample_rate = reader.spec().sample_rate;
    let samples: Vec<i16> = reader.into_samples().filter(|s| s.is_ok()).map(|s| s.unwrap()).collect();
    let mut index = HashMap::new();
    for (i, s) in samples.iter().enumerate() {
        index.entry(*s).or_insert(Vec::new()).push(i);
    }



    Sound { sound: samples, index, sample_rate }

}



