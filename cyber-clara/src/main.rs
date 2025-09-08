/***
 * Template to a Kattis solution.
 * See: https://open.kattis.com/help/rust
 * Author: Viola Söderlund <violaso@kth.se>
 * Edit: Benjamin Widman <bwidman@kth.se>
 */

use std::io;

/// Kattis calls main function to run your solution.
fn main() {
    // get standard input stream
    let input = io::stdin();

    // Get input lines as a vector of strings
    // see: https://doc.rust-lang.org/std/io/trait.BufRead.html
    let lines = input.lines()
        .map(|_line| _line.ok().unwrap())
        .collect::<Vec<String>>();          // Converts iterator into vector. Not necessary, see example solution.
    // The map acts on every element in the iterator, getting the value inside the returned Result, assuming the result is Ok(value) and not Err(error_message).
    // ok() returns an Option, so we unwrap it to get the value inside.
    
    //split the vector into names & surnames using the length variable
    
    let length = lines[0].parse::<usize>().unwrap();
    // turns the first element of the vector into a usize



    let forenames = &lines[1..length+1];
    let surnames  = &lines[length+1..];
    // assert_eq!(forenames.len(), surnames.len());
    // assert_eq!(forenames.len(), length); // making sure both lists' lengths are equal to the length variable
    let mut unique_full_names: Vec<String> = vec![];
    for i in 0..length {
        let full_name= format!("{}{}{}", forenames[i], ' ', surnames[i]);
        if !unique_full_names.contains(&full_name) {
            unique_full_names.push(full_name);
        }
    }
    println!("{}", unique_full_names.len());
}