use std::io;

fn main() {
    let number = read_value();
    print!("While:");
    fibonacci_while(number);
    println!("");
    print!("For:");
    fiboacci_for(number);
    println!("");
}

fn read_value() -> u32 {
    loop {
        println!("Please, digit a number");
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Failed to read line");
    
        let number: u32 = match number.trim().parse() {
            Ok(number) => break number,
            Err(_) => continue
        };
    }
}

fn fibonacci_while(mut number: u32) {

    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut sum: u128;

    print!("{a} {b} ");

    while number > 2 {
        
        sum = a + b;
        a = b;
        b = sum;

        number -= 1;
        print!("{sum} ");
    }
}


fn fiboacci_for(number: u32) {

    let mut a: u128 = 0;
    let mut b: u128 = 1;

    for _ in 0..number {
        print!("{a} ");
        let sum: u128 = a + b;
        a = b;
        b = sum;
    }

}