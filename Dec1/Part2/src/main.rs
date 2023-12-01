use std::collections::HashMap;

fn main() {
    let translations = HashMap::from([
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut output = 0;
    let input = include_str!("input.txt");
    for line in input.lines() {
        let mut vect: Vec<i128> = Vec::new();
        let mut temp = String::new();
        for ch in line.chars() {
            if ch.is_numeric() {
                vect.push(ch.to_digit(10).unwrap() as i128);
                continue;
            } else {
                temp.push(ch);
                for key in translations.keys() {
                    if temp.contains(key) {
                        vect.push(translations[key] as i128);
                        let last = temp.chars().last().unwrap();
                        temp = String::new();
                        temp.push(last);
                    }
                }
            }
        }
        output += vect[0] * 10 + vect[vect.len() - 1];
        vect.clear();
    }
    println!("{:?}", output);
}
