fn main() {
    let message = "Test";
    println!("{} -> {}", message, rot13(message));

}

fn rot13(message: &str) -> String {
    let upper_case = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<char>>();
    let lower_case = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<char>>();
    let mut result = String::new();

    for c in message.chars() {
        if c.is_alphabetic() {
            if c.is_uppercase() {
                let index = upper_case.iter().position(|&x| x == c).unwrap();
                let new_index = (index + 13) % upper_case.len();
                let new_char = upper_case[new_index];
                result.push(new_char);
            } else {
                let index = lower_case.iter().position(|&x| x == c).unwrap();
                let new_index = (index + 13) % lower_case.len();
                let new_char = lower_case[new_index];
                result.push(new_char);
            }
        } else {
            result.push(c);
        }
    }

    result
}

fn best_rot13(message: &str) -> String {
     message.chars().map(|c| {
        match c {
            'A' ..= 'M' | 'a' ..= 'm' => ((c as u8) + 13) as char,
            'N' ..= 'Z' | 'n' ..= 'z' => ((c as u8) - 13) as char,
             _ => c,
        }
    }).collect()
}