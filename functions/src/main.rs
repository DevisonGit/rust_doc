fn main() {
    println!("Hello, world!");
    another_function();
    another_function_value(5);
    print_labeled_measurement(10, 'h');
    statements();
    call_five();
    call_plus_one();
}

// definindo uma função
fn another_function() {
    println!("Another function");
}


// função com parametros
fn another_function_value(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}");
}

// statements and expressions
fn statements(){
    // statements -> executam algo
    let _x = 6;

    // expressions -> retornam algo
    let y = {
        let x: i32 = 3;
        x + 1
    };
    println!("The value of y is {y}");
}

// função com retorno
fn five() -> i32 {
    5
}

fn call_five() {
    let x = five();
    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn call_plus_one(){
    let x = plus_one(5);
    println!("The value of x is {x}");
}
