fn main() {
    let mut x = 5;
    { //Creating a scope block
        let y = &mut x; // y is a mutable reference to x
        *y = 10; // Modify x through y
    }
    { //Creating a second scope block
        let z = &mut x; // z is a mutable reference to x
        *z = 15; // Modify x through z
    }
    println!("x = {}", x); // This will print 15
}
