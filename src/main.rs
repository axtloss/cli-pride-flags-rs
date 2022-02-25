use std::fs;
use json;
use colored::*;
use terminal_size::{Width, Height, terminal_size};
use rand::{Rng, thread_rng};
use clap::{Arg, App};

fn main() {

    let matches = App::new("cli-pride-flags-rs")
        .version("0.1.0")
        .author("Amy <axtlos@tar.black>")
        .about("A CLI tool to generate pride flags")
        .arg(Arg::with_name("flag")
            .short("F")
            .long("flag")
            .value_name("FLAG")
            .help("Sets the flag to use")
            .takes_value(true)
            .required(true)
            .index(1))
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .value_name("FILE")
            .help("Sets a custom json file to use")
            .takes_value(true))
        .arg(Arg::with_name("width")
            .short("w")
            .long("width")
            .value_name("WIDTH")
            .help("Sets the width of the pride flag")
            .takes_value(true))
        .arg(Arg::with_name("height")
            .short("h")
            .long("height")
            .value_name("HEIGHT")
            .help("Sets the height of the pride flag")
            .takes_value(true))
        .get_matches();
        

    let mut flag = matches.value_of("flag").unwrap();
    let flags = json::parse(&fs::read_to_string(matches.value_of("file").unwrap_or("flags.json")).unwrap()).unwrap();
    let mut available_flags: Vec<&str> = Vec::new();
    for i in flags.entries() {
        available_flags.push(i.0)
    }
    if flag == "random" {
        let y: usize = thread_rng().gen_range(0, flags.len());
        flag = available_flags[y];
    }
    let len = flags[flag].len();
    let mut flag_height = 0;
    for i in 0..len.clone() {
        flag_height += flags[flag][i]["height"].as_u16().unwrap() as usize;
    }
    if let Some((Width(mut w), Height(h))) = terminal_size() {
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
            for _j in 0..flags[flag][i]["height"].as_usize().unwrap()*multiplier {
                let rgb1 = colorsys::Rgb::from_hex_str(flags[flag][i]["code"].as_str().unwrap()).unwrap();
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
