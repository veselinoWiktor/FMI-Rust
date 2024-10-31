use std::fmt::Debug;

fn main() {
    let arr = vec![Some(1.1), Some(2.2), None].to_json();
    println!("Vector as json: {}", arr);
    trait_objects_exmp();
    trait_objects_exmp02();
}

fn to_json<T: ToJson>(value: T) -> String {
    value.to_json()
}

trait ToJson {
    fn to_json(&self) -> String {
        String::from("null")
    }
}

impl ToJson for String {
    fn to_json(&self) -> String {
        format!("\"{}\"", self)
    }
}

impl ToJson for i32 {
    fn to_json(&self) -> String {
        format!("{}", self)
    }
}

impl ToJson for f32 {
    fn to_json(&self) -> String {
        format!("{}", self)
    }
}

impl<T: ToJson> ToJson for Option<T>{
    fn to_json(&self) -> String {
        match self {
            Some(val) => val.to_json(),
            None => String::from("null")
        }
    }
}

impl<T> ToJson for Vec<T> where T: ToJson {
    fn to_json(&self) -> String {
        let mut iter = self.iter();

        let mut result = match iter.next() {
            Some(first) => first.to_json(),
            None => String::new()
        };

        for e in iter {
            result. push_str(", ");
            result.push_str(&e.to_json());
        }

        format!("[{}]", result)
    }
}

struct Student {
    age: i32,
    full_name: String,
    number: i32,
    hobby: Option<String>
}

impl ToJson for Student {
    fn to_json(&self) -> String {
        format!(
            r#"{{
                "age": {},
                "full_name": {},
                "number": {},
                "hobby": {}
            }}"#,
            self.age.to_json(), self.full_name.to_json(),
            self.number.to_json(), self.hobby.to_json()
        )
    }
}

// Множество типажи
fn _log_json_transformation<T>(value: T) 
where 
    T: ToJson,
    T: Debug
{
    println!("{:?} -> {}", value, value.to_json())
}

// Кога можем да имплементираме типаж?
// Orphan rule: можем да имплементираме trait T за тип S ако:
//      1) trait-a T е дефиниран в нашия crate
//      2) тиша S е дефиниран в нашия crate

// Static Dispatch
/*
    Компилатора генерира отделната версия на to_json за всяка нейна имплементация
        *to_json::<String>
        *to_json::<i32>
        *to_json::<Student>
    При компилиране се избира правилният вариант на функцията за дадения случай
    Това се нарича мономорфизъм
*/

// Trait Objects
// Dynamic Dispatch

fn _to_json(value: &dyn ToJson) -> String {
    value.to_json()
}

// една версия на функцията - to_json вече не е generic
// подава се &dyn ToJson - нещо, което имплементира trait ToJson
// коя точно имплементация се използва се решава по време на изпълнение

/*
&dyn trait се нарича trait object
    дебел указател (fat pointer), съдържа:
    указател към обект
    указател към виртуална таблица за съответния trait
това още се нарича type erasure
    забравяме информация какъв е конкретния тип на обекта
    Помним само какви интерфейси имплементира
*/

fn trait_objects_exmp() {
    let trait_object: &dyn ToJson = &5;

    println!("{}", _to_json(trait_object));
    println!("{}", _to_json(&5));
    println!("{}", _to_json(&5 as &dyn ToJson));
}


// Можем да използваме trait objects да си направим не-хомогенен вектор.
impl ToJson for Box<dyn ToJson> {
    fn to_json(&self) -> String {
        (**self).to_json()
    }
}

fn trait_objects_exmp02() {
    let values = vec![
        Box::new(1.1_f32) as Box<dyn ToJson>,
        Box::new(3_i32),
        Box::new(String::from("stuff"))
    ];

    println!("{}", _to_json(&values));
}

/*
    Object Safety!!!

Не за всеки trait може да се построи trait object
Тези, от които може, се наричат object safe
С две думи - един trait e object safe, ако може да се построи виртуална таблица за него
*/

// Примери за неща, които правят trait-а не object-safe

trait NotObjectSafe {
    type Item;

    // шаблонни функции
    fn generic<T>(&self, val: &T);

    // Функции, които приемат аргумент от тип self или връщат Self
    fn reciever_by_value(self);
    fn self_argument(&self, other: Self);
    fn duplicate(&self) -> Self;

    // Функции, които приемат или връщат асоцииран тип
    fn get_item(&self) -> &Self::Item;
    fn set_item(&mut self, item: &Self::Item);

    // и други...
}

// Хитрина - можем да направим trait object-safe, ако сложим ограничение where Self: Sized на проблематичните функции
trait ObjectSafe {
    fn to_json(&self) -> String;
    fn to_bytes(self) -> Vec<u8> where Self: Sized;
}