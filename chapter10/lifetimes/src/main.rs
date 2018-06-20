struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'ab> ImportantExcerpt<'ab> {
    fn announce_and_return_part<'b>(&'b self, annountment: &'b str) -> &'b str {
        println!("Hey, this is the announcement: {}", annountment);
        if annountment.len() > 5 {
            annountment
        } else {
        self.part
        }
    }
}

fn main() {
    let string1 = String::from("hello world");

    {
        let string2 = String::from("abc");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is '{}'", result);
    }
    
    let sentence; // We need to declare sentence here because otherwise it would be removed from memory before excerpt (things are removed in opposite order that they are created (a stack))
    let excerpt;

    {
        sentence = String::from("This is the first part. Thes is the second");
        let first_part = sentence.split(".")
            .next()
            .expect("could not find a sentence");
        excerpt = ImportantExcerpt {part: first_part};
    }

    println!("The exerpt is {}", excerpt.part);
    println!("{}", excerpt.announce_and_return_part("Hello world"));
}

fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}