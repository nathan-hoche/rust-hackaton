pub struct Animal {
    race: String,
    name: String,
    age: i32,
}
  
pub struct Dog {
    race: String,
    name: String,
    age: i32,
}

trait Extract {
    fn get_name(&self) -> &String;
    fn get_race(&self) -> &String;
}

impl Extract for Animal {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_race(&self) -> &String {
        &self.race
    }

}

impl Extract for Dog {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_race(&self) -> &String {
        &self.race
    }
}

fn main() {
    let cat = Animal {
        name: "Fluppy".to_string(),
        race: "Cat".to_string(),
        age: 2,
    };

    let dog = Dog {
        name: "Luigi".to_string(),
        race: "Bulldog Français".to_string(),
        age: 4,
    };

    println!("This animal is a {} and his name is {}", cat.get_race(), cat.get_name());
    println!("This dog is a {} and his name is {}", dog.get_race(), dog.get_name());
}