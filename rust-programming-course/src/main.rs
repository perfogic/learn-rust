// macro_rules! test_me {
//     // expr: is variable
//     ($a: expr, $b: expr; $c: expr) => {
//         $a + $b + $c
//     };
// }

// macro_rules! input {
//     // ty: is data type
//     ($t: ty) => {{
//         let mut n = String::new();
//         std::io::stdin()
//             .read_line(&mut n)
//             .expect("Fail to read input");
//         let n: $t = n.trim().parse().expect("invalid input");
//         n
//     }};
// }

// // macro do nothing with ownership
// // I just need to care about the expansion
// // macro world can't take the variable outside it world
// // to pass variable i have to use identifier
// macro_rules! some_macro {
//     ($var: ident) => {
//         $var = $var + 1;
//     }
// }

// macro_rules! create_function {
//     ($func_name: ident, $input: ident, $type_input: ty) => {
//         fn $func_name($input: $type_input) {
//             println!("You have call {:?} with input: {:?}", stringify!($func_name), stringify!($type_input));
//         }
//     };
// }

// create_function!(test_me, x, i32);
// fn main() {
//     println!("Hello, world!");
//     println!("{}", test_me![3, 4; 5]);
//     println!("{}", input!(i32));
//     let mut x = 4;
//     some_macro!(x);
//     println!("x: {x}");

//     test_me(5);
// }

// NEW SECTION
// macro_rules! string_concat {
//     () => {
//         String::new()
//     };

//     ($some_str: expr) => {{
//         let mut temp_str = String::new();
//         temp_str.push_str($some_str);
//         temp_str
//     }};

//     ($some_s1: expr, $some_s2: expr) => {{
//         let mut temp_str = String::new();
//         temp_str.push_str($some_s1);
//         temp_str.push_str($some_s2);
//         temp_str
//     }};

//     // Repeated pattern, *: 0 or more, +: 1 or more, ?: 0 or 1
//     ($($some_str: expr), *) => {{
//         let mut temp_str = String::new();
//         $(temp_str.push_str($some_str);)*
//         temp_str
//     }};
// }

// // Repeating patterns
// fn main() {
//     let str_null = string_concat!();
//     let str_single = string_concat!("Fish");
//     let str_repeat = string_concat!("Fish", "Cat", "Bulk");
//     println!("{:?}", str_repeat);

// }

// NEW SECTION
macro_rules! make_struct {
    ($name:ident {$($field:ident: $ty:ty),*}) => {
        #[derive(Debug)]
        struct $name {
            $($field: $ty),*
        }
    };
}

// Sample usage 
make_struct!(MyStruct {
    field1: i32,
    field2: String
});

fn main(){
    let x = MyStruct {
        field1: 3,
        field2: "Dang".to_string()
    };
    println!("{:?}", x);
}