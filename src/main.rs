mod non_recursive;
mod recursive;

use std::time::Instant;

fn main() {
    println!("Here is the recursive one:");
    let start = Instant::now();
    recursive::hanoi(10, 'A', 'C', 'B');
    let recursive_elapsed = start.elapsed();

    println!("Here is the non-recursive one:");
    let start = Instant::now();
    non_recursive::hanoi(10, 'A', 'C', 'B');
    let non_recursive_elapsed = start.elapsed();

    println!("Recursive time elapsed: {:.9?}", recursive_elapsed);
    println!("Non-recursive time elapsed: {:.9?}", non_recursive_elapsed);
}
