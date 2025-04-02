mod question1;
mod question2;
mod question3;
mod question4;
mod question5;
mod question6;
use std::io;





fn main() {
    println!("Hello, world!");

    println!("Please select a question to run (1-6):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    match input.trim() {
        "1" => question1::run(),
        "2" => question2::run("Alice", 18, 95.5),
        "3" => question3::run(),
        "4" => question4::run(),
        "5" => question5::run(),
        "6" => {
            println!("Please enter three arguments for question 6:");
            println!("Argument 1 (string):");
            let mut arg1 = String::new();
            io::stdin().read_line(&mut arg1).expect("Failed to read argument 1");
            println!("Argument 2 (string):");
            let mut arg2 = String::new();
            io::stdin().read_line(&mut arg2).expect("Failed to read argument 2");
            println!("Argument 3 (true/false):");
            let mut arg3 = String::new();
            io::stdin().read_line(&mut arg3).expect("Failed to read argument 3");
            let arg3_bool = arg3.trim().eq_ignore_ascii_case("true");

            if let Err(e) = question6::run(arg1.trim(), arg2.trim(), arg3_bool) {
            eprintln!("Failed to run question 6: {}", e);
            }
        }
        _ => println!("Invalid selection. Please choose a number between 1 and 6."),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question1() {
        // Assuming question1::run() returns a result or has a testable side effect
        question1::run();
        // Add assertions here if applicable
    }

    #[test]
    fn test_question2() {
        // Assuming question2::run() takes parameters and has a testable output
        question2::run("TestName", 25, 88.5);
        // Add assertions here if applicable
    }

    #[test]
    fn test_question3() {
        question3::run();
        // Add assertions here if applicable
    }

    #[test]
    fn test_question4() {
        question4::run();
        // Add assertions here if applicable
    }

    #[test]
    fn test_question5() {
        question5::run();
        // Add assertions here if applicable
    }

    #[test]
    fn test_question6() {
        // Assuming question6::run() returns a Result
        let result = question6::run("arg1", "arg2", true);
        assert!(result.is_ok(), "question6::run() should return Ok");
    }
}
