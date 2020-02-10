fn main() {
    let test = "aaa";

    let s = String::from("hello word");  // s 进入作用域

    let str = first_word(&s);

    println!("{}", str);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for byte in bytes.iter().enumerate() {
        println!("{},{}", byte.0, byte.1);
    }

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
