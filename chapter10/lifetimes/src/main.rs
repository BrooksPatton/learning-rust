fn main() {
    let string1 = String::from("hello world");

    {
        let string2 = String::from("abc");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is '{}'", result);
    }
    
}

fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}