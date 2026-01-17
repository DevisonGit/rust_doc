use std::io;

fn main() {
    // cria uma variavel mutavel, por padrão não são.
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    
    // constante
    const THREE_HOUR_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The constant is {THREE_HOUR_IN_SECONDS}");

    // sombreamento 
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
    
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Spaces {spaces}");

    let  guess: u32 = "42".parse().expect("Not a number");
    println!("The guess {guess}");
    println!();

    // Tipos escalar -> Inteiros, Ponto flutuante, booleanos, caracateres
    println!("Type Integer");
    type_integer();
    println!();

    println!("Type Floating-Point");
    type_float();
    println!();


    println!("Numeric Operations");
    numeric_operation();
    println!();


    println!("Type boolean");
    type_boleano();
    println!();

    println!("Type character");
    type_character();
    println!();

    println!("Type Tuple");
    type_tuple();
    println!();

    println!("Type Array");
    type_array();
    println!();

    type_array_index();

}

fn type_integer(){
    // Inteiros
    // aceita sinal
    let x_signed: i8 = -12;
    println!("The i8 signed {x_signed}");

    // não aceita sinal
    let y_unsiged: u8 = 12;
    println!("The u8 unsigned {y_unsiged}");
}

fn type_float(){
    //Ponto Flutuante
    // aceita sinal
    let x = 2.0; // Padrão f64
    println!("The f64 default {x}");
    let y: f32 = 3.0;
    println!("The f32 {y}");
}

fn numeric_operation() {
    // adição
    let sum = 5 + 10;
    println!("5 + 10 = {sum}");

    // subtração
    let difference = 95.5 - 4.3;
    println!("95.5 - 4.3 = {difference}");

    // multiplicação 
    let product = 4 * 30;
    println!("4 * 30 = {product}");

    // divisão
    let quotient = 54.7 / 32.2;
    println!("54.7 / 32.2 = {quotient}");
    let truncated = -5 / 3;
    println!("-5 / 3 = {truncated}");

    // resto da divisão
    let remainder = 43 % 5;
    println!("43 % 5 = {remainder}");
}

fn type_boleano() {
    // false ou true
    let t = true;
    println!("Type boolean {t}");
    let f = false;
    println!("Type boolean {f}");
}

fn type_character() {
    let c = 'z';
    println!("character {c}");
    let z: char = 'Z';
    println!("character {z}");
    let heart_eyed_cat = '😻';
    println!("character {heart_eyed_cat}");
}

fn type_tuple() {
    let tup: (i32, f64, u8) = (500, 5.0, 1);

    let (x, y, z) = tup;

    // desestruturação da tupla
    println!("The value of x is {x}");
    println!("The value of y is {y}");
    println!("The value of z is {z}");

    // acessando com .
    println!("The value of x is {0}", tup.0);
    println!("The value of y is {0}", tup.1);
    println!("The value of z is {0}", tup.2);
}

fn type_array() {
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "march", "April", "May", "June", 
    "July", "August", "September", "October", "November", "December"];

    //definindo [tipo; quantidade]
    let b: [i32; 5] = [1, 2, 3, 4, 5];

    // definido elementos e quiantidade
    let c = [3, 5];

    // acessando valores
    println!("Array a element [0] {0}", a[0]);
    println!("Array months element [0] {}", months[0]);
    println!("Array b elemet [0] {}", b[0]);
    println!("Array c element [0] {}", c[0]);
}

fn type_array_index() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    
    println!("The value of the element at index {index} is : {element}");

}