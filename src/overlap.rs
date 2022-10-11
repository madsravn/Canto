use hound;
use std::collections::HashMap;

pub struct Sound {
    samples: Vec<i16>,
    index: HashMap<i16, Vec<usize>>,
    sample_rate: u32,
}

#[derive(Debug)]
pub struct Position {
    pub start_one: usize,
    pub start_two: usize,
    pub length: usize,
}

pub fn open_sound(filename: &str) -> Sound {
    let reader = hound::WavReader::open(filename).expect("Should be able to open file");
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

    Sound {
        samples,
        index,
        sample_rate,
    }
}

/// Iterate over needle_sound and see how many places in master_sound it is present.
pub fn find_overlap(master_sound: &Sound, needle_sound: &Sound) -> Vec<Position> {
    let mut vec = Vec::new();

    for (i, sample) in needle_sound.samples.iter().enumerate() {
        if let Some(positions) = master_sound.index.get(sample) {
            for position in positions.iter() {
                // Look here if we need to continue
                if check_if_continue(*position, i, &vec) {
                    continue
                }
                let length = find_similar_length(master_sound, needle_sound, *position, i);
                let pos = Position {
                    start_one: i,
                    start_two: *position,
                    length,
                };

                if pos.length > 20 {
                    println!("Adding {:?}", pos);
                    vec.push(pos);
                }
            }
        }
    }
    vec
}


fn check_if_continue(start_one: usize, start_two: usize, positions: &Vec<Position>) -> bool {



    true
}

fn find_similar_length(
    sound_one: &Sound,
    sound_two: &Sound,
    sound_one_start_pos: usize,
    sound_two_start_pos: usize,
) -> usize {
    let mut length: usize = 0;
    while &sound_one.samples.get(sound_one_start_pos + length)
        == &sound_two.samples.get(sound_two_start_pos + length)
    {
        if &sound_one.samples.get(sound_one_start_pos + length) == &None {
            break;
        }
        length = length + 1;
    }
    length
}
