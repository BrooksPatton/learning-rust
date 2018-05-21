// * Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

fn main() {
    let sentence = "Hello world"; // -> ello-hay orld-way

    pig_latin(sentence);
}

fn pig_latin(str: &str) {
    for word in str.split(" ") {
        convert_to_pig_latin(word);
    }
}

fn convert_to_pig_latin(word: &str) {
    let chars: Vec<&str> = word.split("").collect();
    let first_letter = chars[1];
    let mut result = String::from("");

    // plan is to put the characters into the result String
}