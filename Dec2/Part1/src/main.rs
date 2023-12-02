fn main() {
    let input = include_str!("input.txt");
    let max: [i32; 3] = [12, 13, 14];
    let mut output: i32 = 0;
    let mut fail: bool = true;
    for line in input.lines() {
        let mut slices_colon: Vec<String> = Vec::new();
        slices_colon = line.split(": ").map(|s| s.to_string()).collect();
        let mut id = slices_colon[0].split(" ").last().unwrap();
        let mut slices_semicolon: Vec<Vec<String>> = slices_colon[1]
            .split("; ")
            .map(|s| s.split(", ").map(|s| s.to_string()).collect())
            .collect();
        'outer: for x in &slices_semicolon {
            for y in x {
                let mut pair: Vec<String> = y.split(" ").map(|s| s.to_string()).collect();

                if (pair[1] == "red" && pair[0].parse::<i32>().unwrap() <= max[0]) {
                    fail = false;
                } else if (pair[1] == "green" && pair[0].parse::<i32>().unwrap() <= max[1]) {
                    fail = false;
                } else if (pair[1] == "blue" && pair[0].parse::<i32>().unwrap() <= max[2]) {
                    fail = false;
                } else {
                    fail = true;
                    break 'outer;
                }
            }
        }
        if fail {
            continue;
        } else {
            output += id.parse::<i32>().unwrap();
        }
    }
    println!("{}", output)
}
