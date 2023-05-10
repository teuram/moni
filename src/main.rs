
use std::{
    fs::File,
    io::{BufReader, BufRead}
};

fn moni(file: File) {
    let reader = BufReader::new(file);

    let mut sum: f32 = 0.0;
    let mut num: f32;

    for line in reader.lines() {

        let line = line.unwrap().trim().to_string();

        if line.eq("-- total") {
            println!("{:9.2}\n", sum);
            continue;
        }

        if let Ok(a) = line.parse::<f32>() {
            num = a;
        } else {
            continue;
        }

        println!("{:9.2} {} {:.2}\x1b[0m" , sum,
                 {
                     if num < 0.0 {
                         "\x1b[38;2;255;0;0m->"
                     } else if num > 0.0 {
                         "\x1b[38;2;0;255;0m<+"
                     } else {
                         "--"
                     }
                 }, num.abs());

        sum += num;
    }
}

fn main() {
    let env = std::env::var("MONI");

    match env {
        Ok(env) => {
            let file = File::open(env)
                .expect("File moni can not reading");
            moni(file);
        },
        Err(e) => {
            println!("{}", e.to_string());
        }
    }

}

