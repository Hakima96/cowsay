extern crate clap;
extern crate rand;
#[macro_use]
extern crate include_dir;
extern crate wee_alloc;

use clap::{App, Arg};
use rand::prelude::*;
use std::io::{self, Read};
use std::str;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use include_dir::Dir;

const PROJECT_DIR: Dir = include_dir!("src/cows/");

struct CowBubble {
    sleft: &'static str,
    sright: &'static str,
    topleft: &'static str,
    midleft: &'static str,
    botleft: &'static str,
    topright: &'static str,
    midright: &'static str,
    botright: &'static str,
}

fn list_cows() -> Vec<String> {
    PROJECT_DIR
        .files()
        .iter()
        .map(|file| file.path().to_string_lossy().replace(".cow", ""))
        .collect()
}

fn format_animal(s: String, thoughts: &str, eyes: &str, tongue: &str) -> String {
    s.lines()
        .filter(|&x| !x.starts_with("##") && !x.contains("EOC"))
        .collect::<Vec<_>>()
        .join("\n")
        .trim_end()
        .replace("$eyes", eyes)
        .replace("$thoughts", thoughts)
        .replace("$tongue", tongue)
        .replace("\\\\", "\\")
        .replace("\\@", "@")
}

fn make_bubble(s: String, _width: usize, _think: bool, _wrap: bool) -> String {
    s
}

fn main() {
    let matches = App::new("cowsay")
        .version("v0.1.0")
        .author("Syrus Akbary <syrus@wasmer.io>")
        .arg(
            Arg::with_name("MESSAGE")
                .help("Message for cow to say")
                .multiple(true),
        )
        .arg(
            Arg::with_name("cow")
                .short("f")
                .value_name("COW")
                .help("Which cow should say"),
        )
        .arg(
            Arg::with_name("width")
                .short("W")
                .value_name("WIDTH")
                .help("Max width of cow text bubble"),
        )
        .arg(
            Arg::with_name("nowrap")
                .short("n")
                .help("Disable word wrap"),
        )
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("list")
                .help("List Cows"),
        )
        .arg(
            Arg::with_name("random")
                .long("random")
                .help("Choose random cow"),
        )
        .get_matches();

    if matches.is_present("list") {
        let list = list_cows();
        println!("{:?}", list);
        std::process::exit(0);
    }

    let mut cow = matches.value_of("cow").unwrap_or("default").to_owned();
    if matches.is_present("random") {
        let cows = list_cows();
        cow = cows.choose(&mut thread_rng()).unwrap().to_owned();
    }

    let width = matches
        .value_of("width")
        .unwrap_or("40")
        .parse::<usize>()
        .unwrap();
    let wrap = !matches.is_present("nowrap");
    let message_vals: Vec<&str> = matches.values_of("MESSAGE").unwrap_or_default().collect();
    let mut message = message_vals.join(" ");
    if message.is_empty() {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();
        message = buffer.trim_end().to_string();
    }

    let tongue = matches.value_of("tongue").unwrap_or(" ");

    // Charger le contenu du fichier de la vache
    let cowbody = PROJECT_DIR
        .get_file(&format!("{}.cow", cow))
        .expect("Cow file not found")
        .contents_utf8()
        .expect("Failed to read cow file");

    println!("{}", make_bubble(message, width, false, wrap));
    println!("{}", format_animal(cowbody.to_string(), "\\", "oo", tongue));
}
