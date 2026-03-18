const DAYS: [(&str, &str); 12] = [
    ("first", "And a partridge in a pear tree"),
    ("second", "Two turtle doves"),
    ("third", "Three french hens"),
    ("fourth", "Four calling birds"),
    ("fifth", "Five golden rings"),
    ("sixth", "Six geese a-laying"),
    ("seventh", "Seven swans a-swimming"),
    ("eigth", "Eight maids a-milking"),
    ("ninth", "Nine ladies dancing"),
    ("tenth", "Ten lords a-leaping"),
    ("eleventh", "Elven pipers piping"),
    ("twelfth", "Twelve drummers drumming"),
];

fn main() {
    println!("On the {} of Christmas, my true love sent to me", DAYS[0].0);
    println!("A partridge in a pear tree");
    println!();
    for i in 1..12 {
        println!("On the {} of Christmas, my true love sent to me", DAYS[i].0);
        println!("{}", DAYS[i].1);
        for j in (0..i).rev() {
            println!("{}", DAYS[j].1)
        }
        println!()
    }
}
