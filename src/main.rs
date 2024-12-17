#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
)]

use std::{env, time::SystemTime};
use chrono::prelude::*;
use moon_phase;

fn main() {
    let args: Vec<String> = env::args().collect();
    let local: DateTime<Local> = Local::now();
    let greet = greeting_text(local.hour());
    // dbg!(args);
    if args.len() < 2 {
        println!("{}, whoever you are!", greet);
    } else {
        println!("{}, {}!", greet, args[1]);
    }
    println!("Today is {}.{}.{}, and time is {}:{}:{}", 
        local.day0(), local.month(), local.year(), local.hour(), local.minute(), local.second());
    let mp = moon_phase::MoonPhase::new(SystemTime::now());
    println!("The Moon is {}, and it is in {}", mp.phase_name, mp.zodiac_name);
}

fn greeting_text(hour: u32) -> String {
    let mut str = String::from("Good ");
    if hour < 12 {
        str = str + "morning";
    } else if hour < 17 {
        str = str + "afternoon"
    } else {
        str = str + "evening";
    }
    return str;
}

