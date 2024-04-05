use std::io;
use std::io::prelude::*;
use chrono::{self, NaiveDate};
use regex::{self, Regex};
use exitcode;

fn main() {
    let mut running: bool = true;

    while running {
        
        //Get first date
        let start_date = get_date("Enter start date (dd/mm/yyyy):");

        //Get second date
        let end_date = get_date("Enter end date (dd/mm/yyyy):");

        let date_diff = end_date.signed_duration_since(start_date);

        println!("{} days from {} to {}", date_diff.num_days(), start_date, end_date);

        let mut exit_answer = String::new();

        while exit_answer.as_str().trim() != "y" && exit_answer.as_str().trim() != "n" {
            println!("Would you like to run again? (y/n): ");

            io::stdin()
                .read_line(&mut exit_answer)
                .expect("No input detected!");

            match exit_answer.as_str().trim() {
                "n" => { println!("Goodbye!"); running = false; },
                "y" => { clear_screen(); },
                _ => { println!("Please enter a valid answer!") },
            }
        }
    }

    println!("Press any key to continue...");
    io::stdin().read(&mut [0u8]).unwrap();
    std::process::exit(exitcode::OK);
}

fn check_date(date_val_in: &String) -> bool{
    let reg_pattern = r"(([0][1-9])|([1-2][0-9])|([3][0-1]))\/(([0][1-9])|([1][0-2]))\/([0-9]{4})";

    let reg_check = Regex::new(reg_pattern).unwrap();

    reg_check.is_match(date_val_in.as_str())
}

fn get_date(prompt: &str) -> NaiveDate{
    let mut input_valid = false;
    let mut date_in = String::new();

    while !input_valid{ 
        println!("{}", prompt);

        let input_result = io::stdin()
            .read_line(&mut date_in);

        input_valid = match input_result {
            Ok(_) => true,
            Err(_) => false,
        };

        if input_valid {            

            input_valid = check_date(&date_in);

            if !input_valid {
                println!("Please enter a valid date (dd/mm/yyyy)");
            }
        }
        else {
            println!("Please enter a value!");
        }
    }

    NaiveDate::parse_from_str(date_in.as_str().trim(), r"%d/%m/%Y").unwrap()
}

fn clear_screen(){
    let _result = std::process::Command::new("cmd").arg("/C").arg("cls").status().or_else(|_| std::process::Command::new("clear").status()).unwrap().success();
}