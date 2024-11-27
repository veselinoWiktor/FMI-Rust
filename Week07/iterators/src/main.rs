use std::cell::Cell;
use std::collections::{BTreeMap, HashMap};
use std::iter::Iterator;

fn main() {
    // Итераторите може да са крайни
    // Итератори по колекция
    for word in &["foo", "bar", "baz"] {
        println!("{}", word);
    }

    // Итераторите може да са безкрайни
    // Поток от стойности
    // for n in 1.. {
    //     println! {"{n} mississippi"}
    // }

    // Разгъване на for-цикъл
    // for loop_variable in iterator {
    //     code();
    // }

    // {
    //     let mut iter = IntoIterator::into_iter(iterator);
    //     loop {
    //         match iter.next() {
    //             None => break,
    //             Some(loop_variable) => { code(); }
    //         }
    //     }
    // }

    map_example();
    println!();
    map_example02();
    println!();
    filter_example();
    println!();
}

// pub trait Iterator {
//     type Item;
//
//     // Required method
//     fn new(&mut self) -> Option<Self::Item>;
//
//     // Provided methods
//     // ...75 methods!!!
// }
//
// pub trait IntoIterator {
//     type Item;
//     type IntoIter: Iterator<Item = Self::Item>;
//
//     fn into_iter(self) -> Self::IntoIter;
// }

//use std::alloc::Allocator

// Вектора може да се превърне в итератор.
// Това консумира вектора - итератора връща един по един елементите по стойност
// impl<T, A: Allocator> IntoIterator for Vec<T, A> {
//     type Item = T;
//     type IntoIter = std::vec::IntoIter<T, A>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         todo!()
//     }
// }

/*
Еквивалентно е дали ще итерираме по vec или по vec.into_iter()
 */
fn vec_iter_example() {
    let vec = vec![1, 2, 3];

    for val in &vec {
        todo!();
    }
    // --> for val in IntoIterator::into_iter(vec);


    // let mut v_iter = vec.into_iter();
    // for val in v_iter {
    //     todo!();
    // }
    // --> for val in IntoIterator::into_iter(vec.into_iter());
    // --> for val in IntoIterator::into_iter(<Vec<_> as IntoIterator>::into_iter(vec))
}

/*
    АДАПТЕРИ ЗА ИТЕРАТОР

Iterator trait-a изисква да имплементираме една единствена функция - next;
Но предоставя голяма количество функции наготово
Повечето са адаптери - консумират итератора и връщат нов итератор, който връща променен поток от елементи
 */


/*
    MAP

Трансформира всяка стойност от итератора, използвайки функция T -> U
*/

fn map_example() {
    for num in [1, 2, 3].iter().map(|&x| x * x) {
        println!("{}", num);
    }
}

/*
Функцията map връща структура std::iter::Map, която си запазва оригиналния итератор и функцията.
 */

fn map_example02() {
    let nums = &[1, 2, 3];

    let doubled = nums.iter().map(|&x| x * x);

    println!("{:?}", doubled)
}
// Итераторите са мързеливи - извикването на map не изпълнява нищо, само създава структура.


//Closure-а ще се извика чак когато започнем да обхождаме итератора
fn map_example03() {
    let nums = &[1, 2, 3];

    let doubled = nums.iter().map(|&x| {
        println!("closure called");
        x * x
    });

    for num in doubled {
        println!("{}", num);
    }
}

/*
При компилиране с оптимизации структурите могат да се премахнат и извикванията на функцията да се inline-нат
Често генерираното assembly за код с итератори и код с цикли е едно и също
 */

fn map_example04() {
    let iter1 = [1, 2, 3].iter();
    let func = |&x| x * x;

    let iter2 = iter1.clone().map(func); // clone за да се компилира
    for e in iter2 {
        // todo!();
    }

    // Работи все едно е написано ... :

    for e1 in iter1 {
        let e2 = func(e1);
        //todo!()
    }
}

/*
    FILTER
 */

fn filter_example() {
    let nums = &[1, 2, 3, 4, 5];

    for e in nums.iter().filter(|&x| x % 2 != 0) {
        println!("{e}");
    }
}

fn filter_plus_map_example() {
    let nums = &[1, 2, 3, 4, 5];

    for e in nums
        .iter()
        .filter(|&x| x % 2 != 0)
        .map(|&x| x * x) {
        println!("{e}");
    }
}

fn filter_plus_map_example02() {
    let nums = &["12", "-23", "foo", "4", "bar"];

    for e in nums
        .iter()
        .map(|s| s.parse::<i32>())
        .filter(|n| n.is_ok())
        .map(|n| n.unwrap()) {
        println!("{e}")
    }
}

/*
    FILTER_MAP
 */

fn filter_map_example() {
    let nums = &["12", "-23", "foo", "4", "bar"];

    for e in nums
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
    {
        println!("{e}")
    }
}

//  FLATTEN

// Ако имаме итератор, който връща итератори, flatten премахва едно ниво на вложеност

fn flatten_example() {
    let capitals_map = BTreeMap::from([
        ('A', vec!["Athens", "Amsterdam"]),
        ('B', vec!["Brussels", "Berlin", "Budapest"])
    ]);

    for city in capitals_map
        .iter()
        .map(|(_, city_list)| city_list.iter())
        .flatten()
    {
        println!("{city}");
    }
}

//  FLAT_MAP

// map + flatten

fn flat_map_example() {
    let capitals_map = BTreeMap::from([
        ('A', vec!["Athens", "Amsterdam"]),
        ('B', vec!["Brussels", "Berlin", "Budapest"])
    ]);

    for city in capitals_map
        .iter()
        .flat_map(|(_, city_list)| city_list.iter())
    {
        println!("{city}");
    }
}

// приема и функции които връщат I: IntoIterator

fn flat_map_example02() {
    let capitals_map = BTreeMap::from([
        ('A', vec!["Athens", "Amsterdam"]),
        ('B', vec!["Brussels", "Berlin", "Budapest"])
    ]);

    for city in capitals_map
        .iter()
        .flat_map(|(_, city_list)| city_list.iter())
    {
        println!("{city}");
    }
}

// TAKE & SKIP

// take(n) - връща първите N елемента от итератора, след това None
// skip(n) - пропуска първите N елемента от итератора
// take_while(fn) - връща елементи, докато подадената функция връща true
// skip_while(fn) - пропуска елементи, докато подадената функция връща true
// полезни при безкрайни итератори

// CLONED & COPIED

// Iterator<Item = &T> -> Iterator<Item = T>
// iter.cloned() = iter.map(|x| x.clone())
// iter.copied() = iter.map(|&x| x), но трябва T: Copy
// основно се ползват с итератори по примитивни типове
// имаме итератор по &число, трябва ни итератор по число

fn clone_copied_example() {
    let nums = &[1, 2, 3, 4, 5];

    let odd_nums = nums.iter()
        .filter(|&x| x % 2 != 0)
        .copied()
        .collect::<Vec<i32>>();
}

//  ZIP

// Обхожда два итератора едновременно
fn zip_example() {
    let values_a = &["foo", "bar", "baz"];
    let values_b = &[1.2, 3.4, 5.6];

    for (a, b) in values_a.iter().zip(values_b.iter())
    {
        println!("{} => {}", a, b);
    }
}

// За повече симтеричност има свободна функция std::iter::zip

fn zip_example02() {
    let values_a = &["foo", "bar", "baz"];
    let values_b = &[1.2, 3.4, 5.6];

    for (a, b) in std::iter::zip(values_a, values_b)
    {
        println!("{} => {}", a, b);
    }
}

// ако двата итератора имат различна дължина - спира, като се изчерпи по-краткия
// можем да комбинираме с безкраен интервал, за да итерираме по елемент с индекс

fn zip_example03() {
    let arr = &["foo", "bar", "baz"];

    for (i, word) in (0..).zip(arr) {
        println!("arr[{}] = {}", i, word);
    }
}

//  ENUMERATE

// Тъй като последния пример с zip се използва често има отделна функция enumerate

fn enumerate_example() {
    let arr = &["foo", "bar", "baz"];

    for (i, word) in arr.iter().enumerate() {
        println!("arr[{}] = {}", i, word);
    }
}

/*
find(pred) - връща Some(elem) с първия елемент, който отговаря на предиката, или None
all(pred) - връща true ако всички елементи отговарят на предиката, иначе false
any(pred) - връща true ако съществува елемент, отговарящ на предиката, иначе false
 */

//  FOLD

/*
fold(init, fn) - събира всички елементи на итератора в една стойност, използвайки подадената функция
приема функция (B, T) -> B
    първият аргумент е акумулираната стойност до момента
    вторият аргумент е елемент от итератора
    операцията също се нарича reduce или accumulate
 */

fn fold_example() {
    let nums = &[1, 2, 3, 4, 5];

    let sum = nums.iter().fold(0, |acc, x| acc + x);
    println!("{}", sum);
}

fn fold_example02() {
    let nums = &[1, 2, 3, 4, 5];

    let expr = nums.iter().fold(String::from('X'), |acc, x| {
        format!("({acc} + {x}")
    });
    println!("{}", expr)
}

/*
sum - fold, използвайки оператор +
product - fold, използвайки оператор *
 */

fn sum_example() {
    let nums = &[1, 2, 3, 4, 5];

    let sum = nums.iter().sum::<i32>();

    println!("{}", sum)
}

//  COLLECT

// fold, който събира елементите в дадена колекция
// generic по типа на резултата

fn collect_example() {
    let arr = &[1, 2, 3];

    let vec = arr.iter().collect::<Vec<&i32>>();

    println!("{:?}", vec);
}

/*
Типа на резултата трябва да имплементира FromIterator

pub trait Iterator {
    /* ... */

    fn collect<B>(self) -> B
    where
        B: FromIterator<Self::Item>,
        Self: Sized,
    { /* ... */ }
}

pub trait FromIterator<T>: Sized {
    fn from_iter<I>(iter: I) -> Self
       where I: IntoIterator<Item = T>;
}

подобно на str::parse и FromStr
 */

// Някои полезни имплементации:

fn collect_example02() {
    // Iterator<Item = T> => Vec<T>

    let v: Vec<_> = vec![1, 2, 3]
        .into_iter()
        .collect();

    println!("{:?}", v);
}

fn collect_example03() {
    // Iterator<Item = char> => String

    let s: String = vec!['a', 'b', 'c']
        .into_iter()
        .collect();

    println!("{:?}", s)
}

fn collect_example04() {
    // Iterator<Item = (K, V)> => HashMap<K, V>

    let map: HashMap<_, _> = vec![(String::from("foo"), 1), (String::from("bar"), 2)]
        .into_iter()
        .collect();

    println!("{:?}", map);
}

fn collect_example05() {
    // Iterator<Item = Result<T, E>> => Result<Vec<T>, E>

    let res: Result<Vec<_>, &str> = vec![Ok(1), Ok(2), Ok(3)]
        .into_iter()
        .collect();

    println!("{:?}", res);
}

fn collect_example06() {
    // Iterator<Item = Result<T, E>> => Result<Vec<T>, E>

    let res: Result<Vec<_>, _> = vec![Ok(1), Err("not two"), Ok(3)]
        .into_iter()
        .collect();

    println!("{:?}", res);
}

// EXAMPLE

struct Grid {

}

impl Grid {
    // Get the cells that are neighbours to the cell at coords (x, y)

    fn neighbours(&self, x:i32, y:i32) -> impl Iterator<Item =  &Cell> {
        let coords = &[(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

        coords.iter()
            .copied()
            .map(move |offset_x, offset_y| { (x + offset_x, y + offset_y) })
            .filter(move |&coords| self.is_inside(coords))
            .map(move |coord| self.cell_at(coord))
    }
}
