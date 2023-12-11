
fn main() {
    println!("{}", rgb(255, 255, 255));
    println!("{}", rgb_best(214, 0, 25));
}

fn rgb(r: i32, g: i32, b: i32) -> String {
    let mut hex = String::new();
    let mut r = r;
    let mut g = g;
    let mut b = b;
    if r > 255 {
        r = 255;
    } else if r < 0 {
        r = 0;
    }
    if g > 255 {
        g = 255;
    } else if g < 0 {
        g = 0;
    }
    if b > 255 {
        b = 255;
    } else if b < 0 {
        b = 0;
    }
    hex.push_str(&hex_transformation(r));
    hex.push_str(&hex_transformation(g));
    hex.push_str(&hex_transformation(b));
    hex
}

fn hex_transformation(num: i32) -> String {
    let mut hex = String::new();
    let mut num = num;
    let mut remainder;
    let mut quotient;
    let mut hex_map = std::collections::HashMap::new();
    hex_map.insert(10, "A");
    hex_map.insert(11, "B");
    hex_map.insert(12, "C");
    hex_map.insert(13, "D");
    hex_map.insert(14, "E");
    hex_map.insert(15, "F");
    let mut count = 0;
    while count < 2{
        remainder = num % 16;
        quotient = num / 16;
        if remainder > 9 {
            hex.push_str(hex_map.get(&remainder).unwrap());
        } else if remainder == 0 {
            hex.push('0');
        }
        else {
            hex.push_str(&remainder.to_string());
        }
        num = quotient;
        count += 1;
    }
    hex.chars().rev().collect::<String>()
}

fn rgb_best(r: i32, g: i32, b: i32) -> String {
    format!(
        "{:02X}{:02X}{:02X}",
        r.clamp(0, 255),
        g.clamp(0, 255),
        b.clamp(0, 255)
    ) 
}