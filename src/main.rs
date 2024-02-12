fn factorial(n: u32) -> u32 {
    let mut product = 1;
    for i in 1..=n {
        product *= dbg!(i);
    }
    product
}

fn fizzbuzz(n: u32) {
    for i in 0..=n {
        if i % 3 == 0 {
            println!("{i} is divisible by 3");
        } else if i % 5 == 0 {
            println!("{i} is divisible by 5");
        } else if i % 3 == 0 && i % 5 == 0 {
            println!("{i} is divisible by both 3 and 5");
        } else {
            println!("{i} is neither divisible by 3 or 5");
        }
    }
}

fn main() {
    fizzbuzz(44);
    let n = 4;
    println!("{n}! = {}", factorial(n));
}
