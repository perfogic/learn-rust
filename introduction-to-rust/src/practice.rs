use std::{collections::HashMap, ops::Add};

#[derive(Debug)]
enum SomeValue {
    StringValue(String),
    IntValue(u32),
}

// Array and Vector
fn practice_one() {
    let multi_array: [SomeValue; 4] = [
        SomeValue::StringValue(String::from("one")),
        SomeValue::IntValue(3),
        SomeValue::StringValue(String::from("two")),
        SomeValue::IntValue(4),
    ];

    for item in &multi_array {
        match item {
            SomeValue::StringValue(data) => {
                println!("The string is {}", data);
            }
            SomeValue::IntValue(data) => {
                println!("The integer is {}", data);
            }
        }
    }

    println!("{:?}", multi_array);

    let mut vector: Vec<&str> = vec!["one", "two", "three"];
    vector.push("four");
    println!("{:?}", vector);
}

#[derive(Debug)]
enum CharacterValue {
    Name(String),
    Age(i32),
    Items(Vec<String>),
}
// HashMap
fn practice_two() {
    let mut profile: HashMap<&str, CharacterValue> = HashMap::new();

    profile.insert("name", CharacterValue::Name(String::from("Pham Minh Dang")));
    profile.insert("age", CharacterValue::Age(23));
    profile.insert(
        "items",
        CharacterValue::Items(vec![String::from("laptop"), String::from("iphone")]),
    );

    println!("Profile {:?}", profile);

    // Two ways getting item in a hashmap
    match profile.get("name") {
        Some(value_data) => {
            println!("value: {:?}", value_data);
        }
        None => {
            println!("value is none");
        }
    }

    let value = profile.get("name").unwrap();
    println!("value is {:?}", value);
}

fn error_check(check: bool) -> Result<i8, &'static str> {
    if check {
        Err("This is an error")
    } else {
        Ok(1)
    }
}

// Error handling
fn practice_three() {
    println!("{:?}", error_check(false));
    println!("{:?}", error_check(false).is_err());
    println!("{:?}", error_check(true));
    println!("{:?}", error_check(true).is_err());

    let result: i8 = error_check(true).expect("This has been caught");
    println!("{}", result);
}

fn use_string(string: &String) {
    println!("{}", string);
}

// Borrowing
fn practice_four() {
    let string_one = String::from("github.com/");

    // This will not work because the value have been borrowed
    // string_one.add("perfogic");
    // use_string(&string_one);

    println!("{}", string_one);
    use_string(&string_one);
    string_one.add("perfogic");
}

// Scopes
fn practice_five() {
    let one = &"one";
    let two: &str;
    let three: String;

    {
        println!("{}", one);
        two = "two";
        three = String::from("three");
    }

    println!("{}", one);
    println!("{}", two);
    println!("{}", three);
}

fn get_highest<'a>(first_number: &'a i8, second_number: &'a i8) -> &'a i8 {
    if first_number > second_number {
        first_number
    } else {
        second_number
    }
}

fn get_longer_str(first_number: String, second_number: String) -> String {
    if first_number.len() > second_number.len() {
        first_number
    } else {
        second_number
    }
}

// Lifetimes
fn practice_six() {
    let one = String::from("1");
    let outcome;
    
    {
        let two = String::from("2");
        outcome = get_longer_str(one.to_owned(), two.to_owned());
    }

    println!("{} {}", outcome, one);

}

pub fn practice_from_book() {
    // practice_one();
    // practice_two();
    // practice_three();
    // practice_four();
    // practice_five();
    practice_six();
}
