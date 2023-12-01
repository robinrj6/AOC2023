fn main() {
    let mut sum: Vec<i128> = Vec::new();
    let input = include_str!("input.txt");
    for line in input.lines() {
        let mut vect: Vec<i128> = Vec::new();
        
        for ch in line.chars() {
            if ch.is_numeric() {
                vect.push(ch.to_digit(10).unwrap() as i128);
            }
        }
        let first = vect[0];
        let last = vect[vect.len() - 1];
        if vect.len() == 0 {
            continue;
        } else {
            let value = (first * 10) + last;
            sum.push(value)
        }
    }
    let total: i128 = sum.iter().sum();
    println!("Total: {}", total);
}
