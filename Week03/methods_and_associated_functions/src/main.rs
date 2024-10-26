fn main() {
    let _user = User::new(String::from("Иванчо"), String::from("ivan40@abv.bg"));
    method_invokation();
}

// struct блока съдържа само полетата на структурата
struct User {
    username: String,
    email: String,
    sign_in_count: u64
}

// методи и функции се добавят в отделен impl блок
// разделение между данни и логика
impl User {
    fn new(username: String, email: String) -> User {
        User {
            username,
            email,
            sign_in_count: 0
        }
    }
}

// В Rust няма конструктури 
// Конвенцията е да има асоциирана функция, която да създава обект от типа
// Обикновено името е: new, from_*, with_*
// Но има и изключения напр: File::open();

// В Rust има деструктури
// Дефинират се чрез trait-a Drop

struct Rectangle { width:f64, height: f64 }

impl Rectangle {
    // Метод - функция, която приема като първи аргумент self, &self, &mut self (method reciever)
    // Полетата се достъпват през аргумента self
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// Позволено е делкарирането на повече от един impl блок. Удобно е при групиране на методи.
impl Rectangle {
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    } 
}

struct Rectangle2 { width: f64, height: f64 }

impl Rectangle2 {

    // Типа Self
    // Достъпен във вътрешността impl блок
    // Псевдним на типа, за който имплементираме
    fn new(width: f64, height: f64) -> Self {
        Self {width, height}
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }
    // е еквивалентно на
    // fn area(self: &Self) -> f64 {
    //     self.width * self.height
    // }
    // е еквивалентно на
    // fn area(self: &Rectangle) -> f64 {
    //     self.width * self.height;
    // }
}

fn method_invokation() {
    let rect = Rectangle2::new(2.0, 3.0);
    let area = rect.area(); // Могат да се извикват със синтаксиса за методи
    let area = Rectangle2::area(&rect); // могат да се извикват и като асоциирани функции

    println!("area = {}", area);
    // както полетата, методите се достъпват с .
    // компилаторът автоматично добавя *, & или &mut, така че типа на аргумента да съвпадне с типа на method reciever-а 
}