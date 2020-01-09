use std::collections::HashMap;

fn main() {
    // testing_takes_ownership();
    //testing_refactoring_struct_tuples();
    //testing_dangle_error() 
    //testing_vector();
    //testing_strings();
    //testing_internal_representation_of_string();
    testing_hash_map();
}

fn testing_hash_map() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    //creating a hashmap from a vector of tuples
    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    print!("{:?}", scores);

}


fn testing_internal_representation_of_string() {
    let len = String::from("Hola").len();

}

fn testing_strings() {
    let mut a = String::new();

    let mut s = String::from("Test");
    s.push_str(" appending string");
    println!("{}", s);
    let s1 = String::from("raise error if access by index");
    //let h = s1[0];
    //iterating of string with chars method
    println!("iterating the string chars"); 
    for g in "Ola".chars() {
        println!("{}", g);
    }

    //iterating over the bytes
    println!("iterating the bytes of the string in utf8"); 
    for b in "Ѫi".bytes() {
        println!("{}", b);
    }
}

fn testing_vector() {
    let mut ve: Vec<i32> = Vec::new();
    ve.push(1);

    println!("Simple vec {:?}", ve);

    let mut v = vec![1 , 2 , 3];
    v.push(4);

    println!("Vec content: {:?}", v);
    //both v and ve goes out of scope and is freed


    //reading elements
    let v2 = vec![1,2,3,4,5];
    let third: &i32 = &v2[2];//using ref to access can throw panic at runtime 
    println!("The third element is {}", third);

    match v2.get(2) {//using get returns an Option<&T>
        Some(third) => println!("Third is {}", third),
        None => println!("There is not third element"),
    }//match handles gracefully a non existant key with None


    //iterate over unmutable vec
    let v3 = vec![ 1, 2, 3, 4, 5];
    for i in &v3 {
        println!("{}", i);
    }

    //over mutable with changes
    let mut vecmut = vec![1, 2, 3, 4];
    for i in &mut vecmut {
        *i += 500; // *i dereference operator to get value in i before we use
    }
    println!("{:?}", vecmut);
}

#[derive(Debug)] //Adding Trait to support debug with {:?}
struct Rectangle {
    width: u32,
    height: u32,
}
fn testing_refactoring_struct_tuples() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("\nArea of rectangle in pixels -> {}", area(&rect1));

    let rect2 = Rectangle::square(2);
    println!("\nArea in pixels -> {}", area(&rect2));

    println!(
        "\nUsing supported Debug of a struct from a Trait-> {:?}",
        rect2
    );
}

impl Rectangle {
    //defined as method in impl Rectangle Counttext  which receives self as ref
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // associated functions are often used for constructors that return an instance of the struct
    // it can resid into struct but are functions not methods due it do not have an instance of the
    // struct to work with
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn testing_dangle_error() {
    let _ref_to_noth = dangle_error();
}

//throws error because I am trying to return a ref to and deallocated point of memory. 
fn dangle_error() -> String {//it returns a ref to a string
    let s = String::from("brilha brilha");
    // &s //it returns a ref to the string s but it is goes out of scope and is dropped
        //One solution could be return s using move ownership out since nothing will be deallocated
    s
}

// defined as function
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn testing_takes_ownership() {
    let s = String::from("Some simple texti!"); //s goes into scope
    takes_ownership(s); // s MOVES into function
                        // so now it is no longer valid here

    let x = 5;
    makes_copy(x); // x would move but since it is a u32 which uses the STACK to handle this this known size required of memory it COPY, so x is valid below the function
    println!("Integer after makes_copy -> {}", x);
    //Trying to use s here will thown an compile exception
}
fn takes_ownership(somestring: String) {
    println!("Inside takes_ownership -> {}", somestring);
} // Here somestring goes out of the scope and the Drop Trait is called. Memory is freed.
fn makes_copy(example_integer: u32) {
    println!("Inside makes_copy-> {}", example_integer);
} // The example_integer goes out of the scope

/*

fn testing_owner_refs() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let _s3 = takes_and_gives_back_ownership(s2);

    let len = calculate_length_with_reference(&s1);
    println!("The length of {} is {}.", s1, len);

    let mut s = String::from("HI");
    change(&mut s);

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
