mod prelude;
use prelude::*;

fn main() {
    let user = User::new("Jinhee", 28);
    println!("User: {:?}", user);

    let numbers = vec![1, 2, 3, 4, 5];
    let doubled = numbers.map(|x| x * 2).filter(|x| *x > 5);
    println!("Filtered Doubled: {:?}", doubled);

    match compute_result(2) {
        Some(v) => println!("Result: {}", v),
        None => println!("Computation failed."),
    }

    let total = numbers.fold(0, |a, b| a + b);
    println!("Total: {}", total);
}
