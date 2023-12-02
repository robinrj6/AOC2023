fn main() {
    let input = include_str!("input.txt");
    let max: [i32; 3] = [12, 13, 14];
    let mut output: i32 = 0;
    let mut min: [i32; 3] = [0, 0, 0];

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
                let pair: Vec<String> = y.split(" ").map(|s| s.to_string()).collect();

                if (pair[1] == "red" && pair[0].parse::<i32>().unwrap() > min[0]) {
                    min[0] = pair[0].parse::<i32>().unwrap();
                } else if (pair[1] == "green" && pair[0].parse::<i32>().unwrap() > min[1]) {
                    min[1] = pair[0].parse::<i32>().unwrap();
                } else if (pair[1] == "blue" && pair[0].parse::<i32>().unwrap() > min[2]) {
                    min[2] = pair[0].parse::<i32>().unwrap();
                } else {
                }
            }
        }
        output += (min[0] * min[1] * min[2]);
        min = [0, 0, 0];
    }
    println!("{}", output)
}
