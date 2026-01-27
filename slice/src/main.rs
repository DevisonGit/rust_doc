fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);
    s.clear();

    slices();
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


}
