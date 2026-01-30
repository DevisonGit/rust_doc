fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);
    println!("{word}");
    s.clear();


    slices();

    println!();
    let s = String::from("VAI Cortinas");
    let new_word = first_world_slice(&s);
    println!("{new_word}");


    let a = [1,2,3,4,5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2,3]);

}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn slices() {
    let s = String::from("Hello world");
    
    let hello = &s[0..6];
    let world = &s[6..11];

    println!("{hello}");
    println!("{world}");

    let s = String::from("Hello");


    let slice = &s[0..2];
    println!("{slice}");
    let slice = &s[0..2];
    println!("{slice}");

    let len = s.len();
    let slice = &s[3..len];
    println!("{slice}");
    let slice = &s[3..];
    println!("{slice}");

    let slice = &s[0..len];
    println!("{slice}");
    let slice = &s[..];
    println!("{slice}");

}

fn first_world_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}