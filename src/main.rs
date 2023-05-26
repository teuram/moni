
use std::{
    fs::File,
    io::{BufReader, BufRead},
    collections::VecDeque
};

enum Color {
    GREEN,
    RED,
    BLUE,
    YELLOW,
    NONE
}

impl Color {
    fn to_escape(&self) -> &str {
        match self {
            Color::RED => "\x1b[1;31m",
            Color::BLUE => "\x1b[1;34m",
            Color::GREEN => "\x1b[1;32m",
            Color::YELLOW => "\x1b[1;32m",
            Color::NONE => "\x1b[0m"
        }
    }
}

struct ColorString {
    text: String,
    color: Color
}

impl ColorString {
    fn new<T>(t: T, color: Color) -> ColorString
    where T: ToString {
        ColorString {
            text: t.to_string(),
            color
        }
    }
    fn get_text(&self) -> &str {
        self.text.as_str()
    }
}

impl std::fmt::Display for ColorString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
            format!("{}{}{}",
                    self.color.to_escape(),
                    self.get_text(), Color::NONE.to_escape()
                )
            )
    }
}

fn gradient(text: String, positive: bool) -> ColorString {
    if positive {
        ColorString::new(text, Color::GREEN)
    } else {
        ColorString::new(text, Color::RED)
    }
}

fn parse_moni(file: File) {
    let reader = BufReader::new(file);

    let mut sum: f32 = 0.0;
    let mut num: f32;
    let mut values: VecDeque<f32> = VecDeque::<f32>::new();

    for line in reader.lines() {

        let line = line.unwrap().trim().to_string();

        if line.eq(">>") {
            println!("{:9.2}\n", sum);
            println!("{:9.2} {} {:?}", sum, ColorString::new(">>", Color::BLUE), values);
            values.push_front(sum);
            sum = 0.0;
            println!();
            continue;
        }
        if line.eq("<<") {
            println!("{:9.2}\n", sum);
            println!("{:9.2} {} {:?}", sum, ColorString::new("<<", Color::BLUE), values);
            if let Some(val) = values.pop_front() {
                let less: bool = sum < 0.0;
                sum += val;
                println!("{:9.2}", sum);
                if less {
                    println!("{} {} is less than zero", ColorString::new("warning:", Color::YELLOW), ColorString::new(sum - val, Color::RED));
                }
            }
            println!();
            continue;
        }

        if let Ok(a) = line.parse::<f32>() {
            num = a;
        } else {
            continue;
        }

        println!("{:9.2} {}",
            sum,
            gradient(
                format!("{} {:.2}", {
                    if num >= 0.0 {
                        "<+"
                    } else {
                        "->"
                    }
                }, num.abs()
            ),
            num >= 0.0));

        sum += num;
    }
    println!("{:9.2}", sum);
}

fn main() {
    let env = std::env::var("MONI");

    match env {
        Ok(env) => {
            let file = File::open(env.as_str());
            if let Err(_) = file {
                println!("Error: file \"{}\" cannot be read", env.as_str());
                std::process::exit(1);
            }
            parse_moni(file.unwrap());
        },
        Err(_) => {
            println!("Error: environment variable MONI is not set");
            std::process::exit(1);
        }
    }

}

