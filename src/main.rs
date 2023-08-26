/*
Optional struct, in Rust not exist 'null' or 'nil' type, for error manage  we can use Optional (like java)
enum Option<T> {
    None,     // The value doesn't exist
    Some(T),  // The value exists
}
*/
fn main() {
    println!("Example for Optional");

    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

    // pick the first item:
    let first = fruits.get(0);
    println!("{:?}", first);

    // pick the third item:
    let third = fruits.get(2);
    println!("{:?}", third);

    // pick the 99th item, which is non-existent:
    let non_existent = fruits.get(99);
    println!("{:?}", non_existent);
}
