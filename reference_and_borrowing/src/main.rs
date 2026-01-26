fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    // change(&s1); -> erro as referencias são imutaveis por padrão

    let mut s1 = String::from("hello");
    change(&mut s1);
    println!("{s1}");

    let reference_to_nothing = no_dangle();
    println!("{reference_to_nothing}")
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// Erro referencias são imutaveis por padrão
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}