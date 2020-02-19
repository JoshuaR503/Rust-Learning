enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}


fn main() {

}


fn value_in_cents(coin: Coin) -> u8 {
    // This seems very similar to an expression used with if, but there’s a big difference:
    // with if, the expression needs to return a Boolean value, but here, it can be any type. 
    match coin {
        // Next are the match arms. An arm has two parts: a pattern and some code. 
        // The first arm here has a pattern that is the value Coin::Penny 
        // and then the => operator that separates the pattern and the code to run. 
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}