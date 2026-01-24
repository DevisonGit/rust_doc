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

    println!("Version ChatGPT");
    my_version(&gifts, &days);

    let gifts = [
        "a Partridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Gold Rings",
        "Six Geese a-Laying",
        "Seven Swans a-Swimming",
        "Eight Maids a-Milking",
        "Nine Ladies Dancing",
        "Ten Lords a-Leaping",
        "Eleven Pipers Piping",
        "Twelve Drummers Drumming",
    ];

    println!("Version ChatGPT");
    chatgpt_version(&gifts, &days);


}

fn my_version(gifts: &[&str], days: &[&str]) {

    let mut index = 0;
    for day in days{
        println!("The {day} day of Christmas,");
        println!("My true love sent to me");
        if index == 0 {
            println!("A Partridge in a Pear Tree.");
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

fn chatgpt_version (gifts: &[&str], days: &[&str]) {
    for (day_index, day) in days.iter().enumerate(){
        println!("The {} day of Christmas,", day);
        println!("My true love sent to me");

        for gift_index in (0..=day_index).rev() {
            if gift_index == 0 && day_index !=0 {
                println!("And {}.", gifts[gift_index]);
            } else if gift_index == 0 {
                println!("{}.", gifts[gift_index]);
            } else {
                println!("{},", gifts[gift_index]);
            }
        }
    println!()
    }
}