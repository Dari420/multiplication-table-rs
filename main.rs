use std::io::{stdin, Read};
use std::time::Duration;
use std::thread::{sleep};
use std::process::exit;
use std::fs;
use std::fs::File;
use std::path::Path;

enum Value {
    Int(isize),
}

fn main() {
    let path = Path::new("output.txt");
    println!("Welcome to the multiplication table generator");
    File::create(path);
    loop {
        let mut f = File::open(path).expect("Unable to open file");
        let mut data = String::new();
        let mut multiplier_rows: isize = 0;
        let mut multiplier_cols: isize = 0;
        let mut limit = String::new();
        let newline = "\n";
        println!("Pick limit:");
        stdin()
            .read_line(&mut limit)
            .expect("Fatal error");
        let c_limit = limit.replace("\n", "").replace("\r\n", "");
        match parse_string(&c_limit) {
            Some(Value::Int(i)) => {
                for rows in 0isize..c_limit.parse().unwrap() {
                    multiplier_rows += 1;
                    multiplier_cols = 0;
                    for columns in 0isize..c_limit.parse().unwrap() {
                        multiplier_cols += 1;
                        let mut product = &multiplier_rows * multiplier_cols;
                        print!("{}, ", product);
                        &data.push_str(&product.to_string());
                        &data.push_str(", ");
                        fs::write(path, &data);
                    }
                    &data.push_str("\n");
                    print!("\n")
                }
                println!("Done");
            }
            None => {
                println!("Invalid entry! Please enter a non decimal number");
                continue
            }
        }
        loop {
            let mut again_or_not = String::new();
            println! ("Go again? y/n");
            stdin()
                .read_line(&mut again_or_not)
                .expect("Invalid entry! crashing");
            match again_or_not.trim_end() {
                "y" => {
                    break;
                },
                "n" => {
                    println! ("Closing calculator");
                    sleep(Duration::new(1, 0));
                    exit(0);
                },
                _ => println! ("Invalid entry! Please choose y or n")
            }
        }
    }
}
//parse strings
fn parse_string(s: &str) -> Option<Value> {
    if let Ok(i) = s.parse() {
        Some(Value::Int(i))
    } else {
        None
    }
}