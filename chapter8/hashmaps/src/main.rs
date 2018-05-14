use std::collections::HashMap;

fn main() {
    create_hash_map();
    create_hash_map_with_references();
    use_zip_to_create_hash_map();
    collecting_words_from_string();
}

fn create_hash_map() {
    let mut scores = HashMap::new();
    let yellow = String::from("yellow");

    scores.insert(String::from("red"), 12);
    scores.insert("blue".to_string(), 24);
    scores.insert(yellow, 22);
    scores.entry("yellow".to_string()).or_insert(50); // If yellow exists, do nothing
    scores.entry("magenta".to_string()).or_insert(55); // If magenta does not exist add it and set to 55

    println!("creating hash map: {:?}", scores);

    let red_score = scores.get("red"); // This cannot be a String, but must be a reference
    println!("The red score is: {}", red_score.unwrap());
}

fn create_hash_map_with_references() {
    let mut scores = HashMap::new();
    let yellow = "yellow";
    let red = "red";

    scores.insert(yellow, 23);
    scores.insert(red, 24);

    println!("creating hash map with references: {:?}", scores);

    for (key, value) in scores {
        println!("For loop key: {} and value: {}", key, value);
    }
}

fn use_zip_to_create_hash_map() {
    let teams = vec!["red", "blue"];
    let scores = vec![55, 23];
    let mut teams_hash: HashMap<_, _> = teams.into_iter().zip(scores.into_iter()).collect();

    println!("using zip: {:?}", teams_hash); 

    teams_hash.insert("red", 12); // after using .iter, we would need references. .into_iter takes ownership
    println!("Now red has been updated: {:?}", teams_hash);
}

fn collecting_words_from_string() {
    let my_string = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in my_string.split("") {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("The characters in the str: {:?}", map);
}