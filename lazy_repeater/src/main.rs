fn main() {
    let mut looper = make_looper("Hello, world!");
    println!("{}", looper());
    for _ in 0..15 {
        println!("{}", looper());
    }
}

fn make_looper(string: &str) -> impl FnMut() -> char + '_ {
    let mut chars = string.chars().cycle();
    move || chars.next().unwrap()
}