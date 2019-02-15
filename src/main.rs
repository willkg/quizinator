use rand::thread_rng;
use rand::seq::SliceRandom;
use std::env;
use std::f64;
use std::io;
use std::io::Write;
use std::time::SystemTime;


fn main() {
    // the lhs and rhs possibilities
    let rhs_numbers = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

    let mut lhs_numbers = Vec::new();

    for arg in env::args() {
        match arg.trim().parse() {
            Result::Ok(val) =>
                lhs_numbers.push(val),
            Result::Err(_e) =>
                println!("I don't understand \"{}\".", arg),
        }
    }

    if lhs_numbers.len() == 0 {
        println!("No numbers specified. Adding 1 and 2.");
        lhs_numbers.push(1);
        lhs_numbers.push(2);
    }

    println!("Using {:?}.", lhs_numbers);

    let mut problems = Vec::new();

    // generate all possible problem combinations
    let mut rng = thread_rng();
    for lhs_num in &lhs_numbers {
        for rhs_num in &rhs_numbers {
            problems.push(vec![lhs_num.clone(), rhs_num.clone()]);
        }
    }

    // now shuffle the problems
    problems.shuffle(&mut rng);

    // now the quiz!
    println!("QUIZINATION!!!!");
    let mut total_right: u64 = 0;
    let mut total_wrong: u64 = 0;
    let start_time = SystemTime::now();
    for problem in &problems {
        let mut guess_as_num: i32 = -1;
        loop {
            print!("{} * {} = ? ", problem[0], problem[1]);
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

        if guess_as_num == problem[0] * problem[1] {
            total_right = total_right + 1;
            println!("Correct! 😀");
        } else {
            total_wrong = total_wrong + 1;
            println!("Sorry--that's not correct. 🤮");
        }
    }

    // Print summary of quizinator session
    let elapsed_time: u64 = start_time.elapsed().unwrap().as_secs();
    let total_problems: u64 = total_right + total_wrong;
    let time_per_problem: f64 = f64::from_bits(elapsed_time) / f64::from_bits(total_problems);

    println!("Elapsed: {} seconds", elapsed_time);
    println!(
        "Total problems: {}, {:.2}s per problem",
        total_problems,
        time_per_problem
    );
    println!("Total right: {}", total_right);
    println!("Total wrong: {}", total_wrong);
}
