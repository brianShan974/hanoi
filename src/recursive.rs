pub fn hanoi(n: usize, from: char, to: char, via: char) {
    if n == 1 {
        println!("{} -> {}", from, to);
    } else {
        hanoi(n - 1, from, via, to);
        println!("{} -> {}", from, to);
        hanoi(n - 1, via, to, from);
    }
}
