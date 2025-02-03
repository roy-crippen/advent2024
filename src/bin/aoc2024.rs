use std::collections::HashMap;
use std::time::Instant;

use clap::Parser;
use rayon::prelude::*;

use lib::common::Solution;
use lib::day_01::solve_day_01;
use lib::day_02::solve_day_02;
use lib::day_03::solve_day_03;
use lib::day_04::solve_day_04;
use lib::day_05::solve_day_05;
use lib::day_06::solve_day_06;
use lib::day_07::solve_day_07;
use lib::day_08::solve_day_08;
use lib::day_09::solve_day_09;
use lib::day_10::solve_day_10;
use lib::day_11::solve_day_11;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Run all AOC 2024 days in parallel
    #[arg(short, long, action)]
    par: bool,

    /// Run one AOC day
    #[arg(short, long, default_value = None)]
    day: Option<u8>,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);

    let fs = vec![
        solve_day_01,
        solve_day_02,
        solve_day_03,
        solve_day_04,
        solve_day_05,
        solve_day_06,
        solve_day_07,
        solve_day_08,
        solve_day_09,
        solve_day_10,
        solve_day_11,
    ];

    let days_m: HashMap<u8, fn() -> Solution> = fs
        .iter()
        .enumerate()
        .map(|(i, f)| ((i + 1) as u8, *f))
        .collect();

    let start = Instant::now();
    let mut solutions: Vec<Solution> = Vec::new();
    if let Some(day) = args.day {
        if let Some(f) = days_m.get(&day) {
            let solution = f();
            println!(
                "{:<30}, {:<30}, {:?}",
                solution.part_a, solution.part_b, solution.duration
            );
        }
    } else if args.par && args.day.is_none() {
        fs.par_iter().map(|&f| f()).collect_into_vec(&mut solutions);
    } else {
        solutions = fs.iter().map(|&f| f()).collect();
    }
    println!();
    for solution in solutions {
        println!(
            "{:<30}, {:<30}, {:?}",
            solution.part_a, solution.part_b, solution.duration
        );
    }
    println!("\ntotal elapsed time: {:?}", start.elapsed());
}
