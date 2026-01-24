fn main() { // s não e valido aqui
    let s = "hello"; // s é valido deste ponto em diante

    println!("{s}");

    let mut s1 = String::from("hello");
    s1.push_str(", world!");
    println!("{s1}");

    let s2 = s1; // Rust passsa a considerar s1 como invalido
    // println!("{s1}"); -> esta linha da erro

    // a string original sai imediatamente do escopo
    let mut s2 = String::from("hello");
    s2 = String::from("ahoy"); // executa o drop
    println!("{s2}");

    // clone -> dados do heap são copiados
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 {s2}");

    let s = String::from("hello");
    takes_ownership(s); // s foi passado para a propriedade da função
    // println!("{s}") gera erro s não existe mais aqui

    let x = 5;
    makes_copy(x);
    println!("{x}"); // x existe pq i32 implementa copia trait

    // retornar valores tbm pode transferir a propriedade
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    println!("{s1}");
    // println!("{s2}"); não existe mais
    println!("{s3}");

} // s não é mais valido aqui


fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}


fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String{
    a_string
}