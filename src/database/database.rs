use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::os::unix::prelude::FileExt;

pub struct Database {
    pub lists: HashMap<String, Vec<String>>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            lists: HashMap::new(),
        }
    }

    pub fn write(&self) {
        let file = File::open("db/lists.sl").unwrap();
        let mut offset: u64 = 0;

        for user in self.lists.keys() {
            let data = format!("{}#{}", user, self.lists.get(user).unwrap().join(","));
            offset = file.write_at(data.as_bytes(), offset).unwrap() as u64;
        }
    }

    pub fn read(&mut self) -> &Self {
        let file = File::open("db/lists.sl").unwrap();
        let lines = std::io::BufReader::new(file).lines();

        for line in lines {
            let line = line.unwrap();
            let data: Vec<&str> = line.split("#").collect();
            let user = data[0];
            let line: Vec<&str> = data[1].split(",").collect();
            let mut lines_owned: Vec<String> = vec![];
            for item in line {
                lines_owned.push(item.to_owned())
            }

            self.lists.insert(user.to_owned(), lines_owned);
        }

        self
    }
}
