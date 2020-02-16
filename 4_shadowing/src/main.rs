// This program first binds x to a value of 5. 
// Then it shadows x by repeating let x =, taking the original value and adding 1 so the value of x is then 6. 
// The third let statement also shadows x, multiplying the previous value by 2 to give x a final value of 12. 
// When we run this program, it will output the following:
fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}