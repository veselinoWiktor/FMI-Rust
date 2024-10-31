use std::str::FromStr;

fn main() {
    let x = i32::from_str("-13");
    let y = u8::from_str("323");
    let z = f32::from_str("5e-3");

    println!("{:?}\n{:?}\n{:?}", x, y, z);
    println!();

    // reciprocal to from_str and FromStr

    let x = "-13".parse::<i32>();
    let y = "323".parse::<u8>();
    let z = "5e-3".parse::<f32>();
    // Типа тук при parse е задължителен

    println!("{:?}\n{:?}\n{:?}", x, y, z);
    println!();

    let x: Result<i32, <i32 as FromStr>::Err> = "-13".parse();
    let y: Result<u8, <u8 as FromStr>::Err> = "323".parse();
    let z: Result<f32, <f32 as FromStr>::Err> = "5e-3".parse();
    // Тук можем да изтървем типа

    println!("{:?}\n{:?}\n{:?}", x, y, z);
    println!();

    let x: Result<i32, _> = "-13".parse();
    let y: Result<u8, _> = "323".parse();
    let z: Result<f32, _> = "5e-3".parse();
    // Тук можем да изтървем типа и типа на грешката

    println!("{:?}\n{:?}\n{:?}", x, y, z);
    println!();

    let x: i32 = "-13".parse().unwrap();
    // let y: u8 = "323".parse().unwrap();
    let z: f32 = "5e-3".parse().unwrap();
    // Тук можем да премахнем и Result

    println!("{:?}\n???\n{:?}", x, z);
    println!();

    parse_student();
    println!();
}

// FromStr за наши структури

#[derive(Debug)]
struct Student {
    name: String,
    faculty_number: String,
}

impl FromStr for Student {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(", ").collect::<Vec<_>>().as_slice() {
            [name, faculty_number] => {
                let name = name.to_string();
                let faculty_number = faculty_number.to_string();
                Ok(Self {
                    name,
                    faculty_number,
                })
            }
            _ => Err(String::from("🤷🤷🤷")),
        }
    }
}

fn parse_student() {
    let s1: Result<Student, _> = "Данчо Е. Студент, 12345".parse();
    let s2: Result<Student, _> = "Гинка Билоба, 77777".parse();
    let s3: Result<Student, _> = "Бял Мерцедес".parse();

    println!("{:?}\n{:?}\n{:?}", s1, s2, s3);
}
