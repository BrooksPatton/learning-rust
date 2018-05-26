// * Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

fn main() {
    let sentence = "Hello world apple"; // -> ello-hay orld-way
    let pig_latin_sentence = pig_latin(sentence);

    println!("{} -> {}", sentence, pig_latin_sentence);
}

fn pig_latin(input: &str) -> String {
    // Preallocate some memory so there are fewer reallocations
    let mut result = String::with_capacity(input.len());
    for (idx, word) in input.split(" ").enumerate() {
        // For every word that is not the first, add a space before
        if idx != 0 {
            result.push(' ');
        }
        convert_to_pig_latin(word, &mut result);
    }

    result
}

fn convert_to_pig_latin(word: &str, output_buffer: &mut String) {
    // Get the first character
    let first_letter = match word.chars().next() {
        Some(ch) => ch,
        None => return, // Just return if the word is emtpy
    };

    match first_letter {
        'a' | 'e' | 'i' | 'o' | 'u' => {
            *output_buffer += word;
            *output_buffer += "-hay";
        },
        _ => {
            // Below we iterate over the length of the word, searching for the end of the first
            // character in case it spans multiple bytes.
            // Alternatively create an iterator over the chars, skip the first one and then push
            // the remaining chars onto the output string: `word.chars().skip(1).for_each(|c| output_buffer.push(c));`
            for idx in 1..word.len() {
                // Once we find the character boundary we slice the &str and add everything
                // except the first character to the output
                if word.is_char_boundary(idx) {
                    *output_buffer += &word[idx..];
                    break;
                }
            }
            output_buffer.push('-');
            // Since the lowercase version of the first letter could be multiple chars
            // use .to_lowercase() and push all resulting chars onto the output string
            first_letter.to_lowercase().for_each(|c| output_buffer.push(c));
            *output_buffer += "ay";
        },
    }
}
