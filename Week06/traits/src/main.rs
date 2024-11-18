fn main() {
    debug_trait_example();
    println!();
    my_impl_for_debug_trait();
    println!();
}

// ЧЕСТО ИЗПОЛЗВАНИ ТИПАЖИ

// Стандартната библиотека дефинира често използвани типажи
// Голяма част от Rust екосистемата разчита на тях
// Само ние можем да имплементираме стандартните trait-ове за наши типове
// Ако пишем библиотека - добре е да имплементираме всички стандартни trait-ове, които можем

// СПИСЪК
// -> Copy
// -> Clone
// -> Eq
// -> PartialEq
// -> Ord
// -> PartialOrd
// -> Hash
// -> Debug
// -> Default

// CLONE

trait Clone {
    fn clone(&self) -> Self;

    fn clone_from(&mut self, souce: &Self) { // ...
    }
}

// Създава копие на обекта
// Позволява да си дефинираме собствена логика за копирането
// Поддържа #[derive(Clone)], ако всички полета имплементират Clone !!!
// Имплементацията от derive извиква clone на всички полета рекурсивно !!!
// Рядко ще се налага да направим ръчна имплементация на Clone, защото не работим с гола памет !!!

// COPY

trait Copy: Clone {}

// Marker trait
// Показва, че стойността може да се копира чрез копиране на паметта байт по бай т.е. memcopy
// Променя се семантиката за присвояване на стойност от преместване (move) на копиране (copy)
// Изисква Clone да е имплентиран за съответния тип !!!
// Може да се добави с #[derive(Copy)]
// Или като цяло с Clone - #[derive(Copy, Clone)], поредността няма значение
// Можем да имплементираме Copy само ако:
// -> Всички полета са Copy
// -> Типа няма дефиниран дестуктор (т.е. не е Drop)

// DROP

trait Drop {
    fn drop(&mut self);
}

// Позволява да дефинираме деструктури
// Методът се изивква автоматично, когато обекта излезе от Scope
// Не може да се изивква ръчно
// Вика се drop на всяко поле рекурсивно, ако имплементира Drop
// Можем да използваме std::mem::drop за да "накараме" drop-ване

// Имплементация на std::mem::drop
fn drop<T>(_x: T) {}

// DEFAULT

trait Default {
    fn default() -> Self;
}

// Позволява създаване на обект със стоност по подразбиране
// Може да се добави с #[derive(Default)], ако всички полета имплементират Default
// Default или fn new() -> Self?
// -> може и двете
// -> Default е интерфейс - позволява използването на типа в generic код
// -> new е по-очаквано от програмиста

// HASH

// Използва се от типове и функции, които позволяват хеширане
// Например ключовете HashMap и HashSet
// Може да се добави с #[derive(Hash)], ако всички полета имплементират Hash

// DISPLAY

// Изполва се от placeholder-a {}, за форматиране на стойност, която ще се показва на потребителя
// Не може да се derive-не за разлика от Debug

use std::fmt::{self, Display, Formatter};

struct MagicTrick {
    description: String,
    secrets: Vec<String>,
    skills: Vec<String>,
}

impl Display for MagicTrick {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Магически трик {:?}", self.description)
    }
}

// Макрос write
// write!(f, "Магически трик {:?}", self.description)
// Подобно на print! и format!
// Записва форматиран текст в структура, която имплементира std::fmt::Write или std::io::Write
// разгръща се до
// f.write_fmt(format_args!("Магически трик {:?}", self.description))
// всички подобни макроси (print!, write!, format!) се свеждат до format_args!
// format_args! е вградено в компилатора и не се разгръща до Rust код.

// DEBUG

// placeholder {:?}
// форматиране на стойност, която ще показваме само с цел дебъгване
// #[derive(Debug)] имплементира версия по подразбиране

#[derive(Debug)]
struct MagicTrick02 {
    description: String,
    secrets: Vec<String>,
    skills: Vec<String>,
}

fn debug_trait_example() {
    let trick = MagicTrick02 {
        description: String::from("Изчезваща монета"),
        secrets: vec![String::from("Монетата се прибира в ръкава")],
        skills: vec![String::from("Бързи ръце"), String::from("Заблуда")],
    };

    println!("{:?}", trick);
}

// Може да напишем и собствена имплементация
// std::fmt::Formatter съдържа набор от полезни функции - pad, precision, width и други
// use std::fmt::{Debug};
//
// impl Debug for MagicTrick02 {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         f.debug_struct("Трик")
//             .field("описание", &self.description)
//             .field("тайни", &self.secrets)
//             .field("умения", &self.skills)
//             .finish()
//     }
// }

// fn my_impl_for_debug_trait() {
//     let trick = MagicTrick02 {
//         description: String::from("Изчезваща монета"),
//         secrets: vec![String::from("Монетата се прибира в ръкава")],
//         skills: vec![String::from("Бързи ръце"), String::from("Заблуда")]
//     };
//
//     println!("{:?}", trick)
// }

// ПРЕДИФИНАРНЕ НА ОПЕРАТОРИ

// Операторите се дефинират с trait-ове
// Видяхме, trait-a Add, с който дефинираме +

trait Add<RHS = Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}

// Списък на възможните оператори за предифиране
// -> Add, Sub, Mul, Div, Rem
// -> BitAnd, BirOr, BitXor, Shl, Shr
// -> *Assign (AddAssign, SubAssign, и т.н)
// -> Neg, Not
// -> Index
// -> IndexMut

// Предефиниране на PartialEq

trait PartialEq<Rhs = Self>
where
    Rhs: ?Sized,
{
    fn eq(&self, other: &Rhs) -> bool;
    fn ne(&self, other: &Rhs) -> bool;
}

// Дефинира операторите == и !=
// Не е задължително a == a да върне true

// Предефиниране на Eq

trait Eq: PartialEq<Self> {}
// Marker trait
// Задължава a == a да е true
// trait Eq : PartialEq<Self> означава, че Eq е subtrait на PartialEq<Self>.
// За да имплементираме Eq за T, трябва да имаме имплементация на PartialEq<T> за T


// Предифиниране на PartialOrd
trait PartialOrd<Rhs = Self>: PartialEq<Rhs>
where Rhs: ?Sized
{
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;

    fn lt(&self, other: &Rhs) -> bool;
    fn le(&self, other: &Rhs) -> bool;
    fn gt(&self, other: &Rhs) -> bool;
    fn ge(&self, other: &Rhs) -> bool;
}

// Дефинира операторите < <= > >=
// PartialOrd дефинира частична наредба

enum Ordering {
    Less,
    Equal,
    Greater
}


// Предифиниране на ORD

trait Ord: Eq + PartialOrd<Self> {
    fn cmp(&self, other: &Self) -> Ordering;

    fn max(self, other: Self) -> Self;
    fn min(self, other: Self) -> Self;
}

// Дефинира тотална наредба, т.е. само от a < b, a == b, a > b, е изпълнено