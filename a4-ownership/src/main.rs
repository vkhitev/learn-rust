fn borrow() -> String {
    let s1 = String::from("hello");
    let s2 = s1;
    return s2;
}

fn clone() -> (String, String) {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    return (s1, s2);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn use_calculate_length() {
    let s1 = String::from("hello");
    let (_s2, _len) = calculate_length(s1);
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

fn use_calculate_length_ref() {
    let s1 = String::from("hello");
    let _l = calculate_length_ref(&s1);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn use_change() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn multiple_refs() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s;
    println!("{}", r3);
}

fn _first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_sliced(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn use_first_word_sliced() {
    let my_string = String::from("hello world");

    let _word = first_word_sliced(&my_string[..]);

    let my_string_literal = "hello world";

    let _word = first_word_sliced(&my_string_literal[..]);

    let _word = first_word_sliced(my_string_literal);
}

fn main() {
    borrow();
    clone();
    use_calculate_length();
    use_calculate_length_ref();
    use_change();
    multiple_refs();
    use_first_word_sliced();
}
