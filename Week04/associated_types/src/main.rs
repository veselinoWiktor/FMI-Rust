fn main() {
    println!("Hello, world!");
}

trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for std::str::Chars<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl Iterator for std::str::Bytes<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

// Асоциирани типове и шаблонни типажи
trait Add<RHS = Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}

impl Add for i32 {
    type Output = i32;
    fn add(self, rhs: i32) -> i32 {
        self + rhs
    }
}

impl Add for String {
    type Output = String;

    fn add(self, rhs: String) -> String {
        format!("{} {}", self, rhs)
    }
}

struct Student;
struct StudentGroup {
    members: Vec<Student>,
}

impl Add for Student {
    type Output = StudentGroup;

    fn add(self, rhs: Student) -> StudentGroup {
        StudentGroup {
            members: vec![self, rhs],
        }
    }
}
