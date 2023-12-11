use std::thread;

fn main() {
    let mut threads = Vec::new();
    let mut i = 0;
    while i < 10 {
        let start = i * 100000;
        let end = (i + 1) * 100000;

        let t = thread::spawn(move || {
            let primes = get_primes(start, end);
            println!("Thread {} finished", i);
            return primes;
        });

        threads.push(t);
        i += 1;
    }

    for t in threads {
        let primes = t.join().unwrap();
        println!("Primes: {:?}", primes);
    }
}

fn get_primes(start: u32, end: u32) -> Vec<u32> {
    let mut primes: Vec<u32> = Vec::new();
    let mut i = start;
    while i < end {
        if prime_number(i) {
            primes.push(i);
        }
        i += 1;
    }
    primes
}

fn prime_number(n: u32) -> bool {
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}
