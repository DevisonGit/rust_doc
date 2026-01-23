use std::io;

fn main() {

    const FAHRENHEIT: &str = "Fahrenheit";
    const CELSIUS: &str = "Celsius";

    loop {
        println!("Please choise");
        println!("1 - converter {FAHRENHEIT} to {CELSIUS}");
        println!("2 - converter {CELSIUS} to {FAHRENHEIT} ");

        let choise = read_values();

        if choise == 1.0 {
            loop {
                println!("Enter {FAHRENHEIT} value: ");
                let fahr = read_values();
                let cels = fahrenheit_to_celsius(fahr);
                println!("{fahr}°F == {cels}°C");
                break;
            }

        } else if choise == 2.0 {
            println!("Enter {CELSIUS} value: ");
            let cels = read_values();
            let fahr = celsius_to_fahrenheit(cels);
            println!("{cels}°C == {fahr}°F")
        }

    }

}


fn fahrenheit_to_celsius(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f32) -> f32 {
    (c *  9.0 / 5.0) + 32.0
}

fn read_values() -> f32 {
    let mut choise = String::new();
    io::stdin().read_line(&mut choise).expect("Failed to read line");

    let choise: f32 = match choise.trim().parse() {
        Ok(ch) => ch,
        Err(_) => 0.0
    };
    choise
}