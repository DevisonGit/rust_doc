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

    // aceita sinal
    let x_signed: i8 = -12;
    println!("The i8 signed {x_signed}");

    // não aceita sinal
    let y_unsiged: u8 = 12;
    println!("The u8 unsigned {y_unsiged}");

}
