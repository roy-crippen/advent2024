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

use std::time::Instant;

fn main() {
    let start = Instant::now();
    let ds = [
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
    ];

    println!();
    ds.iter().for_each(|f| {
        let solution = f();
        println!(
            "{:<30}, {:<30}, {:?}",
            solution.part_a, solution.part_b, solution.duration
        );
    });
    println!("\ntotal elapsed time: {:?}", start.elapsed());
}
