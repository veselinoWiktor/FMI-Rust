// Fearless concurrency

// * Rust предотвратява data races
//      - 2 нишки достъпват една и съща памет
//      - поне единия достъп е за писане
//      - достъпите не са синхронизирани
// * Rust закодира в типовата си система понятието за thread safety
//      - кои обекти или операции могат да се използват безопасно в паралелен код
//      - компилационна грешка при нарушаване
// * Rust не може да предотварти логически бъгове - race conditions, deadlocks и др.
//      - но добри абстракции помагат с това

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::mpsc;
use std::{mem, thread};
use std::sync::mpsc::{SendError, TrySendError};
// Нишки
//  * thread::spawn() пуска нова нишка на ОС
//  * подадената функция се изпълнява в новата нишка
//  * когато функцията завърши, нишката се спира

fn first_thread_spawn() {
    // Ако искаме да изчакаме, трябва да си запазим JoinHandle
    // join() блокира, докато никата не приключи
    // spawn връща JoinHandle
    // можем да използваме join за да изчакаме пуснатите нишки
    // Когато JoinHandle се drop-не нишката се detach-ва
    let handle = thread::spawn(|| {
        // това няма да се принтира, защото програмата
        // ще завърши преди втората нишка да е започнала
        // останалите нишки се убиват   `
        println!("hi from spawned thread")
    });
    println!("hi from main thread");
    let _ = handle.join();
}

fn second_thread_spawn() {
    let handle = thread::spawn(||
        {
            // very hard computation ...
            42
        });

    let answ = handle.join();
    println!("The answer is {:?}", answ);
}

// panic! в нишка unwind-ва стека и убива нишката
// ако това е главната нишка, panic! убива програмата (и всички други нишки)
// ако не е главната нишка - връща се грешка от join()
// паниката може да се продължи с join().unwrap()
fn panic_in_thread() {
    let handle = thread::spawn(|| {
        panic!("too hard computation...");
    });

    let answ = handle.join();
    println!("The answer is {:?}", answ);
}

/// Споделяне на стойности
/// Нека искаме да достъпим една и съща стойност от няколко нишки
/// Тривиалният подход...
/// ```
/// fn trivial_access_to_value() {
///     let nums = vec![1, 2, 3, 4];
///
///     let handle = thread::spawn(|| {
///        for i in &nums {
///             println!("number {}", i);
///         }
///     });
///
///     let _ = handle.join();
/// }
/// ```
/// ```
/// error[E0373]: closure may outlive the current function, but it borrows `nums`, which is owned by the current function
///   --> src/main.rs:66:32
///    |
/// 66 |     let handle = thread::spawn(|| {
///    |                                ^^ may outlive borrowed value `nums`
/// 67 |         for i in &nums {
///    |                   ---- `nums` is borrowed here
///    |
/// ```
/// новосъздадената нишка може да надживее функцията в която е извикана
/// затова `Rust` не позволява да подадем референция към локални променливи
/// това се налага от ограничението на spawn, която приема F: `static
/// ```
/// pub fn spawn<F, T>(f: F) -> JoinHandle<T> where
///     F: FnOnce() -> T + Send + 'static,
///     T: Send + 'static
/// ```
fn access_to_values() {
    let _ = thread::spawn(thread1).join();
    println!("thread 1 exited");

    thread::sleep(std::time::Duration::from_millis(100));
}

fn thread1() {
    println!("thread1 started");

    thread::spawn(|| {
        println!("thread2 started");
        thread::sleep(std::time::Duration::from_millis(1));
        println!("thread2 will exit.");
    });

    println!("thread1 will exit.");
}

/// Ако използваме стойността само от новата нишка,
/// можем да я преместим с `move` closure
fn moves_access_values() {
    let nums = vec![0, 1, 2, 3];

    let handle = thread::spawn(move || {
        for i in &nums {
            println!("number {i}");
        }
    });

    let _ = handle.join();
}

/// Но това не би работило ако имаме повече от една нишка
///```
///fn move_access_values_in_two_threads() {
///    let nums = vec![0, 1, 2, 3];
///
///    let mut handles = vec![];
///    for _ in 0..2 {
///        handles.push(thread::spawn(move || {
///            for i in &nums {
///                println!("number {}", i);
///            }
///        }));
///    }
///
///    for h in handles {
///        let _ = h.join();
///    }
///}
/// ```
/// ```
///error[E0382]: use of moved value: `nums`
///    --> src/main.rs:136:36
///     |
/// 132 |     let nums = vec![0, 1, 2, 3];
///     |         ---- move occurs because `nums` has type `Vec<i32>`, which does not implement the `Copy` trait
/// ...
/// 135 |     for _ in 0..2 {
///     |     ------------- inside of this loop
/// 136 |         handles.push(thread::spawn(move || {
///     |                                    ^^^^^^^ value moved into closure here, in previous iteration of loop
/// 137 |             for i in &nums {
///     |                       ---- use occurs due to use in closure
/// ```

/// # Scoped threads
/// Един вариант е да използваме scoped threads API-то
fn use_scoped_threads() {
    let nums = vec![0, 1, 2, 3];

    thread::scope(|s /*:thread::Scope<'_, '_> */| {
        // тази функция се изпълнява в същата нишка

        for _ in 0..2 {
            // Scope::spawn създава нова нишка
            // Новата нишка може да дръжи референция към локални променливи
            s.spawn(|| {
                for i in &nums {
                    println!("number {}", i);
                }
            });
        }
    });

    // на края на функцията всички нишки създадени чрез Scope::spawn
    // се join-ват.
}

/// # Споделяне на стойности
///
/// Дръг вариант е да използваме нещо, което:
///     * притежава стойността, за да покрием ограничението F: 'static
///     * позволява споделяне на стойността
///
/// ```
/// fn access_to_value() {
///     // TODO: какво да добавим тук?
///     let nums = vec![1, 2, 3, 4];
///
///     let handle = thread::spawn(|| {
///         for i in &nums {
///             println!("number {}", i);
///         }
///     });
///
///     let _ = handle.join();
/// }
/// ```
/// # Споделяне на стойности - Rc
/// - `Rc` позволява 'споделена собственост' (shared ownership)
/// - Това да ли ще проработи?
/// ```
/// fn access_to_value() {
///     let nums_vec = vec![0, 1, 2, 3];
///     let nums_rc = Rc::new(nums_vec);
///
///     let mut handles = vec![];
///     for _ in 0..2 {
///         let nums_rc = Rc::clone(&nums_rc);
///         handles.push(thread::spawn(move || {
///             for i in &*nums_rc {
///                 println!("number {}", i);
///             }
///         }));
///     }
///
///     for h in handles {
///         let _ = h.join();
///     }
/// }
/// ```
/// - Не
/// ```
/// error[E0277]: `Rc<Vec<i32>>` cannot be sent between threads safely
///   --> src/bin/main_1c4b38ddc008640338e13791daaf3eccafcab0dc.rs:11:36
///    |
/// 11 |           handles.push(thread::spawn(move || {
///    |                        ------------- ^------
///    |                        |             |
///    |  ______________________|_____________within this `{closure@src/bin/main_1c4b38ddc008640338e13791daaf3eccafcab0dc.rs:11:36: 11:43}`
///    | |                      |
///    | |                      required by a bound introduced by this call
/// 12 | |             for i in &*nums_rc {
/// 13 | |                 println!("number {}", i);
/// 14 | |             }
/// 15 | |         }));
///    | |_________^ `Rc<Vec<i32>>` cannot be sent between threads safely
/// ```
/// # Споделяне на стойност - Arc
/// - Трябва да използваме Arc
fn access_to_values_with_arc() {
    let nums_vec = vec![0, 1, 2, 3];
    let nums_arc = Arc::new(nums_vec);

    let mut handles = vec![];
    for _ in 0..2 {
        let nums_arc = Arc::clone(&nums_arc);
        handles.push(thread::spawn(move || {
            for i in &*nums_arc {
                println!("number {}", i);
            }
        }));
    }

    for h in handles {
        let _ = h.join();
    }
}
/// # Споделяне на стойности - Arc
/// `std::sync::Arc`
/// - "Automatically Reference Counted" value
/// - аналогично на `Rc` (споделена собственост, позовлява взимане на `&T` към вътрешността)
/// - но използва атоматрни операции за броене на референциите
/// - поради това може да се използва от няколко нишки едновременно

/// # Send & Sync
/// - Грешката, която получихме беше че Rc<Vec<i32>> не имплементира Rc
/// - Следователно closure-а F не имплементира Send
/// - а spawn изисква F: Send
/// ```
/// pub fn spawn<F, T>(f: F) -> JoinHandle<T> where
///     F: FnOnce() -> T + Send + 'static,
///     T: Send + 'static
/// ```
///
/// Трейтовете `std::marker::Send` и `std::marker::Sync` показват дали един тип е thread safe.
/// Т.е дали обекти от този тип могат да се използват безопасно в многонишков контекст.
///
/// ## Send
/// - позволява прехвърляне на стойност между нишки
/// - пример за типове, които не са Send:
///     - thread local типове, напр. rang::rngs::ThreadRng
///     - Rc
///     - голи указатели - *const T и *mut T
///
/// ## Sync
/// - позволява споделен достъп до стойност от няколко нишки
/// - т.е. позволява прехвърляне на референция `&T` между нишки
/// - `T: Sync` ⟺ `&T: Send`
/// - пример за типове, които не са `Sync`:
///     - internal mutability без синхронизация - `Rc`, `Cell`, `RefCell`
///     - голи указатели - `*const T` и `*mut T`
///
/// ## Въпрос?
/// Дали обикновен тип като `Vec<T>` имплементира `Sync`?
///     - Да, ако `T: Sync`
///     - Ако нашата нишка има `&Vec<_>` - никой не може да модофицира вектора
///     - Ако нашата нишка има `&mut Vec<_>` - никой друг няма референция до вектора
///
/// ## Auto Traits
/// - имплеметира се автоматично ако всички полета са съответно Sync и Send
///
/// ## Unsafe Traits
/// - unsafe са за ръчна имплементация
/// ```
/// struct MyBox(&mut u8);
///
/// unsafe impl Send for MyBox {}
/// unsafe impl Sync for MyBox {}
/// ```
///
/// ## Деимплементация
/// ```
/// - Само за nightly
/// #![feature(optin_builtin_traits]
///
/// struct SpecialToken(u8);
///
/// impl !Send for SpecialToken {}
/// impl !Sync for SpecialToken {}
/// ```
/// - автоматичната имплементация никога няма да е грешна от само себе си
/// - но можем да пишем код, който разчита, че определен тип не може да се прехвърля / споделя
///
/// - хак за stable деимплементация на Send & Sync
/// ```
/// use std::marker::PhantomData;
///
/// struct SpecialToken(u8, PhantomData<*const ()>);
/// ```

/// # Примитиви за синхронизация
/// - Стандартния пример за грешен многонишков алогритъм не се компилира
///
/// ```
/// fn standard_multithreading_algo() {
///     let v = Arc::new((0..100).collect::<Vec<_>>());
///     let mut sum = 0;
///
///     let t1 = {
///         let v = Arc::clone(&v);
///         let sum = &mut sum;
///         thread::spawn(move || for i in &v[0..50] { *sum += i; })
///     };
///
///     let t2 = {
///         let v = Arc::clone(&v);
///         let sum = &mut sum;
///         thread::spawn(move || for i in &v[51..100] { *sum += i; })
///     };
///
///     let _ = t1.join();
///     let _ = t2.join();
///     println!("sum: {}", sum);
/// }
/// ```
///
/// ```
/// error[E0597]: `sum` does not live long enough
///    --> src/main.rs:346:19
///     |
/// 342 |     let mut sum = 0;
///     |         ------- binding `sum` declared here
/// ...
/// 346 |         let sum = &mut sum;
///     |                   ^^^^^^^^ borrowed value does not live long enough
/// 347 |         thread::spawn(move || for i in &v[0..50] { *sum += i; })
///     |         -------------------------------------------------------- argument requires that `sum` is borrowed for `'static`
/// ...
/// 359 | }
///     | - `sum` dropped here while still borrowed
///
/// error[E0499]: cannot borrow `sum` as mutable more than once at a time
///    --> src/main.rs:352:19
///     |
/// 346 |         let sum = &mut sum;
///     |                   -------- first mutable borrow occurs here
/// 347 |         thread::spawn(move || for i in &v[0..50] { *sum += i; })
///     |         -------------------------------------------------------- argument requires that `sum` is borrowed for `'static`
/// ...
/// 352 |         let sum = &mut sum;
///     |                   ^^^^^^^^ second mutable borrow occurs here
///
/// error[E0502]: cannot borrow `sum` as immutable because it is also borrowed as mutable
///    --> src/main.rs:358:25
///     |
/// 346 |         let sum = &mut sum;
///     |                   -------- mutable borrow occurs here
/// 347 |         thread::spawn(move || for i in &v[0..50] { *sum += i; })
///     |         -------------------------------------------------------- argument requires that `sum` is borrowed for `'static`
/// ...
/// 358 |     println!("sum: {}", sum);
///     |                         ^^^ immutable borrow occurs here
///     |
///     = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
/// ```
///
/// Защо не се компилира? какъв може да е типа на `sum`?
/// - `&mut i32` - не можем да имаме два пъти `&mut`, а и `spawn` очаква `'static`
/// - `Arc<i32>` - нямаме как да модифицираме съдържанието
/// - `Arc<Cell<i32>>`, `Arc<RefCell<i32>>` - `Cell` и `RefCell` не са `Sync`
///
/// Можем да го накараме да работи
/// - мутекс
/// - атомарни числа
/// - да връщаме резултат от нишката
/// - ...
///
/// ## Модула std::sync
/// - std::sync
/// - Arc
/// - Mutex, RwLock
/// - Condvar, Barrier
/// - atomic
/// - mpsc
fn standard_multithreading_algo() {
    // мутекса опакова стойността, която предпазва
    let mutex = Mutex::new(10);

    {
        // заключваме мутекса
        // `lock()` връща "умен указател" с deref до "&T" и "&mut T"
        let mut lock = mutex.lock().unwrap();
        *lock += 32;

        // мутекса се отключва когато `lock` се деалокира
    }
}

/// # Mutex
/// - mutual exclusion
/// - използва се, за да ни даде ексклузивен достъп до някакъв общ ресурс
/// - scope-а за който имаме ексклузивен достъп се нарича `критична секция`
/// - работи по следния начин
///     - съдържа флаг - дали мутекса е заключен или освободен
///     - ако мутекса е отключен и извикаме `lock` - заключваме го
///     - ако мутекса е заключен и извикаме `lock` - нишката се спира
///     - операционната система ще я събуди когато мутекса е свободен
///
/// Обикновено мутекса се възприема като примитива която определя кричтична секция
/// ```
/// lock(my_mutex);
/// // начало на критичната секция
///
/// do_stuff(shared_data);
///
/// // край на критичната секция
/// unlock(my_mutex);
/// ```
/// В Ръст това не би било удобно, защото не се дава достатъчно иформация на
/// компилатора как ползваме данните.
/// Затова `Mutex` е generic и опакова данните
///
/// - `Mutex<T>` опакова данни от тип T
/// - ако искаме мутекс без данни може да се използва `Mutex<()>`
/// - `mutex.lock()` връща `Result<MutexGuard<'a, T>, PoisonError>`
/// - `mutex.lock().unwrap()` връща `MutexGuard<'a, T>`
/// - `MutexGuard` има `Deref` до `&T` и `&mut T`
/// - единствения начин да достъпим данните е през `MutexGuard`
///
/// ## Panic
/// - `mutex.lock()` връща `Result<MutexGuard<'a, T>, PoisonError>`
/// - ако нишката е заключила мутекс и влезе `panic!` по това време, може
/// данните да са останали в (логическо) невалидно състояние
/// - мутекса се зачита за отровен
/// - От `PoisonError` може да се извади `MutexGuard`
/// - Често срещано е резултата от `lock` просто да се unwrap-не

/// # RwLock
/// - Reader-writer lock
/// - позволява четене от много нишки места
/// - или писане от едно място
/// - подобно на `RefCell`, но в многонишков контекст

/// # Mutex или RwLock
/// - `Mutex` е по-бърз и по-лек от `RwLock`
/// - `Mutex` налага дисциплина да държим критическите секции възможно най-кратки
/// - понякога `RwLock` се налага - напр. опаковане на стари C++ библиотеки

/// # Arc + Mutex
/// Подобно на `Rc<RefCell<T>>, може често да виждате Arc<Mutex<T>> и Arc<RwLock<T>>
fn arc_plus_mutex() {
    let v = Arc::new((0..100).collect::<Vec<i32>>());
    let total_sum = Arc::new(Mutex::new(0));

    let t1 = {
        let v = Arc::clone(&v);
        let total_sum = Arc::clone(&total_sum);
        thread::spawn(move || {
            let local_sum = v[0..50].iter().sum::<i32>();
            *total_sum.lock().unwrap() += local_sum;
        })
    };

    let t2 = {
        let v = Arc::clone(&v);
        let total_sum = Arc::clone(&total_sum);
        thread::spawn(move || {
            let local_sum = v[50..].iter().sum::<i32>();
            *total_sum.lock().unwrap() += local_sum;
        })
    };

    let _ = t1.join();
    let _ = t2.join();
    println!("sum = {}", *total_sum.lock().unwrap());
}

fn arc_plus_mutex_02() {
    let v = (0..100).collect::<Vec<i32>>();
    let mut total_sum = Mutex::new(0);

    thread::scope(|s| {
        s.spawn(|| {
            let local_sum = v[0..50].iter().sum::<i32>();
            *total_sum.lock().unwrap() += local_sum;
        });

        s.spawn(|| {
            let local_sum = v[50..].iter().sum::<i32>();
            *total_sum.lock().unwrap() += local_sum;
        });
    });

    println!("sum: {}", *total_sum.get_mut().unwrap());
}

/// # Атомарни числа
/// - аритметичните операции се свеждат до няколко отделни инструкции
/// - едновременни операции могат видят стари стойности
/// - затова не могат да се използват от можество нишки без синхронизация
/// ```
/// let mut num = 10;
///
/// // thread 1                 // thread 2
/// num += 5;                   num += 5;
///
/// // =============================================
///
/// let reg = load(&num);
/// let reg = reg + 5;          let reg = load(&num);
///                             let reg = reg + 5
///                             store(&mut num, reg);
/// store(&mut num, reg);       /* num = 15 */
/// /* num = 15 */
/// ```
///
/// има специални процесорни инструкции, които правят аритемтична операция за една инструкция
/// атомарни / неразделими
/// ```
/// // псевдокод
/// let num = 10;
///
/// // thread 1                 // thread 2
/// fetch_add(&num, 5);         fetch_add(&num, 5);
///
/// load(&num);                 load(&num);
/// /* num = 20 */              /* num = 20 */
/// ```
/// - атомарните числа използват атомарни инструкции
/// - `AtomicUsize`, `AtomicIsize`, `AtomicU8`, `AtomicU16`, ...
/// - `AtomicBool`
/// - `AtomicPtr`
/// - аритметични побитови операции: `fetch_add`, `fetch_xor`, ...
/// - операции по паметта: `load`, `store`, `compare_and_swap`, ...
///
/// Атомарните числа могат да се модифицират през споделена референиця
fn atomic_modification() {
    let num = AtomicI32::new(10); // няма 'mut'

    num.fetch_add(5, Ordering::SeqCst);
    num.fetch_add(5, Ordering::SeqCst);

    println!("{}", num.load(Ordering::SeqCst));
}
/// - Удобни са за създаване на различни флагове и броячи
/// - Стоят в основата на много алгоритми и структури от данни
/// - Препоръчително да се използват пред `Mutex<{integer}>`
fn atomic_example() {
    let v = Arc::new((0..100).collect::<Vec<i32>>());
    let total_sum = Arc::new(AtomicI32::new(0));

    let t1 = {
        let v = Arc::clone(&v);
        let total_sum = Arc::clone(&total_sum);
        thread::spawn(move || {
            let local_sum = v[..50].iter().sum::<i32>();
            total_sum.fetch_add(local_sum, Ordering::SeqCst);
        })
    };

    let t2 = {
        let v = Arc::clone(&v);
        let total_sum = Arc::clone(&total_sum);
        thread::spawn(move || {
            let local_sum = v[50..].iter().sum::<i32>();
            total_sum.fetch_add(local_sum, Ordering::SeqCst);
        })
    };

    let _ = t1.join();
    let _ = t2.join();
    println!("sum = {}", total_sum.load(Ordering::SeqCst));
}

fn atomic_example_with_scope() {
    let v = (0..100).collect::<Vec<_>>();
    let total_sum = AtomicI32::new(0);

    thread::scope(|s| {
        s.spawn(|| {
            let local_sum = v[0..50].iter().sum::<i32>();
            total_sum.fetch_add(local_sum, Ordering::SeqCst);
        });

        s.spawn(|| {
            let local_sum = v[50..].iter().sum::<i32>();
            total_sum.fetch_add(local_sum, Ordering::SeqCst);
        });
    });

    println!("sum: {}", total_sum.load(Ordering::SeqCst));
}

fn atomic_example_02() {
    let should_stop = Arc::new(AtomicBool::new(false));
    let t1 = thread::spawn({
        let should_stop = Arc::clone(&should_stop);
        move || {
            while !should_stop.load(Ordering::SeqCst) {
                println!("running");
                thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    });

    thread::sleep(std::time::Duration::from_millis(300));
    should_stop.store(true, Ordering::SeqCst);

    let _ = t1.join();
}

/// # Канали
/// Don't communicate by sharing memory
/// Share memory by communicating
fn channels_in_std() {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        sender.send(10).unwrap();
    });

    println!("received {}", receiver.recv().unwrap());
}

/// # Типове канали
/// ## Неограничен канал
/// - unbounded / infinitely buffered / "asynchronous"
/// - `std::sync::mpsc::channel()`
/// - `(Sender, Receiver)`
/// - Буфера се оразмерява, когато се напълни
/// - изпращането на съобщения никога не блокира
fn unbounded_channel() {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        sender.send(1).unwrap();
        sender.send(2).unwrap();
        sender.send(3).unwrap();
    });

    assert_eq!(receiver.recv().unwrap(), 1);
    assert_eq!(receiver.recv().unwrap(), 2);
    assert_eq!(receiver.recv().unwrap(), 3);
}

/// ## Ограничен канал
/// - bounded / "synchronous"
/// - `std::sync::mpsc::sync_channel(k)`
/// - `(SyncSender, Receiver)`
/// - има буфер за k съобщения
/// - изпращане на съобщения ще блокира, ако буфера е пълен
fn synchronous_channel() {
    let (sender, receiver) = mpsc::sync_channel(1);

    thread::spawn(move || {
        // Записва съобщението и връща веднага
        sender.send(1).unwrap();

        // ще блокира докато главната нишка не извика `receiver.recv()`
        sender.send(2).unwrap();
    });

    assert_eq!(receiver.recv().unwrap(), 1);
    assert_eq!(receiver.recv().unwrap(), 2);
}

/// ## Методи - Sender
fn async_sender_methods() {
    let (sender, receiver) = mpsc::channel();

    assert_eq!(sender.send(12), Ok(()));

    // унищожава получателя
    // съобщението `12` никога няма да бъде получено
    mem::drop(receiver);

    // грешка - получателя е унищожен
    // можем да си върнем съобщението `23` от грешката
    assert_eq!(sender.send(23), Err(SendError(23)));
}

/// ## Методи - SyncSender
fn sync_sender_methods() {
    let (sender, receiver) = mpsc::sync_channel(1);

    assert_eq!(sender.try_send(12), Ok(()));
    assert_eq!(sender.try_send(23), Err(TrySendError::Full(23)));

    mem::drop(receiver);

    assert_eq!(sender.try_send(23), Err(TrySendError::Disconnected(23)));
}

fn multiple_senders() {
    let (sender, receiver) = mpsc::channel();
    let sender2 = sender.clone();

    thread::spawn(move || {
        sender.send(1).unwrap();
        sender.send(2).unwrap();
    });

    thread::spawn(move || {
        sender2.send(3).unwrap();
        sender2.send(4).unwrap();
    });

    println!("{} {} {} {}",
        receiver.recv().unwrap(), receiver.recv().unwrap(),
        receiver.recv().unwrap(), receiver.recv().unwrap());
}

/// # Множество получатели
/// - не може - каналите са multi-producer, single-consumer
/// - `Receiver` не може да се клонира
/// - `Receiver` е `Send` - можем да го изпратим на друга нишка
/// - `Receiver` не е `Sync` - не можем да подадем `&Receiver` или `Arc<Receiver>`
/// ## Методи
fn receiver_methods() {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        for i in (0..50).rev() {
            sender.send(i).unwrap();
        }
    });

    while let Ok(msg) = receiver.recv() {
        println!("received {:?}", msg);
    }
}

/// ## Итератори
fn receiver_iter() {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        for i in (0..50).rev() {
            sender.send(i).unwrap();
        }
    });

    // обхожда всички съобщения в канала
    // ако има налично съобщение блокира
    // излиза от цикъла когато всички изпращачи са унищожени
    for msg in receiver.iter() {
        println!("received {}", msg);
    }

    // обхожда всички вече изпратени съобщения в канала,
    // след което излиза от цикъла
    for msg in receiver.try_iter() {
        println!("received {}", msg);
    }
}


fn main() {
    first_thread_spawn();
    println!();
    second_thread_spawn();
    println!();
    panic_in_thread();
    println!();
    access_to_values();
    println!();
    moves_access_values();
    println!();
    use_scoped_threads();
    println!();
    access_to_values_with_arc();
    println!();
    arc_plus_mutex();
    println!();
    arc_plus_mutex_02();
    println!();
    atomic_modification();
    println!();
    atomic_example();
    println!();
    atomic_example_with_scope();
    println!();
    atomic_example_02();
    println!();
    channels_in_std();
    println!();
    multiple_senders();
    println!();
    receiver_methods();
    println!();
}
