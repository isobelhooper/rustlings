// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!

fn main() {
    // let mut a = vec![1, 2, 3, 4, 5];
    // 
    // for number in (100..300) {
    //     a.push(number);
    // }
    let a = [5; 1000]; // 1000-length array, every value is 5.
    
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
