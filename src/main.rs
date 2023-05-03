
use std::{
    fs::File,
    io::{BufReader, BufRead},
    cmp::Ordering
};

fn main() {

    let file = File::open("moni")
        .expect("File moni can not reading");
    let reader = BufReader::new(file);

    let mut sum: f32 = 0.0;
    let mut num: f32;

    for line in reader.lines() {

        let line = line.unwrap().trim().to_string();

        if line.eq("-- total") {
            println!("{:6}\n", sum);
            continue;
        }

        if let Ok(a) = line.parse::<f32>() {
            num = a;
        } else {
            continue;
        }

        println!("{:6} {} {}\x1b[0m" , sum,
                 {
                     if num > 0.0 {
                         "\x1b[38;2;255;0;0m->"
                     } else if num < 0.0 {
                         "\x1b[38;2;0;255;0m<+"
                     } else {
                         "--"
                     }
                 }, num.abs());
                 // match num.cmp(&0) {
                 //     Ordering::Less => "\x1b[38;2;255;0;0m->",
                 //     Ordering::Greater => "\x1b[38;2;0;255;0m<+",
                 //     _ => ""
                 // }, num.abs());

        sum += num;
    }

}

