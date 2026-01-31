fn main() {
    let width = 30;
    let height = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );

    // refatorando usando tuplas
    let rect = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_tupla(rect) 
    );

    // refatorando com struct
    let rect_struct = Rectangle {
        width: 30,
        height:  50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect_struct) 
    );

    println!("rectangle is {rect_struct:?}");
    // um pouco mais facil de ler
    println!("rectangle is {rect_struct:#?}");

    // mostra o debug de uma propriedade
    let scale = 2;
    let rectangle = Rectangle{
        width: dbg!(30 * scale),
        height:50,
    };

    dbg!(&rectangle);

}

fn area(width: u32, height: u32) -> u32{
    width * height
}

fn area_tupla(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}