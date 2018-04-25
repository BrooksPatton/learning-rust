fn main() {
    let days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelth"
    ];

    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four colly birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    for i in 0..days.len() {
        println!("On the {} day of christmas my true love sent to me", days[i]);
        println!("{}", gifts[i]);

        for j in (0..i).rev() {
            println!("{}", gifts[j]);
        }

        println!("\n", )
    }
}