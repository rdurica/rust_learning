use std::collections::HashMap;

pub fn hashmap_example() {
    let mut heroes: HashMap<i32, &str> = HashMap::new();
    heroes.insert(1, "Flash");
    heroes.insert(2, "Batman");

    for (k, v) in heroes.iter() {
        println!("{}: {}", k, v);
    }

    if heroes.contains_key(&1) {
        println!("Key 1 exists");
    }
}
