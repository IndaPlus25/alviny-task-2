use std::io;

fn main() {
    let input = io::stdin();
    let lines=input
        .lines()
        .map(|_line| _line.ok().unwrap())
        .collect::<Vec<String>>();
    // let input: &'static str = "1 2 3 4 5 6";
    //println!("{:?}", lines);
    if lines.len() > 1 {
        println!("{}", halfsum(&lines[1]));
    } else {
        println!("0")
    }
}

fn halfsum(input: &str) -> i64 {
    let mut temp/* : Vec<i64>*/ = input
        // convert input into a vector of signed 64bit integers AND NOTHING ELSE
        .split(' ')
        .collect::<Vec<&str>>();
    temp.retain(|x| !x.is_empty());

    let mut input_vector: Vec<i64> = temp
        .iter()
        .map(|x| x.parse::<i64>().ok().unwrap())
        .collect::<Vec<i64>>();
        // sort to find largest number
    input_vector.sort();
    input_vector.reverse();
    // input_vector has structure [1, 2, 3,... ]
    // split input_vector in half:
    let mut new_input: Vec<_> = vec![];
    for i in input_vector
        .iter()
        .take(input_vector.len().div_ceil(2)) {
            new_input.push(i)
    };
    //println!("{:?}", input_vector);
    let mut x = 0;
    for i in new_input {
        x += i;
    }
    x
}