macro_rules! test_me {
    // expr: is variable
    ($a: expr, $b: expr; $c: expr) => {
        $a + $b + $c
    };
}

macro_rules! input {
    // ty: is data type
    ($t: ty) => {{
        let mut n = String::new();
        std::io::stdin()
            .read_line(&mut n)
            .expect("Fail to read input");
        let n: $t = n.trim().parse().expect("invalid input");
        n
    }};
}

// macro do nothing with ownership
// I just need to care about the expansion
// macro world can't take the variable outside it world
// to pass variable i have to use identifier
macro_rules! some_macro {
    ($var: ident) => {
        $var = $var + 1;
    }
}

macro_rules! create_function {
    ($func_name: ident, $input: ident, $type_input: ty) => {
        fn $func_name($input: $type_input) {
            println!("You have call {:?} with input: {:?}", stringify!($func_name), stringify!($type_input));
        }
    };
}

create_function!(test_me, x, i32);
fn main() {
    println!("Hello, world!");
    println!("{}", test_me![3, 4; 5]);
    println!("{}", input!(i32));
    let mut x = 4;
    some_macro!(x);
    println!("x: {x}");

    test_me(5);
}
