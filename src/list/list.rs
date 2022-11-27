use std::io::stdin;

pub struct List {
    pub list: Vec<String>,
    pub name: String,
    pub current_user_index: u32,
}

impl List {
    pub fn new() -> Self {
        Self {
            list: vec![],
            name: String::from(""),
            current_user_index: 0,
        }
    }

    pub fn print_menu(&mut self) -> &Self {
        println!("*********************");
        println!(" 1 - Print list.");
        println!(" 2 - Add to list.");
        println!(" 3 - Delete from list.");
        println!(" 4 - Quit.");
        println!(" Enter your choice and press return: ");

        let mut choice = String::new();
        stdin().read_line(&mut choice).unwrap();

        match choice.parse::<u32>().unwrap() {
            1 => self.print_list(),
            2 => self.add_item(),
            3 => self.delete_item(),
            4 => return self,
            _ => {
                println!("Sorry, choice not implemented yet.");
                self
            }
        }
    }

    pub fn print_list(&self) -> &Self {
        println!("*** Printing List ***");
        println!("{}", self.list.join("*"));

        self
    }

    pub fn add_item(&mut self) -> &Self {
        println!("*** Add Item ***");
        println!("Type in an item and press enter: ");

        let mut item = String::new();
        stdin().read_line(&mut item).unwrap();

        self.list.push(item);

        println!("Successfully added an item to the list.");

        self
    }

    pub fn delete_item(&mut self) -> &Self {
        if self.list.is_empty() {
            println!("No items to delete.");
            return self; // This return is not needed but it's easier to read if I add it.
        }

        println!("*** Delete Item ***");
        println!("Select an item index number to delete.");

        for i in 0..(self.list.len()) {
            print!("{}: {}", i, self.list[i]);
        }

        let mut choice = String::new();
        let index = stdin().read_line(&mut choice).unwrap() - 1;

        if index > self.list.len() {
            println!("Invalid index.");
            return self.delete_item();
        }

        self.list.remove(index);

        self
    }
}
