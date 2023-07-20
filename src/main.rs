use std::{io::{self}, collections::HashMap};
// use crate::averages::{mean, median, mode};

mod averages;

fn main() {
    // println!("Enter the list of numbers");

    // let mut input = String::new();

    // io::stdin()
    //     .read_line(&mut input)
    //     .expect("Failed to read line");

    // let mut numbers: Vec<i32> = input.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

    // dbg!(mean(&numbers));
    // dbg!(median(&mut numbers));
    // dbg!(mode(&numbers));
    let mut store = HashMap::new();

    loop {
        println!("Enter a command:");

        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read command");

        let split_command: Vec<&str> = command.split_whitespace().collect();

        match split_command.as_slice() {
            ["Add", name, "to", department] => {
                let entry = store.entry(department.to_string()).or_insert(Vec::new());
                entry.push(name.to_string())
            },
            ["Exit"] => {
                break;
            },
            ["Show", "departments"] => {
                for (key, _) in &store {
                    println!("{key}")
                }
            },
            ["Show", "people"] => {
                for (_, people) in &store {
                    for person in people {
                        println!("{person}")
                    }
                }
            },
            [_, ..] => {

            },
            [] => {
                println!("No command!")
            }
        }
    }
}

