use::std::mem;

fn main() {
    let myCh: &str = "Abb📝";

    println!("{:?}", myCh.chars());
    println!("{}", mem::size_of_val(myCh));
    println!("{}", myCh.);
}
