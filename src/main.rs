mod database;
mod list;

use std::io::stdin;
use std::env;
use database::Database;
use list::List;

fn print_menu(list: &mut List) -> &List {
    println!("*********************");
    println!(" 1 - Print list.");
    println!(" 2 - Add to list.");
    println!(" 3 - Delete from list.");
    println!(" 4 - Quit.");
    println!(" Enter your choice and press return: ");

    let mut choice = String::new();
    stdin().read_line(&mut choice).unwrap();

    match choice.trim_end().parse::<u32>().unwrap() {
        1 => list.print_list(),
        2 => list.add_item(),
        3 => list.delete_item(),
        4 => std::process::exit(0),
        _ => {
            println!("Sorry, choice not implemented yet.");
            list
        },
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Invalid arguments.");
        return;
    }

    let user = &args[1];
    let mut database = Database::new();
    database.read();
    let mut list = match database.lists.get(user) {
        Some(data) => {
            List {
                list: data.to_owned(),
                name: user.to_owned(),
            }
        },
        None => {
            println!("User not found. Creating new user.");
            let list = List::new();
            database.lists.insert(user.to_owned(), list.list.to_owned());
            database.write();
            list
        },
    };

    loop {
        print_menu(&mut list);
        database.write();
    }
}

