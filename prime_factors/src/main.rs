use std::collections::HashMap;

fn main() {
    println!("{}", prime_factors(7775460));
}

fn prime_factors(n: i64) -> String {
    let mut primes = HashMap::new();
    let mut n = n;
    let mut counter = 2;

    while n > 1 {
        if n % counter == 0 {
            n = n / counter;
            primes.insert(counter, primes.get(&counter).unwrap_or(&0) + 1);
            counter = 2;
        } else {
            counter += 1;
        }
    }
    let mut result = String::new();
    let mut keys: Vec<&i64> = primes.keys().collect();
    keys.sort();
    for &key in keys.iter() {
        let value = primes.get(key).unwrap();
        if *value > 1 {
            result.push_str(&format!("({}**{})", key, value));
        } else {
            result.push_str(&format!("({})", key));
        }
    }
    result

}


fn prime_factors_BTree(n: i64) -> String {
    let mut n = n as u64;
    let mut d = 2;
    let mut mem = std::collections::BTreeMap::new();
    while d <= n {
      if n % d == 0 {
        n /= d;
        let old = mem.entry(d).or_insert(0);
        *old += 1;
      } else {
        d += 1;
      }
    }
    mem.iter().map(|(key, val)| match *val {
      1 => format!("({})", key),
      _ => format!("({}**{})", key, val),
    }).collect::<String>()
  }