use std::{collections::HashMap};

fn main() {
    println!("{:?}", mean_mode(&[12, 15, 6, 3, 5, 6 ,1, 3, 4, 2, 7, 8, 6]));
    println!("{}", pig_latin("avocal consonante"));
}

fn mean_mode(arr: &[u32]) -> (u32, u32) {
    let mut array = arr.to_owned();
    array.sort_unstable();
    let median = match array.get(array.len()/2) {
        Some(s) => *s,
        None => u32::MAX
    };

    let mut count: HashMap<u32, u32> = HashMap::new();
    for i in array {
        let c = count.entry(i).or_insert(0);
        *c += 1;
    }

    let mut mode = u32::MAX;
    let mut mode_count: u32 = 0;
    for (i, c) in count {
        if c > mode_count {
            mode = i;
            mode_count = c;
        }
    }

    (median, mode)
}

fn pig_latin(s: &str) -> String {
    const VOWELS: [char;5] = ['a','e','i','o','u'];

    let mut string: String = String::new();

    for w in s.split_whitespace() {
        let mut iterator = w.chars();
        let c = match iterator.next() {
            Some(c) => c,
            None => ' '
        };
        if VOWELS.contains(&c) {
            string.push_str(w);
            string.push_str("-hay ");
        } else {
            loop {
                let c = iterator.next();
                match c {
                    Some(c) => string.push(c),
                    None => break
                }
            }
            string.push_str(&format!("-{c}ay "));
        }
    }
    string.trim().to_string()
}