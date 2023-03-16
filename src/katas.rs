fn positive_sum(slice: &[i32]) -> i32 {
  // my
  // let mut sum = 0;

  // for x in slice {
  //     if x > &0 {
  //         sum += x;
  //     }
  // }

  // sum

  // --------

  // best
  // slice.iter().filter(|x| x.is_positive()).sum()

  // --------

  // clever
  slice.iter().fold(0, |accumulator, &current_value| {
    accumulator + if current_value > 0 { current_value } else { 0 }
  })
}

fn even_or_odd(i: i32) -> &'static str {
  // my
  // if i % 2 == 0 {
  //     return "Even";
  // }

  // best
  // match i % 2 {
  //     0 => "Even",
  //     _ => "Odd",
  // }

  // clever
  ["Even", "Odd"][i as usize % 2]
}

fn solution(phrase: &str) -> String {
  // my
  // let arr: Vec<char> = phrase.chars().collect();
  // let mut reverse = String::new();
  // for x in (0..arr.len()).rev() {
  //     reverse += &arr[x].to_string();
  // }
  // reverse

  // best
  // phrase.chars().rev().collect();

  println!("{:?}", phrase.chars().rev().collect::<String>());
  "C".to_string()
}

fn bool_to_word(value: bool) -> &'static str {
  if value {
    "Yes"
  } else {
    "No"
  }
}

fn number_to_string(i: i32) -> String {
  i.to_string()
}

fn remove_char(s: &str) -> String {
  // my
  // let arr: Vec<char> = s.chars().collect();
  // let mut formatted_string = String::new();
  // for x in 0..arr.len() {
  //   let i = arr
  //     .iter()
  //     .enumerate()
  //     .map(|(index, _)| index)
  //     .collect::<Vec<usize>>();

  //   if i[x] != 0 && i[x] != &arr.len() - 1 {
  //     formatted_string += &arr[x].to_string()
  //   }
  // }
  // formatted_string

  // best
  // s[1..s.len() - 1].to_string()

  // clever
  s.chars().skip(1).take(s.chars().count() - 2).collect()
}

fn repeat_str(src: &str, count: usize) -> String {
  // my
  // let mut new_string = String::new();
  // for _ in 0..count {
  //   new_string += src;
  // }

  // new_string

  // clever
  std::iter::repeat(src).take(count).collect::<String>()
}

fn square_sum(vec: Vec<i32>) -> i32 {
  // my
  // vec.iter().map(|n| n.pow(2)).fold(0, |acc, cur| acc + cur)
  vec.iter().map(|s| s * s).sum()
}

fn summation(n: i32) -> i32 {
  // my
  // let mut arr = vec![n];

  // for x in 1..n {
  //   arr.push(x)
  // }

  // arr.iter().sum()

  // clever
  (1..=n).sum()
}

fn no_space(x: String) -> String {
  // my
  // x.replace(" ", "")

  // other
  // println!("{:?}", x.split_whitespace().collect::<String>());

  // other
  println!(
    "{:?}",
    x.chars().filter(|c| !c.is_whitespace()).collect::<String>()
  );

  "".to_string()
}

fn get_count(string: &str) -> usize {
  // my
  // let mut vowels_count: usize = 0;

  // for word in string.split("") {
  //   if word == "a" || word == "e" || word == "i" || word == "o" || word == "u" {
  //     vowels_count += 1
  //   }
  // }

  // vowels_count

  // best
  // string.chars().filter(|&c| "aeiou".contains(c)).count()

  // clever
  // string.matches(|x: char| match x { 'a' | 'e' | 'i' | 'o' | 'u' => true, _ => false }).count()

  // clever 2
  // string.matches(&['a', 'e', 'i', 'o', 'u']).count()

  0
}

fn solve(s: &str) -> String {
  // my
  // if s.chars().filter(|&c| c.is_uppercase()).count() > s.len() / 2 {
  //   s.to_uppercase()
  // } else {
  //   s.to_lowercase()
  // }

  // clever
  let num_uppercase = s.matches(char::is_uppercase).count();
  match num_uppercase > s.len() - num_uppercase {
    true => s.to_uppercase(),
    false => s.to_lowercase(),
  }
}

fn main() {
  // let result = positive_sum(&[1, 4, -1, -6, 10]);
  // let result = even_or_odd(24);
  // let result = solution("Cassio");
  // let result = bool_to_word(false);
  // let result = remove_char("Cassio Oliveira Silva");
  // let result = repeat_str("cassio", 4);
  // let result = square_sum([2, 1].to_vec());
  // let result = summation(8);
  // let result = no_space("ca  asda as123 sad dasd ds d asd ad".to_string());
  // let result = get_count("cassioaaw12sa1aa");
  let result = solve("CASiio");
  println!("{result}");
}
