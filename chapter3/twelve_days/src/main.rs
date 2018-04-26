const DAYS: [&str; 12] = 
[
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

// The type of an array is [type; number_of_items]. 
// Arrays cannot be resized. 
// Since the data for the lyrics don't change, it's best to make them constants.
// 
// Constants must be annotated with their type, the type can't be inferred
// For a function returning strings, you can use fn return_strings() -> [&'static str; 12]
const GIFTS: [&str; 12] = 
[
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

fn main() {
    // .iter() creates an Iterator<DAYS> that returns a &str
    // .iter().enumerate creates an Enumerate<Iterator<DAYS>> that returns a (usize, &str)
    // For more methods, see: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.for_each
    //
    // (i, day) destructures that (usize, &str) into variables that can be used inside the loop
    // Note: Iterators don't need bounds-checking, while array[i] needs to check the bounds
    for (i, day) in DAYS.iter().enumerate() {
        println!("On the {} day of christmas my true love sent to me", day);
        println!("{}", GIFTS[i]);
        // same as writing a for-loop, but one-liner using .for_each() + closure
        // however, "break" and "continue" aren't possible using .for_each()
        (0..i).rev().for_each(|j| println!("{}", GIFTS[j]));
        println!("\n");
    }
}
