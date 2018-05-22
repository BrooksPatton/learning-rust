// * Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

fn main() {
    let sentence = "Hello world apple".to_string(); // -> ello-hay orld-way
    let pig_latin_sentence = pig_latin(&sentence);

    println!("{} -> {}", sentence, pig_latin_sentence);
}

fn pig_latin(str: &str) -> String {
    let mut result = std::string::String::new();
    for word in str.split(" ") {
        result = result + &convert_to_pig_latin(word) + " ";
    }

    let length = result.len();
    result.remove(length - 1);
    result
}

fn convert_to_pig_latin(word: &str) -> String {
    let mut word = word.to_string();
    let first_letter = word.get(..1);
    let first_letter = first_letter.unwrap();
    
    match first_letter {
        "a" | "e" | "i" | "o" | "u" => first_letter.to_string() + &word + "hay",
        _ => {
            let result = std::string::String::new();
            word.push('-');
            word.push(first_letter.to_ascii_lowercase());
            word + "ay"
        }
    }
}