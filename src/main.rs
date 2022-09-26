/*
GOAL:

Make a to-do list app that gives two options: view list or append to list.
the program should be a loop that allows the user to do either action as many
times as they want before quitting.
*/
use std::io;
use clearscreen;
use colored::*;

fn main() {
    println!("TODO");

    let mut running = true;
    let mut todo_list: Vec<String> = Vec::new();

    while running
    {
        println!("({})iew list, ({})ppend to list or ({})uit",
            "V".blue().bold(),
            "A".blue().bold(),
            "Q".blue().bold());

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)
            .expect("read error");

        clearscreen::clear().expect("failed to clear screen");

        match choice.trim() // trim the choice string, 
            .to_lowercase() // convert to lowercase
            .as_ref()       // and compare as a reference
        {
            "v" => view_list(&mut todo_list),
            "a" => append_list(&mut todo_list),
            "q" => running = false,
            _ => println!("Invalid input!"),
        };

        
    }
}

fn view_list(provided_list: &mut Vec<String>)
{
    if provided_list.len() < 1
    {
        println!("{}", "Todo list is empty.".red().bold());
    }
    println!("List length: {}, contents:", provided_list.len());

    for i in 0..provided_list.len()
    {
        println!("{}: {}", i+1, provided_list[i].trim());
    }
}

fn append_list(provided_list: &mut Vec<String>)
{
    let mut new_item = String::new();
    io::stdin().read_line(&mut new_item)
        .expect("read error");

    provided_list.push(new_item);
}