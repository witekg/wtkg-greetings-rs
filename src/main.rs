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

const WEEK_NAMES : [&str; 7] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];

fn main() {
    let args: Vec<String> = env::args().collect();
    let local: DateTime<Local> = Local::now();
    let greet = greeting_text(local.hour());
    if args.len() < 2 {
        println!("{}, whoever you are!", greet);
    } else {
        println!("{}, {}!", greet, args[1]);
    }

    let weekday = local.weekday();
    let custom_date = local.format("%Y.%m.%d");
    let custom_time = local.format("%H:%M:%S");
    println!("Today is {}, {}, and time is {}",
        WEEK_NAMES[weekday.num_days_from_monday() as usize], custom_date, custom_time);
    
    let mp = moon_phase::MoonPhase::new(SystemTime::now());
    println!("The Moon is {}, and it is in {}", mp.phase_name, mp.zodiac_name);
}

fn greeting_text(hour: u32) -> String {
    if hour < 12 {
        return "Good morning".to_string();
    } else if hour < 17 {
        return "Good afternoon".to_string();
    } else {
        return "Good evening".to_string();
    }
}

