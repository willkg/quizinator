use rand::thread_rng;
use rand::seq::SliceRandom;
use std::io;
use std::io::Write;
use std::time::SystemTime;


fn main() {
    // the lhs and rhs possibilities
    let lhs_numbers = vec![1, 2];
    let rhs_numbers = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

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
    let mut total_right: i32 = 0;
    let mut total_wrong: i32 = 0;
    let start_time = SystemTime::now();
    for problem in &problems {
        print!("{} * {} = ", problem[0], problem[1]);
        io::stdout().flush().unwrap();
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess_as_num: i32 = guess.trim().parse().unwrap();

        if guess_as_num == problem[0] * problem[1] {
            total_right = total_right + 1;
            println!("Correct!");
        } else {
            total_wrong = total_wrong + 1;
            println!("Sorry--that's not correct.");
        }
    }
    println!("Elapsed: {} seconds", start_time.elapsed().unwrap().as_secs());
}