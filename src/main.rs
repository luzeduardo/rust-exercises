fn main() {
    testing_refactoring_struct_tuples();
}

struct Rectangle {
    width: u32,
    height: u32,
}
fn testing_refactoring_struct_tuples() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("\nAre of rectangle in pixels -> {}", area(&rect1));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

/*

fn testing_owner_refs() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let _s3 = takes_and_gives_back_ownership(s2);

    let len = calculate_length_with_reference(&s1);
    println!("The length of {} is {}.", s1, len);

    let mut s = String::from("HI");
    change(&mut s);

    let _ref_to_noth = dangle();

    let word = String::from("Eusou alenda");
    let txt = first_word(&word);
    // word.clear();
    println!("First: {}", txt);
}

fn testing_structs() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    };

    let mut user = User {
        email: String::from("edu"),
        username: String::from("asdad"),
        active: true,
        sign_in_count: 1,
    };

    user.sign_in_count = 2;
    println!("Count {}", user.sign_in_count);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn dangle() -> String {
    let s = String::from("brilha brilha");
    s
}

fn change(s: &mut String) {
    s.push_str(", Old");
}

fn calculate_length_with_reference(s: &String) -> usize {
    s.len()
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back_ownership(a_string: String) -> String {
    a_string
}
 */
