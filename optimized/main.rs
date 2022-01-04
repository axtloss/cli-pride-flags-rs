use colored::*;
fn main() {
    let mut flag_height = 0;
    for i in 0..json::parse(&std::fs::read_to_string("flags.json").unwrap()).unwrap()[std::env::var("FLAG").unwrap()].len() {
        flag_height += json::parse(&std::fs::read_to_string("flags.json").unwrap()).unwrap()[std::env::var("FLAG").unwrap()][i]["height"].as_u16().unwrap() as usize;
    }
    if let Some((terminal_size::Width(w), terminal_size::Height(h))) = terminal_size::terminal_size() {
        for i in 0..json::parse(&std::fs::read_to_string("flags.json").unwrap()).unwrap()[std::env::var("FLAG").unwrap()].len() {
            for _j in 0..json::parse(&std::fs::read_to_string("flags.json").unwrap()).unwrap()[std::env::var("FLAG").unwrap()][i]["height"].as_usize().unwrap()*if std::env::var("HEIGHT").unwrap_or("".to_string()).as_str() == "" { if h as usize > flag_height { h as usize / flag_height } else { 1 } } else { std::env::var("HEIGHT").unwrap().parse::<usize>().unwrap() } {
                for _z in 0..if std::env::var("WIDTH").unwrap_or("".to_string()) != "" { std::env::var("WIDTH").unwrap().parse::<u16>().unwrap() } else { w } {
                    print!("{}", "█".on_truecolor(colorsys::Rgb::from_hex_str(json::parse(&std::fs::read_to_string("flags.json").unwrap()).unwrap()[std::env::var("FLAG").unwrap()][i]["code"].as_str().unwrap()).unwrap().red() as u8, colorsys::Rgb::from_hex_str(json::parse(&std::fs::read_to_string("flags.json").unwrap()).unwrap()[std::env::var("FLAG").unwrap()][i]["code"].as_str().unwrap()).unwrap().blue() as u8, colorsys::Rgb::from_hex_str(json::parse(&std::fs::read_to_string("flags.json").unwrap()).unwrap()[std::env::var("FLAG").unwrap()][i]["code"].as_str().unwrap()).unwrap().green() as u8).truecolor(colorsys::Rgb::from_hex_str(json::parse(&std::fs::read_to_string("flags.json").unwrap()).unwrap()[std::env::var("FLAG").unwrap()][i]["code"].as_str().unwrap()).unwrap().red() as u8, colorsys::Rgb::from_hex_str(json::parse(&std::fs::read_to_string("flags.json").unwrap()).unwrap()[std::env::var("FLAG").unwrap()][i]["code"].as_str().unwrap()).unwrap().blue() as u8, colorsys::Rgb::from_hex_str(json::parse(&std::fs::read_to_string("flags.json").unwrap()).unwrap()[std::env::var("FLAG").unwrap()][i]["code"].as_str().unwrap()).unwrap().green() as u8));
                }
            }
        }
    }
}
