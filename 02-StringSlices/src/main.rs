fn main() {
    let s = "Hello World";
    println!("{}", first_word(s));
    println!("{}", second_word(s));
}

fn first_word(s: &str) -> &str {
    for (i, &c) in s.as_bytes().iter().enumerate() {
        if c == b' ' {
            return &s[..i];
        }
    }
    s
}

fn second_word(s: &str) -> &str {
    let mut first: usize  = 0;

    for (i, &c) in s.as_bytes().iter().enumerate() {
        if c == b' ' {
            if first == 0 {
                first = i;
            } else {
                return &s[(first + 1)..i];
            }
        }
    }

    if first == 0 {
        ""
    } else {
        &s[(first + 1)..]
    }
}