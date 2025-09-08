use std::io;

fn main() {
    let input = io::stdin();
    let lines=input
        .lines()
        .map(|_line| _line.ok().unwrap())
        .collect::<Vec<String>>();
    // let input: &'static str = "1 2 3 4 5 6";
    eprintln!("{:?}", lines);
    println!("{}", halfsum(&lines[1]));
}

fn halfsum(input: &str) -> u32 {
    let mut input_vector: Vec<u32> = input
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
        // sort to find largeset number
    input_vector.sort();
    input_vector.reverse();
    // input_vector has structure [1, 2, 3,... ]
    // split input_vector in half:
    let mut new_input: Vec<_> = vec![];
    for i in input_vector
        .iter()
        .take(input_vector.len().div_ceil(2)) {
            new_input.push(i)
    }
    let mut x = 0;
    for i in new_input {
        x += i;
    }
    x
}