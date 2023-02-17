use std::{collections::HashMap, clone};


fn main() {
    //println!("{}", is_valid_walk(&['n','s','n','s','n','s','n','s','n','s']))}
    //println!("{}", comp(vec![121, 144, 19, 161, 19, 144, 19, 11], vec![11*11, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19]));
    //println!("{}",bouncing_ball(3.0, 0.66, 1.5))
    //println!("{:?}", delete_nth(&[20,37,20,21], 1))
    //println!("{:?}", sum_dig_pow(1, 100))
    //println!("{:?}", find_even_index(&[1, 2, 3, 4, 3, 2, 1]))
    println!("{}",high("man i need a taxi up to ubud"))
}

fn find_short(s: &str) -> u32 {
    let split = s.split_ascii_whitespace();
    let mut min:u32 = 100000000;
    for word in split{
        let size:u32 = word.len() as u32;
        if size < min {
            min = size
        }
    }
    return min
}

fn is_valid_walk(walk: &[char]) -> bool {
    if walk.len() != 10 {
        return false;
    }
    let mut array = [0,0,0,0];
    for c in walk{
        if c == &'n'{
            array[0] += 1;
        }
        if c == &'s'{
            array[1] += 1;
        }
        if c == &'w'{
            array[2] += 1;
        }
        if c == &'e'{
            array[3] += 1;
        }
    }
    if array[0] == array[1] && array[2] == array[3]{
        return true;
    }
    return false;
}

//My version

fn comp(a: Vec<i64>, b: Vec<i64>) -> bool {
    let mut result = b;
    // your code
    for v in a{
        if result.contains(&(v*v)){
            let index = result.iter().position(|x| *x == v*v).unwrap();
            result.remove(index);
        }
    }
    if result.len() ==0{
        return  true;
    }
    return  false;
}

//Best version

fn comp_best(a: Vec<i64>, b: Vec<i64>) -> bool {
    let mut a1 = a.iter().map(|&x| x * x).collect::<Vec<_>>(); 
    let mut a2 = b;
    a1.sort();
    a2.sort();
    a1 == a2
}

fn bouncing_ball(h: f64,  bounce: f64,  window: f64) -> i32 {
    
    if h < 0.0 {
        return -1;
    }

    if bounce >= 1.0 || bounce <= 0.0 {
        return  -1;
    } 

    if window >= h {
        return -1;
    } 

    let mut h1 = h;
    let mut count = 1;

    while h1 > window {
        h1 *= bounce;
        if h1 > window {
            count += 2;
        }
    }

    return count;
}

// Best version

fn bouncing_ball_best(h: f64, bounce: f64, window: f64) -> i32 {
    if !(h > 0. && 0. < bounce && bounce < 1. && window < h) {
        -1
    } else {
        (window / h).log(bounce).ceil() as i32 * 2 - 1
    }
}

fn delete_nth_hashmap(lst: &[u8], n: usize) -> Vec<u8> {
    let max:u8 = n as u8;
    let mut answer = vec![];
    let mut map:HashMap<u8, u8> = HashMap::new();
    for nu in lst{
        if map.contains_key(nu){
            if map.get(nu).unwrap() < &max{
                answer.push(*nu);
                map.insert(*nu, map.get(nu).unwrap()+1);
            }
        } else {
            answer.push(*nu);
            map.insert(*nu,1);
        }
    }
    return answer;
}

fn delete_nth(lst: &[u8], n: usize) -> Vec<u8> {
    let mut answer = vec![];
    for num in lst{
        if !answer.contains(num){
            answer.push(*num)
        } else {
            let mut count = 0;
            for val in answer.clone(){
                if val == *num{
                    count += 1;
                }
            }
            if count<n{
                answer.push(*num);
            }
            
        }   
    }
    return answer;
}

fn delete_nth_best(xs: &[u8], n: usize) -> Vec<u8> {
    let mut ks = [0; u8::MAX as usize + 1];
    xs.iter().cloned()
        .filter(|&x| { ks[x as usize] += 1; ks[x as usize] <= n })
        .collect()
}

fn sum_dig_pow(a: u64, b: u64) -> Vec<u64> {
    let mut answer = vec![];
    for num in a..b+1{
        let mut sum:u64 = 0;
        let temp = num.clone().to_string();
        let digits = temp.chars().map(|d| d.to_digit(10).unwrap());
        let mut counter = 1;
        for i in digits{
            sum += (i as u64).pow(counter);
            counter +=1;
        }
        if sum == num{
            answer.push(sum)
        }
    }
    return answer;
}

fn sum_dig_pow_best(a: u64, b: u64) -> Vec<u64> {
    (a..=b)
        .filter(|x| {
            x.to_string()
                .chars()
                .enumerate()
                .fold(0, |sum: u64, (i, n)| {
                    sum + (n.to_digit(10).unwrap() as u64).pow(i as u32 + 1)
                })
                == *x
        })
        .collect()
}

fn find_even_index(arr: &[i32]) -> Option<usize> {
    if arr.len() == 0{
        return Some(0);
    }
    for i in 0..arr.len(){
        let mut sum_a = 0;
        let mut sum_b = 0;
        for j in 0..arr.len(){
            if (j < i){
                sum_a += arr[j];
            }
            else if j>i{
                sum_b += arr[j];
            }
        }
        if sum_a == sum_b {
            return Some(i);
        }
    }

    return None;
}

fn find_even_index_best(arr: &[i32]) -> Option<usize> {
    let (mut r, mut l) = (arr.iter().sum::<i32>(), 0);
    for (i, n) in arr.iter().enumerate() {
        r -= n;
        if r == l { return Some(i) }
        l += n;
    }
    None
}

fn high(input: &str) -> &str {

    let mut words = input.split_ascii_whitespace();
    let mut sum = 0;
    let mut answer:&str = "";
    for i in words{
        let mut temp:u16 = 0;
        for c in i.as_bytes(){
            temp += (*c as u16 - 96);
        }
        if temp > sum{
            sum = temp;
            answer =i;
        }
    }
    return answer;
    
}

fn high_best(input: &str) -> &str {
    input.split_ascii_whitespace().rev().max_by_key(|s| s.chars().map(|c| c as u16 - 96).sum::<u16>()).unwrap_or("")
}

fn is_solved(board: &[&[u8; 3]; 3]) -> i8 {
    
}