
use std::{
    fs::File,
    io::{BufReader, BufRead},
    cmp::Ordering
};

fn main() {

    let file = File::open("moni")
        .expect("File moni can not reading");
    let reader = BufReader::new(file);

    let mut sum: i32 = 0;
    let mut num: i32;

    for line in reader.lines() {

        if let Ok(a) = line.unwrap().trim().parse::<i32>() {
            if a == 0 {
                println!("{:6}", sum);
                continue;
            }
            num = a;
        } else {
            continue;
        }

        println!("{:6} {} {}" , sum,
             match num.cmp(&0) {
                Ordering::Less => "->",
                Ordering::Greater => "<+",
                _ => ""
             }, num.abs());

        sum += num;
    }

}

