fn main() {
    let mut x = 5;
    { //Creating a new scope 
        let y = &mut x;
        *y = 10;
    }
    { //Creating a new scope
        let z = &mut x;
        *z = 15;
    }
    println!("x = {}", x);
}
//Another solution
fn main() {
    let mut x = 5;
    let y = x;
    let z = x; 
    let mut y_mut = y;
    let mut z_mut = z;
    y_mut = 10;
    z_mut = 15;
    println!("x = {}", x);
    println!("y = {}", y_mut);
    println!("z = {}", z_mut);
}