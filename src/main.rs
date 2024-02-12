/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(n: i32) -> u32 {
    if n > 1 {
        if n % 2 == 0 {
            println!("{n} is even");
            collatz_length(n / 2)
        } else {
            println!("{n} is odd");
            collatz_length(n * 3 + 1)
        }
    } else {
        1
    }
}

fn main() {
    println!("{}", collatz_length(56));
}
