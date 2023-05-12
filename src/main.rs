
use std::{
    fs::File,
    io::{BufReader, BufRead}
};

fn color_f32(num: f32) -> String {
    format!("{}{:.2}\x1b[1;0m",
            {
                if num < 0.0 {
                    "\x1b[1;31m"
                } else if num > 0.0 {
                    "\x1b[1;32m"
                } else {
                    "--"
                }
            }, num)
}

fn moni(file: File) {
    let reader = BufReader::new(file);

    let mut sum: f32 = 0.0;
    let mut num: f32;
    let mut values: Vec<f32> = Vec::<f32>::new();

    for line in reader.lines() {

        let line = line.unwrap().trim().to_string();

        if line.eq("-- total") {
            println!("\n{}\ntotal: {}\n", line, color_f32(sum));
            continue;
        }
        if line.eq("-- push") {
            values.push(sum);
            sum = 0.0;
            println!("{}\n", line);
            continue;
        }
        if line.eq("-- pop") {
            if let Some(val) = values.pop() {
                sum += val;
                println!("{}", line);
                if sum - val < val {
                    println!("\x1b[1;33mwarning:\x1b[0m {:.2} is less than zero\n", sum - val);
                }
                // if sum + val < val {
                //     sum += val;
                //     println!("\x1b[1;33mwarning:\x1b[0m {:.2} is less than zero\n", sum - val);
                // } else {
                //     sum += val;
                //     println!("{}\n", line);
                // }
            }
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
                         "\x1b[1;31m->"
                     } else if num > 0.0 {
                         "\x1b[1;32m<+"
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

