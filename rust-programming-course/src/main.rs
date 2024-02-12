fn main() {
    // First lession
    let s1 = String::from("This is superhero 1");
    {
        let s2 = s1;
        println!("Value {:?}", s2);
    }
    // println!("Value {:?}", s1); // this will not work because s2 already release memory alocation for "This is superhero 1" in heap
    // For fixing, let do: let s2 = s1.clone(). which will assign different location in heap
    // however, this can work on primitive types.

    let x = 15;
    let y = x; // this will be copied => each will have their ownership (which will happen on primitive types)
    println!("x: {x}, y: {y}");

    let mut x = 15;
    let y = &x;
    x = 25;
    println!("x: {x}");
}
