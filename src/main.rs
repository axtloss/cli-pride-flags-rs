use std::fs;
use json;
use colored::*;
use terminal_size::{Width, Height, terminal_size};
use rand::thread_rng;
use rand::Rng;

fn main() {
    let parsed = json::parse(&fs::read_to_string("flags.json").unwrap()).unwrap();
    let flag = std::env::var("FLAG").unwrap();
    let mut flag = flag.as_str();
    let size = terminal_size();
    let flags = vec!["agen", "aro", "asex", "bi", "bigen", "classic", "enby", "fluid", "les", "pan", "queer", "trans"];
    let mut rng = thread_rng();
    if flag == "random" {
        let y: usize = rng.gen_range(0, flags.len());
        flag = flags[y];
    }
    let len = parsed[flag].len();
    let mut flag_height = 0;
    for i in 0..len.clone() {
        flag_height += parsed[flag][i]["height"].as_u16().unwrap() as usize;
    }
    if let Some((Width(mut w), Height(h))) = size {
        let multiplier: usize;
        match std::env::var("HEIGHT").unwrap_or("".to_string()).as_str() {
            "" => {
                multiplier = if h as usize > flag_height { h as usize / flag_height } else { 1 };

            }
            _ => {
                multiplier = std::env::var("HEIGHT").unwrap().parse::<usize>().unwrap();
            }
        }
        if std::env::var("WIDTH").unwrap_or("".to_string()) != "" {
            w = std::env::var("WIDTH").unwrap().parse::<u16>().unwrap();
        }
        for i in 0..len.clone() {
            for _j in 0..parsed[flag][i]["height"].as_usize().unwrap()*multiplier {
                let rgb1 = colorsys::Rgb::from_hex_str(parsed[flag][i]["code"].as_str().unwrap()).unwrap();
                let red = rgb1.red() as u8;
                let blue = rgb1.blue() as u8;
                let green = rgb1.green() as u8;
                for _z in 0..w  {
                    print!("{}", "█".on_truecolor(red, green, blue).truecolor(red, green, blue));
                }
            }
        }
    } else {
        println!("Can't get terminal size");
    }
}
