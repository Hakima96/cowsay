extern crate clap;
extern crate rand;
#[macro_use]
extern crate include_dir;
extern crate wee_alloc;
use clap::{App, Arg};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::env;
// use std::fs::File;
use std::io::{self, Read};
use std::str;
// Use `wee_alloc` as the global allocator.
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
        .map(|file| file.path.replace(".cow", ""))
        .collect::<Vec<String>>()
}
fn format_animal(s: String, thoughts: &str, eyes: &str, tongue: &str) -> String {
    s.split("\n")
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
fn make_bubble(s: String, width: usize, think: bool, wrap: bool) -> String {
    // ... (reste du code inchangé)
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
        .arg(Arg::with_name("borg").short("b").help("Borg Cow"))
        .arg(Arg::with_name("dead").short("d").help("Dead Cow"))
        .arg(Arg::with_name("greedy").short("g").help("Greedy Cow"))
        .arg(Arg::with_name("paranoid").short("p").help("Paranoid Cow"))
        .arg(Arg::with_name("stoned").short("s").help("Stoned Cow"))
        .arg(Arg::with_name("tired").short("t").help("Tired Cow"))
        .arg(Arg::with_name("wired").short("w").help("Wired Cow"))
        .arg(Arg::with_name("youthful").short("y").help("Youthful Cow"))
        .arg(
            Arg::with_name("custom")
                .short("e")
                .value_name("EYE_STRING")
                .help("Custom Eyes"),
        )
        .arg(
            Arg::with_name("tongue")
                .short("T")
                .value_name("TONGUE_STRING")
                .help("Custom Tongue"),
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
    match matches.is_present("list") {
        true => {
            let list = list_cows();
            println!("{:?}", list);
            std::process::exit(0);
        }
        false => (),
    };
    let mut cow = matches.value_of("cow").unwrap_or("default").to_owned();
    cow = match matches.is_present("random") {
        true => {
            let mut rng = thread_rng();
            let cows = list_cows();
            cows.choose(&mut rng).unwrap().to_owned()
        }
        false => cow,
    };
    let width = matches
        .value_of("width")
        .unwrap_or("40")
        .parse::<usize>()
        .unwrap();
    let wrap = !matches.is_present("nowrap");
    let message_vals = match matches.values_of("MESSAGE") {
        Some(x) => x.collect::<Vec<_>>(),
        None => vec![""],
    };
    let mut message = message_vals.join(" ");
    message = match &message[..] {
        "" => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).unwrap();
            buffer.trim_end().to_string()
        }
        _ => message,
    };
    let tongue = matches.value_of("tongue").unwrap_or(" ");
    // Cow Eyes
    let borg = matches.is_present("borg");
    let dead = matches.is_present("dead");
    let greedy = matches.is_present("greedy");
    let paranoid = matches.is_present("paranoid");
    let stoned = matches.is_present("stoned");
    let tired = matches.is_present("tired");
    let wired = matches.is_present("wired");
    let youthful = matches.is_present("youthful");
    let custom = matches.value_of("custom").unwrap_or("");
    let mut custombool = false;
    if custom != "" {
        custombool = true;
    }
    let eyes = [
        (borg, "=="),
        (dead, "xx"),
        (greedy, "$$"),
        (paranoid, "@@"),
        (stoned, "**"),
        (tired, "--"),
        (wired, "OO"),
        (youthful, ".."),
        (custombool, custom),
        (true, "oo"),
    ]
    .iter()
    .filter(|&x| x.0)
    .collect::<Vec<_>>()[0]
        .1;
    let think;
    let voice;
    match env::args().collect::<Vec<_>>()[0].ends_with("cowthink") {
        true => {
            think = true;
            voice = "o"
        }
        false => {
            think = false;
            voice = "\\";
        }
    }
    let cowbody = match cow.contains(".cow") {
        true => {
            unimplemented!("Can't provide external cowfiles for now")
            // let mut f = File::open(&cow).unwrap();
            // let mut cowbody = String::new();
            // f.read_to_string(&mut cowbody)
            //     .expect(&format!("Couldn't read cowfile {}", cow));
            // cowbody
        }
        false => {
            let fmt = &format!("{}.cow", &cow);
            let file = PROJECT_DIR
                .get_file(&fmt)
                .expect(&format!("Can't find the cow file {}", cow))
                .contents;
            str::from_utf8(file).unwrap().to_string()
        }
    };
    println!("{}", make_bubble(message.to_string(), width, think, wrap));
    println!("{}", format_animal(cowbody, voice, eyes, tongue));
}
