fn main() {
    println!("Hello, world!");
}

struct Color(f32, f32, f32);
struct Point(f32, f32, f32);

fn struct_tuples() {
    let black = Color(0.0, 0.0, 0.0);
    let origin = Color(0.0, 0.0, 0.0);
}

fn struct_tuples_access() {
    let black = Color(0.0, 0.0, 0.0);

    println!("r: {}, g: {}, b: {}", black.0, black.1, black.2);
}

// Tuple struct с едно поле често се използва от typesafe wrapper
// Това се нарича newtype struct или newtype wrapper
#[derive(Debug, Clone, Copy)]
struct Token(u32);

struct Electron {}
struct Proton;

fn empty_structs() {
    // Възможна е декларацията на празни структури. Могат да се използват като маркери - големина им е 0 байта
    let x = Electron {};
    let y = Proton;
}
