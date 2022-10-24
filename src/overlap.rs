use hound;
use std::collections::HashMap;
use std::path::Path;

#[derive(Clone,Debug)]
pub struct Sound {
    pub samples: Vec<i16>,
    index: HashMap<i16, Vec<usize>>,
    pub sample_rate: u32,
    pub name: String,
    pub max: i16,
    pub min: i16,
}

#[derive(Debug)]
pub struct Position {
    pub start_one: usize,
    pub start_two: usize,
    pub length: usize,
}

const THRESHOLD: i16 = 2;

pub fn open_sound(filename: &str) -> Sound {
    let reader = hound::WavReader::open(filename).expect(&format!("Should be able to open file: {}", filename));
    let sample_rate = reader.spec().sample_rate;
    let samples: Vec<i16> = reader
        .into_samples()
        .filter(|s| s.is_ok())
        .map(|s| s.unwrap())
        .collect();
    let mut index = HashMap::new();
    for (i, s) in samples.iter().enumerate() {
        index.entry(*s).or_insert(Vec::new()).push(i);
    }
    let short_filename = Path::new(filename).file_stem().expect("Should have a filename").to_str().expect("Should be able to convert to str").to_string();
    let min = samples.iter().fold(i16::MAX, |a, &b| a.min(b));
    let max = samples.iter().fold(i16::MIN, |a, &b| a.max(b));

    Sound {
        samples,
        index,
        sample_rate,
        name: short_filename,
        min,
        max,
    }
}

/// Iterate over needle_sound and see how many places in master_sound it is present.
pub fn find_overlap(master_sound: &Sound, needle_sound: &Sound) -> Vec<Position> {
    let mut vec = Vec::new();

    for (i, sample) in needle_sound.samples.iter().enumerate() {
        if let Some(positions) = master_sound.index.get(sample) {
            for position in positions.iter() {
                // Look here if we need to continue
                if check_if_continue(i, *position, &vec) {
                    continue
                }
                let length = find_similar_length(master_sound, needle_sound, *position, i);
                let pos = Position {
                    start_one: *position,
                    start_two: i,
                    length,
                };

                if pos.length > 63 {
                    //println!("Adding {:?}", pos);
                    vec.push(pos);
                }
            }
        }
    }
    vec
}


fn check_if_continue(start_one: usize, start_two: usize, positions: &Vec<Position>) -> bool {
    let res: Vec<&Position> = positions.iter().filter( |x| x.start_one < start_one
                             && x.start_two < start_two
                             && start_one < x.start_one + x.length
                             && start_two < x.start_two + x.length).filter(|x|
                             start_two.saturating_sub(start_one) == x.start_two.saturating_sub(x.start_one)).collect();

    !res.is_empty()
}

// TODO: Which trait lets us subtract values? 
fn within_threshold(one: &Option<&i16>, two: &Option<&i16>) -> bool {
    let res = match (one, two) {
        (Some(&a), Some(&b)) => (a-b).abs() < THRESHOLD,
        _ => false
    };
    res

}

fn find_similar_length(
    sound_one: &Sound,
    sound_two: &Sound,
    sound_one_start_pos: usize,
    sound_two_start_pos: usize,
) -> usize {
    let mut length: usize = 0;
    //while &sound_one.samples.get(sound_one_start_pos + length) == &sound_two.samples.get(sound_two_start_pos + length) {
    while within_threshold(&sound_one.samples.get(sound_one_start_pos + length), &sound_two.samples.get(sound_two_start_pos + length)) {
        if &sound_one.samples.get(sound_one_start_pos + length) == &None {
            break;
        }
        length = length + 1;
    }
    
    let count_non_zeroes = &sound_one.samples[sound_one_start_pos..sound_one_start_pos+length].iter().filter(|&x| x.clone().abs() > THRESHOLD - 1).count();
    let count_non_zeroes_two = &sound_two.samples[sound_two_start_pos..sound_two_start_pos+length].iter().filter(|&x| x.clone().abs() > THRESHOLD - 1).count();
    if count_non_zeroes.clone() > 63 || count_non_zeroes_two.clone() > 63 {
        //println!("FIRST: {:?}", &sound_one.samples[sound_one_start_pos..sound_one_start_pos+length]);
        //println!("SECOND: {:?}", &sound_two.samples[sound_two_start_pos..sound_two_start_pos+length]);
        return length;
    } else {
        0
    }
}
