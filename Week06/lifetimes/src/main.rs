use std::collections::HashMap;

// Искаме да напишем следната функция
// Функцията map_get връща референция
// Компилатора иска да знае колко дълго е валидна тази референция
// Но в сигнатурата на функцията няма достатъчно информация
// fn map_get(map: &HashMap<String, String>, key: &str) -> &str {
//     todo!();
// }

// Lifetimes

// връща два резена от подадения низ
// резултата е обвързан с параметъра "text"
fn split_at(text: &str, index: usize) -> (&str, &str) {
    todo!();
}

// връща стойност от речника по зададен ключ.
// резултата е обвързан с параметъра `map`
fn map_get<'a>(map: &'a HashMap<String, String>, key: &str) -> &'a str {
    &"str"
}

// връща по-дългия от двата низа.
// резултата е обвързан и с `s1` и с `s2`
fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    s1
}

// 'a
// се нарича lifetime параметър или lifetime анотация
// специален вид generic параметър
// използва се да се означи колко дълго живее референция
// &`a X, &`b mut Y
// може да е всякакъв идентификатор, но практиката е да са кратки или еднобуквени
// &'my_lifetime X

fn foo<'a>(x: &'a str) {
    todo!();
    // отделно x: &'a str не ни дава информация
    // измислили сме си име a за периода за който живее референцията x
}

fn foo2<'a>(x: &'a str) -> &'a str { x }
// Заедно x: &'a str и -> &'a str задават ограничение
// резултатът живее колкото x

fn trim<'a>(s: &'a str) -> &'a str { s }

// Не е нужно всички lifetime-и да са еднакви
// резултатът е свързан с оригиналния низ `s`,  но не е свързан с `pattern`
fn first_occurances<'a, 'b>(s: &'a str, pattern: &'b str) -> Option<&'a str> {
    s.matches(pattern).next()
}

// Lifetimes elision
// Следните дефиниции са еквивалентни
fn trim2(s: &str) -> &str {
    todo!();
}

fn trim3<'a>(s: &'a str) -> &'a str { todo!() }

// КОГА ТРЯБВА ДА ПИШЕМ Lifetimes

// В блок код:
//      НИКОГА
//      Компилаторът винаги има всичката нужна информация да определи правилния Lifetime
// В дефиниция на функция
//      Понякога
//      Тук се прилага lifetime elision
// Структура
//      Винаги

// Как работи

// (1) За всеки пропуснат lifetime в аргументите се добавя нов lifetime параметър
fn print(s: &str) {}          // elided
fn print2<'a>(s: &'a str) {}  // expanded

fn foo3(x: (&u32, &u32), y: usize) {}               // elided
fn foo4<'a, 'b>(x: (&'a u32, &'b u32), y: usize) {} // expanded

// (2) Ако за аргументите има само един lifetime параметър (експлицитен или пропуснат), този lifetime се налага на всички пропуснати
// lifetimes в резултата

fn substr(s: &str, until: usize) -> &str { s }                              //elided
fn substr1<'a>(s: &'a str, until: usize) -> &'a str { s }                   //expanded

fn split_at1(s: &str, pos: usize) -> (&str, &str) { (s, s) }                // elided
fn split_at2<'a>(s: &'a str, pos: usize) -> (&'a str, &'a str) { (s, s) }   // expanded

// (3) Ако първият аргумент е &self или &mut self, неговия lifetime се налага на всички пропуснати lifetimes в резултата
// fn get_mut(&mut self) -> &mut T;                                 elided
// fn get_mut<'a>(&'a mut self) -> &'a mut T {}                     expanded

// fn args(&mut self, args: &[T]) -> &mut Self {};                  elided
// fn args<'a, 'b>(&'a mut self, args: &'b [T]) -> &'a mut Self {}  expanded

// Във всички останали случай е грешка да не напишем lifetime annotation

// Обобщение
// Всички референции имат lifetime параметър (&'a X)
// Еднакъв lifetime параметър - двете референции живеят еднакво дълго
// Lifetime elision

// СТАТИЧЕН ЖИВОТ
// let s: &'static str = "Низ литерал";
// Специален lifetime `static
// Референицията е валидна за целия живот на програмата
// Литерали, константи, статични променливи и др.
// По-голям от всеки друг lifetime 'a

// Референции към стурктури
#[derive(Debug)]
struct Words<'a>{
    text: Option<&'a str>,
}

/*
какъв да е типа на полето text?
не искаме String -- излишно копиране
искаме да е референция
 */

use core::option::Option;
use std::fmt;

impl<'a> Words<'a> {
    /*
    fn new(text: &str) -> Words {
        Words { text }
    }
    // Как се попълва lifetime анотациите за new

    fn new<'b>(text: &'b str) -> Words<'b> {
        Words { text }
    }
    aлгоритъмът не взима под внимание lifetime-а 'a
    пропуснати lifetime параметри на структури се попълват по същия начин като референциите
     */

    fn new(text: &'a str) -> Self {
        Words { text: Some(text) }
    }

    fn next_word(&mut self) -> Option<&'a str> {
        let text = self.text?;

        match text.split_once(char::is_whitespace) {
            Some((word, rest)) => {
                self.text = Some(rest);
                Some(word)
            }
            None => {
                self.text = None;
                Some(text)
            }
        }
    }

    // Anonymous lifetimes
    // Можем да използваме анонимен lifetime '_

    fn make_words_iter(text: &str) -> Words<'_> {
        Words::new(text)
    }

    fn print_next_word(words: &mut Words<'_>) {
        if let Some(word) = words.next_word() {
            println!("{}", word);
        }
    }

    // за удобство на четящия
    // показва, че структурата Words съдържа elided lifetime
}


// Имплементация на next_word метода
// Всичко работи, но дали имаме правилните lifetimes?
fn hello() -> &'static str {
    let mut words = Words::new("hello world");
    words.next_word().unwrap()
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn words_basic() {
        let mut words = Words::new("hello world");
        assert_eq!(words.next_word(), Some("hello"));
        assert_eq!(words.next_word(), Some("world"));
        assert_eq!(words.next_word(), None);
    }

    #[test]
    fn empty_string() {
        let mut words = Words::new("");
        assert_eq!(words.next_word(), Some(""));
        assert_eq!(words.next_word(), None);
    }

    #[test]
    fn trailing_whitespace() {
        let mut words = Words::new("hello world ");
        assert_eq!(words.next_word(), Some("hello"));
        assert_eq!(words.next_word(), Some("world"));
        assert_eq!(words.next_word(), Some(""));
        assert_eq!(words.next_word(), None);
    }
}

// LIFETIMES & GENERICS
use std::fmt::Display;

struct Pretty<T: Display>(T);

impl<T: Display> Display for Pretty<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "🌸🌹🌻 {} 🌻🌹🌸", self.0)
    }
}

fn prepare_pretty<T: Display>(something: T) -> Pretty<T> {
    Pretty(something)
}

fn main() {
    let mut map = HashMap::new();
    map.insert(String::from("key"), String::from("value"));

    let value = {
        let key = String::from("key");
        let value = map_get(&map, &key);
        value
    };

    println!("{:?}", value);

    // --------------------------------------
    // Функцията trim връща резултат който живее колкото аргумента
    {
        let s = String::from("   низ   \n"); // --+
        let trimmed = trim(&s);                  // --|-+
        //   | |
    } // <-------------------------------------------------+-+

    // -----------------------------------------
    //Функцията longer връща резултат който живее колкото общия период в който живеят двата ѝ аргумента.
    {
        // Ако двата аргумента живеят различно - ще се вземе по-малкия период
        // има автоматично конвертиране от по-голям до по-малък lifetime
        let s1 = String::from("дългият низ е дълъг"); // --+
        {                                                      //   |
            let s2 = String::from("къс низ"); // --+       |
            let result = longer(&s1, &s2);        // --|--+    |
                                                       //   |  |    |
            println!("{}", result);                    //   |  |    |
        } // <----------------------------------------------+--+    |
                                                       //           |
    } // <----------------------------------------------------------+

    {
        // Ако двата аргумента живеят различно - ще се вземе по-малкия период
        // има автоматично конвертиране от по-голям до по-малък lifetime
        // let s1 = String::from("дългият низ е дълъг"); -----------+
        // let result = {                                           |
        //     let s2 = String::from("къс низ"); // --------+       |
        //     longer(&s1, &s2) ----------------------------|--+    |
        //                                                  |  |    |
        // };// <-------------------------------------------+--+    |
        //                                                          |
        // println!("{}", result) ERROR                             |
    } // <----------------------------------------------------------+

    /*
    error[E0597]: `s2` does not live long enough
      --> src/bin/main_6924ede250fd1bb1a0f74dfd79d60b37d4ab5cb1.rs:8:21
           |
        6  |     let result = {                                //   |
           |         ------ borrow later stored here
        7  |         let s2 = String::from("къс низ"); // --+       |
           |             -- binding `s2` declared here
        8  |         longer(&s1, &s2)                  // --|--+    |
           |                     ^^^ borrowed value does not live long enough
        9  |                                           //   |  |    |
        10 |     }; // <------------------------------------+--+    |
           |     - `s2` dropped here while still borrowed

    For more information about this error, try `rustc --explain E0597`.
    error: could not compile `rust` (bin "main_6924ede250fd1bb1a0f74dfd79d60b37d4ab5cb1") due to 1 previous error
     */

    // ----------------------------------------------------------------------------------------------------
    {
        let text = String::from("обичам мач и боза");
        let result = {
            let pattern = String::from("боза");
            first_occurances(&text, &pattern)
        };

        println!("{:?}", result);
    }

    // Всяка референция има lifetime, който се следи от компилатора
    // Но не е задължително винаги да го анотираме
    // Когато ситуацията не е двусмислена моде да се пропусне
    // Това се нарича lifetime elision

    // ---------------------------------------------------------------------------------------------------------
    // РЕФЕРЕНЦИЯ В СТРУКТУРИ
    // Животът на структурата е ограничен до това колко живее обектът, от който сме взели референция
    {
        let t1 = Words::new("a b c"); // Words<'static>

        {
            let s = String::from("мой таен низ"); // ---+ - 'a
            Words::new(s.as_str());                        //    | - Words<'a>
        }; // <--------------------------------------------------+
    }
    // ----------------------------------------------------------------------------------------------------------
    println!("{}", hello());

    // ----------------------------------------------------------------------------------------------------------
    {
        // `String` имплементира `Display`
        let s: String = String::from("⚘⚘⚘");

        // Pretty<String>
        let pretty = prepare_pretty(s);

        println!("{}", pretty);
    }

    {
        // `&String` имплементира `Display`
        let s: String = String::from("⚘⚘⚘");

        // Pretty<&'a String>
        let pretty = prepare_pretty(&s);

        println!("{}", pretty);

        // generic тип T може да е референция
        //      Тогава той има lifetime, макар че видимо няма lifetime параметър
        //      И всеки тип който съдържа T също има lifetime, защото съдържа референция
        // generic тип T може и да е тип със собственост
        //      Тогава се приема, че има `static lifetime
        // Т = &'a u32       ⇒  T: 'a
        // T = &'static u32  ⇒  T: 'static
        // T = u32           ⇒  T: 'static
    }
}

