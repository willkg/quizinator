// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Quizinator
//!
//! This implements the quizinator script which can be used for practicing math facts.
//!
//! # Examples
//!
//! ```
//! $ quizinator
//! $ quizinator 5
//! $ quizinator --operation m 5 6 7
//! $ quizinator --print
//! ```

extern crate structopt;

use rand::thread_rng;
use rand::seq::SliceRandom;
use std::f64;
use std::io;
use std::io::Write;
use std::process;
use std::time::SystemTime;
use structopt::StructOpt;


// Declare a Cli detailing command line arguments
#[derive(StructOpt, Debug)]
#[structopt(about = "Quizzes you on math facts.")]
struct Cli {
    #[structopt(long = "print")]
    print_option: bool,

    #[structopt(long = "operation", default_value = "a", help = "'m', 'd', 'a', or 's'")]
    operation_option: String,

    #[structopt(multiple = true, required = false, help = "number")]
    lhs_option: Vec<i32>,
}


fn main() {
    // Build lhs and rhs
    let rhs_numbers = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let mut lhs_numbers = Vec::new();
    let opt = Cli::from_args();

    let print_mode = opt.print_option;
    println!("Print mode: {:?}.", print_mode);

    let operation = opt.operation_option.as_str();
    let operation_sym;

    match operation {
        "m" => operation_sym = "*",
        "d" => operation_sym = "/",
        "a" => operation_sym = "+",
        "s" => operation_sym = "-",
        _ => {
            println!("Unknown operation: {:?}.", operation);
            process::exit(1);
        },
    }
    println!("Using operation {:?}.", operation);

    if opt.lhs_option.len() == 0 {
        println!("No numbers specified. Adding 1 and 2.");
        lhs_numbers.push(1);
        lhs_numbers.push(2);
    } else {
        lhs_numbers.extend(opt.lhs_option);
    }
    println!("Using lhs = {:?}  and rhs = {:?}.", lhs_numbers, rhs_numbers);

    // Generate all possible problem combinations
    let mut problems = Vec::new();
    let mut rng = thread_rng();
    for lhs_num in &lhs_numbers {
        for rhs_num in &rhs_numbers {
            problems.push(vec![lhs_num.clone(), rhs_num.clone()]);
        }
    }

    // Shuffle the problems
    problems.shuffle(&mut rng);

    // Now the quiz!
    println!("QUIZINATION!!!!");
    let mut total_right: u64 = 0;
    let mut total_wrong: u64 = 0;
    let start_time = SystemTime::now();
    for problem in &problems {
        let mut guess_as_num: i32 = -1;
        let lhs: i32;
        let rhs: i32;
        let answer: i32;

        if operation == "m" {
            lhs = problem[0];
            rhs = problem[1];
            answer = lhs * rhs;
        } else if operation == "d" {
            lhs = problem[0] * problem[1];
            rhs = problem[0];
            if rhs == 0 {
                // Don't want to divide by zero--that's bad
                continue;
            }
            answer = lhs / rhs;
        } else if operation == "a" {
            lhs = problem[0];
            rhs = problem[1];
            answer = lhs + rhs;
        } else {
            if problem[0] > problem[1] {
                lhs = problem[0];
                rhs = problem[1];
            } else {
                lhs = problem[1];
                rhs = problem[0];
            }
            answer = lhs - rhs;
        }

        // If we're in print mode, then just print it
        if print_mode {
            println!("{} {} {} = ? ", lhs, operation_sym, rhs);

        } else{
            loop {
                print!("{} {} {} = ? ", lhs, operation_sym, rhs);
                io::stdout().flush().unwrap();
                let mut guess = String::new();
                io::stdin().read_line(&mut guess).expect("Failed to read line");

                match guess.trim().parse() {
                    Result::Ok(val) =>
                        guess_as_num = val,
                    Result::Err(_e) =>
                        println!("I don't understand."),
                }
                if guess_as_num != -1 {
                    break;
                }
            }

            if guess_as_num == answer {
                total_right = total_right + 1;
                println!("Correct! 😀");
            } else {
                total_wrong = total_wrong + 1;
                println!("Sorry--that's not correct. 🤮");
                println!("{} {} {} = {}", lhs, operation_sym, rhs, answer);
            }
            println!("")
        }
    }

    if !print_mode {
        // Print summary of quizinator session
        let elapsed_time: u64 = start_time.elapsed().unwrap().as_secs();
        let total_problems: u64 = total_right + total_wrong;
        let time_per_problem: f64 = f64::from_bits(elapsed_time) / f64::from_bits(total_problems);

        println!("Elapsed: {} seconds", elapsed_time);
        println!(
            "Total problems: {}, {:.3}s per problem",
            total_problems,
            time_per_problem
        );
        println!("Total right: {}", total_right);
        println!("Total wrong: {}", total_wrong);
    }
}
