fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // let x = 5; immutable
    println!("Const value {THREE_HOURS_IN_SECONDS}");
    let mut x = 5;
    println!("The value is {x}");
    x = 6;
    println!("The value is {x}");
    // shadowing
    let y = 5;
    let y = y + 1;

    {
        let y = x * 2;
        println!("the val is {y}");
    }
    println!("the val is {y}");

    let spaces = "   ";
    let spaces = spaces.len();

    // mismatch type
    // let mut spaces = "   ";
    // spaces = spaces.len();
}
