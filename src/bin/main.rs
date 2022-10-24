use canto::overlap::{open_sound, find_overlap, Sound};
use std::env;
use std::fs;
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{LineJoin, LineStyle};
use plotlib::view::ContinuousView;

// TODO: Output graphwiz data of each file with name, samples, length and samle rate.
//
// Also output an image everywhere we see a hit. With file one, file two and the diff of them.
// Mark where the diff is at.
//
fn plot(samples: &Vec<i16>, filename: &str) {
    let zips: Vec<f64> = (0..samples.len() * 5).map(|x| x as f64).collect();
    let input = zips.iter().map(|&x| x as f64).zip(samples.iter().flat_map(|&x| [x as f64, x as f64, x as f64, x as f64, x as f64]).collect::<Vec<f64>>()).collect();
    println!("{:?}", input);
    let l1 = Plot::new(input).line_style(
        LineStyle::new()
            .colour("burlywood")
            .linejoin(LineJoin::Round),
    );
    let v = ContinuousView::new().add(l1);
    Page::single(&v).dimensions(5000, 400).save(format!("{}.svg", filename)).expect("saving svg");


}

pub fn main() {

    let args: Vec<_> = env::args().collect();
    if args.len() == 2 {
        let input: Vec<Sound> = fs::read_to_string(&args[1])
            .expect("Should be able to open file")
            .split("\n")
            .filter(|s| s.len() > 0)
            .map(|s| s.to_string())
            .map(|s| open_sound(&s))
            .collect();

        let input_eight: Vec<Sound> = input.iter().filter(|x| x.sample_rate == 8000).map(|x| x.clone()).collect();
        let min = input_eight.iter().fold(i16::MAX, |a, b| a.min(b.min));
        let max = input_eight.iter().fold(i16::MIN, |a, b| a.max(b.max));
        println!("Input eight: min: {}, max: {}", min, max);
        let input_sixteen: Vec<Sound> = input.iter().filter(|x| x.sample_rate == 16000).map(|x| x.clone()).collect();
        let min = input_sixteen.iter().fold(i16::MAX, |a, b| a.min(b.min));
        let max = input_sixteen.iter().fold(i16::MIN, |a, b| a.max(b.max));

        println!("Input sixteen: min: {}, max: {}", min, max);

        //println!("{:?}", input_eight[0]);
        //plot(&input_eight[0].samples, &input_eight[0].name);

        let mut input_one = input_eight.clone();
        input_one.truncate(input_one.len() - 1);
        for (i, sound_one) in input_one.iter().enumerate() {
            for sound_two in input_eight.iter().skip(i + 1) {
                let result = find_overlap(&sound_one, &sound_two);
                if result.len() > 0 {
                    println!("Equalities between {} and {}", sound_one.name, sound_two.name);
                    println!("{:?}", result);
                }
            }
        }

/*
        let mut input_one = input.clone();
        input_one.truncate(input_one.len() - 1);
        for (i, sound_one) in input_one.iter().enumerate() {
            for sound_two in input.iter().skip(i + 1) {
                let result = overlap::find_overlap(&sound_one, &sound_two);
                if result.len() > 0 {
                    println!("Equalities between {} and {}", sound_one.name, sound_two.name);
                    println!("{:?}", result);
                }
            }
        }
        */

        // TODO: Read all files in
        // Compare to each other and find similarities
        // Print similarities and also create files holding the similarities so we can verify.
        //
        // 8 KHz and 16 KHz. Can we compare these at all?
    } 
    if args.len() == 4  {
        let filename = &args[1];
        let start = args[2].parse::<usize>().unwrap();
        let end = start + args[3].parse::<usize>().unwrap();
        let sound = open_sound(filename);
        println!("{} has length of {} samples with sample rate {}", sound.name, sound.samples.len(), sound.sample_rate);
        let data = &sound.samples[start..end];
        println!("{:?}", data);



    }
}
