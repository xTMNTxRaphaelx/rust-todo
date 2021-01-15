use std::collections::HashMap;

fn main() {
    let action = std::env::args().nth(1).expect("please specify an action");
    let item = std::env::args().nth(2).expect("please specify an item");

    let mut todo = Todo::new().expect("Initialisation of db failed");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occured: {}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("todo saved"),
                Err(why) => println!("An error occured: {}", why),
            },
        }
    }
}

struct Todo {
    // use rust builtin HashMap to store key-value pairs
    map: HashMap<String, bool>
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        // open db.json
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;

        // serialize json as hash-map
        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occurred: {}", e),
        }
    }
    fn insert(&mut self, key: String) {
        // insert a new item into our Map
        // we pass true as value
        self.map.insert(key, true);
    }
    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("db.json")?;
        // write to file with serde
        serde_json::to_writer_pretty(f, &self.map)?;
        
        Ok(())
    }
    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}