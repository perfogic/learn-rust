fn main() {
    // tuples
    let simple_tuples = (String::from("This is x"), "hello", 5);
    println!("This is tuple {}", simple_tuples.1);

    // loop
    'outer: loop {
        let mut i = 0;
        'inner: loop {
            i = i + 1;
            if i == 3 {
                println!("Touch");
                break 'inner;
            }
            if i > 5 {
                println!("Break outer loop");
                break 'outer;
            }    
        }
        println!("Break from inner loop");
        break;
    }

    // matches
    let proposal_status = false;
    let proposal_name;

    match proposal_status {
        true => proposal_name = "Ok good",
        false => proposal_name = "Damn so bad",
    }

    println!("proposal name: {proposal_name}");

    // range inclusive
    let vecs = vec![1,2,3];
    let small_vecs = vecs[1..=2].to_vec();
    println!("Vector: {:?}", small_vecs);

    // for loop
    for i in 1..3 {
        println!("i: {i}");
    }

    // Remove warning for unusable value
    let _x = [1,3];

    // static and constant
    static DESTINATION: &str = "destination";

    let x = DESTINATION;
    let mut y = DESTINATION;
    y = "CARGO";
    println!("{} - {}", x, y);
}
