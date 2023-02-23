// -----------------------------------------------------

// fn main() {
//     let mut total = 30;
//     println!("Trabalhou {} horas", total);
//     total = 44;
//     println!("Trabalhou {} horas", total);
// }

// const SECOND_IN_MINUTES: u32 = 60;
// const MINUTES_IN_HOUR: u32 = 60;
// const SECOUNDS_IN_HOUR: u32 = SECOND_IN_MINUTES * MINUTES_IN_HOUR;

// -----------------------------------------------------

// fn main() {
//     let total = 30;
//     let total_em_segundos = total * SECOUNDS_IN_HOUR;
//     println!("Trabalhou {} segundos", total_em_segundos)
// }

// -----------------------------------------------------

// fn main() {
//     // let x: u8 = 5;
//     // let y: u8 = x - 20;

//     let x = 5;
//     let y = x - 20;
// }

// -----------------------------------------------------

// fn main() {
//     // tuplas
//     let mut numbers = (1, 2, 3.5);
//     let (a, b, c) = numbers;
//     numbers.1 = 5000;
//     println!("{}", a);
//     println!("{}", b);
//     println!("{}", c);
//     println!("{:?}", numbers);
//     println!("{:?}", numbers.2);
// }

// -----------------------------------------------------

// fn main() {
//     // matriz
//     let numbers = [1, 2, 3];
//     println!("{:?}", numbers[2]);
//     println!("{:?}", &numbers[..2]);
// }

// static _Y: u32 = 13;

// -----------------------------------------------------

// fn main() {

// }

// -----------------------------------------------------

// fn main() {
//     Heap String
//     String Dinâmica
//     String

//     let mut s = String::new();

//     1° forma ----
//     s.push('C');
//     s.push('a');
//     s.push('s');
//     s.push('s');
//     s.push('i');
//     s.push('o');

//     2° forma ----
//     s.push_str("Cassio");
//     s.push_str(" ");
//     s.push_str("Oliveira");

//     3° forma ----
//     let s = "Cassio".to_string();

//     4° forma ----
//     let s = String::from("Cassio");

//     5° forma ----
//     let name = ['C', 'a', 's', 's', 'i', 'o'];
//     let s = String::from_iter(name);

//     6° forma ----
//     let s: String = "Cassio".into();

//     7° forma ----
//     let mut s = String::from("Cassio");

//     let mut s = "Cassio".to_owned();

//     println!("{s}");
// }

// use std::io;

// fn main() {
//     println!("{:-^40}", "Calculator");
//     let mut s = String::new();
//     println!("Enter your name");

//     io::stdin()
//         .read_line(&mut s)
//         .expect("Error reading console");

//     let banner = "Digite uma sequencia de números \n\
//         separação
//         ";

//     println!("{banner}");
//     println!("Your name is {s}");
//     println!("Quantity letters {}", s.trim().len()); // não usar com emojis ou caracteres especiais
//     println!("Quantity letters {}", s.trim().chars().count()); // usar para contar strings com caracteres especiais
//     println!("{}", s.trim().to_uppercase());
//     println!("{}", s.trim().to_lowercase());
//     println!("{}", s.trim().replace("a", "&"));
// }

// -----------------------------------------------------

// fn main() {
//     println!("{:-^40}", "Calculator");
//     let mut s = String::new();

//     io::stdin()
//         .read_line(&mut s)
//         .expect("Error reading console");

//     // fn clean(c: &str) -> &str {
//     //     c.trim()
//     // }

//     let nums: Vec<i32> = s
//         .split(",")
//         .map(|c| c.trim().parse().expect("Error"))
//         .collect();

//     let result: i32 = nums.iter().sum();

//     println!("{:?}", nums);
//     println!("{:?}", result);
// }

// -----------------------------------------------------

// fn main() {
//     say_hello("Cassio");
//     say_hello("Elanne");
//     let res = add_numbers(5, 10);
//     println!("{:?}", res);
//     sum_numbers("1 2 4")
// }

// fn say_hello(name: &str) {
//     println!("Hello {name}")
// }

// fn add_numbers(x: i32, y: i32) -> i32 {
//     // return x + y; -> usar com clause guard

//     if (x == 0) {
//         return y;
//     }

//     x + y
// }

// fn sum_numbers(number_string: &str) {
//     let sum: Vec<i32> = number_string
//         .trim()
//         .split(" ")
//         .map(|n| n.parse::<i32>().expect("error"))
//         .map(|n| n * 2)
//         .collect();
//     println!("{:?}", sum.iter().sum::<i32>())
// }

// -----------------------------------------------------

fn main() {}
