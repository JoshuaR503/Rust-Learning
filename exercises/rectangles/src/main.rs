// Refactoring with Structs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// To define the function within the context of Rectangle, we start an impl (implementation) block.
impl Rectangle {

    // In the signature for area, we use &self instead of rectangle: &Rectangle because 
    // Rust knows the type of self is Rectangle due to this method’s being inside the impl Rectangle context.
    // Note that we still need to use the & before self.
    // We’ve chosen &self here for the same reason we used &Rectangle in the function version: 
    // we don’t want to take ownership, and we just want to read the data in the struct, not write to it. 
    // If we wanted to change the instance that we’ve called the method on as part of what the method does, we’d use &mut self as the first parameter.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // We can tell what the type of the parameter will be by looking at the code that calls the method: 
    // rect1.can_hold(&rect2) passes in &rect2, which is an immutable borrow to rect2, an instance of Rectangle.
    // This makes sense because we only need to read rect2 (rather than write, which would mean we’d need a mutable borrow), 
    // and we want main to retain ownership of rect2 so we can use it again after calling the can_hold method. 
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // To call this associated function, we use the :: syntax with the struct name; let sq = Rectangle::square(3);
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    // Putting the specifier :? inside the curly brackets tells println! we want to use an output format called Debug.
    println!(
        "\n\nThe area of the rectangle is {:#?} square pixels.\n\n",
        rect1.area()
    );

    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}\n\n\n", rect1.can_hold(&rect3));
    println!("square: {:#?}", Rectangle::square(3));
}
