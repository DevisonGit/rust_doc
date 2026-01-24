fn main() {
    let gifts = [
        "And a Partridge in a Pear Tree", "Two Turtle Doves",
        "Three French Hens", "Four Calling Birds",
        "Five Gold Rings","Six Geese a-Laying", 
        "Seven Swans a-Swimming", "Eight Maids a-Milking",
        "Nine Ladies Dancing", "Ten Lords a-Leaping",
        "Eleven Pipers Piping", "Twelve Drummers Drumming",
        ];
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
    ];

    let mut index = 0;
    for day in days{
        println!("The {day} day of Christmas,");
        println!("My true love sent to me");
        if index == 0 {
            println!("A Partridge in a Pear Tree");
            index += 1;
            println!();
            continue;
        }
        
        for i in (0..=index).rev() {
            if i == 0{
                println!("{}.", gifts[i])
            } else {
                println!("{},", gifts[i])
            }
        }
        index += 1;
        println!()
    }

}

