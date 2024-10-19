use chrono::{DateTime, Local};
use numfmt::Formatter;
use rand::Rng;
use std::{
    io::{stdin, stdout, Write},
    time::{Duration, Instant},
};

use num::integer::gcd;

// https://stackoverflow.com/questions/34837011/how-to-clear-the-terminal-screen-in-rust-after-a-new-line-is-printed
fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn ask_user_for_dice_info() -> (u128, u128) {
    let sides = ask_for_number("How many sides should the die have? ");
    if sides < 2 {
        println!("Error; that's now how dice work.\n");
        return ask_user_for_dice_info();
    }

    let streak_resets = ask_for_number("How many of those sides should break the streak? ");
    if streak_resets >= sides || streak_resets == 0 {
        println!("Error; that doesn't make any sense. Let's try again.\n");
        return ask_user_for_dice_info();
    }

    return (sides, streak_resets);
}

fn ask_for_number(prompt: &str) -> u128 {
    print!("{}", prompt);
    let mut input = String::new();
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut input)
        .expect("Did not enter a correct string.\n");

    input
        .trim()
        .parse()
        .expect("Error; that's not a valid integer.\n")
}

fn simplify_odds(top: u128, bottom: u128) -> (u128, u128) {
    let deno = gcd(top, bottom);
    return (top / deno, bottom / deno);
}

fn print_streak(
    streak: usize,
    last: usize,
    roll_count: u128,
    last_streak_time: Instant,
    last_streak_date: DateTime<Local>,
    roll_odds: (u128, u128),
    decimal_fmt: &mut Formatter,
) {
    clear();

    let formatted_streak = decimal_fmt.fmt2(streak as f64).to_string();

    let formatted_roll_odds_top = decimal_fmt.fmt2(roll_odds.0).to_string();
    let formatted_roll_odds_bot = decimal_fmt.fmt2(roll_odds.1).to_string();

    println!(
        "Each roll has a {} in {} chance of continuing the streak.",
        formatted_roll_odds_top, formatted_roll_odds_bot
    );

    println!("Average rolls/second: {}\n", {
        decimal_fmt.fmt2((roll_count as f64) / last_streak_time.elapsed().as_secs_f64())
    });

    println!(
        "{} | Reached a {} roll streak. (+{})",
        last_streak_date.format("%a %b %e %r").to_string(),
        formatted_streak,
        streak - last
    );

    let chance = roll_odds.0 as f64 / roll_odds.1 as f64;

    let recip = chance.recip();

    let formatted_streak_odds_frac = decimal_fmt
        .fmt2((recip.powi(streak as i32) * 10.0).round())
        .to_string();
    println!(
        "\tThis streak had a 1 in {} chance of happening.",
        formatted_streak_odds_frac
    );

    let formatted_last = decimal_fmt.fmt2(last as f64);
    let formatted_elapsed = format_elapsed(last_streak_time.elapsed());
    println!(
        "\tPrevious highest streak ({}) was {} ago.",
        formatted_last, formatted_elapsed
    );

    println!("\n\nPress [Ctrl + C] to exit the program.");
}

fn format_elapsed(elapsed: Duration) -> String {
    let as_secs = elapsed.as_secs();
    match as_secs {
        0..1 => {
            return format!("{} miliseconds", elapsed.as_millis());
        }
        1..2 => {
            return format!("{} second", as_secs);
        }
        2..60 => {
            return format!("{} seconds", as_secs);
        }
        60..120 => {
            return format!("{} minute", as_secs / 60);
        }
        120..3600 => {
            return format!("{} minutes", as_secs / 60);
        }
        3600.. => {
            return format!("{} hours", ((as_secs * 10) as f32 / 3600.0).round() / 10.0);
        }
    }
}

fn main() {
    clear();
    let (sides, streak_resets) = ask_user_for_dice_info();

    let roll_odds = simplify_odds(sides - streak_resets, sides);

    let mut highest_streak = 0;
    let mut last_record = 0;
    let mut current_streak = 0;
    let mut roll_count = 0;
    let mut last_streak_ts = Instant::now();
    let mut last_streak_date = Local::now();
    let mut last_printout = Instant::now();

    let mut rng = rand::thread_rng();
    let mut decimal_formatter: Formatter = "[.0n]".parse().unwrap();

    loop {
        if last_printout.elapsed().as_secs_f32() > 0.5 {
            print_streak(
                highest_streak,
                last_record,
                roll_count,
                last_streak_ts,
                last_streak_date,
                roll_odds,
                &mut decimal_formatter,
            );
            last_printout = Instant::now();
        }

        let roll = rng.gen_range(1..=sides);
        current_streak += 1;
        roll_count += 1;
        if roll > streak_resets {
        } else if current_streak > highest_streak {
            last_record = highest_streak;
            highest_streak = current_streak;
            current_streak = 0;
            roll_count = 0;
            last_streak_ts = Instant::now();
            last_streak_date = Local::now();
        } else {
            current_streak = 0;
        }
    }
}
