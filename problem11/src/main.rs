use std::fs::read_to_string;
use num_bigint::{BigUint, ToBigUint};

fn main() {
    //Phase 1
    let input: Vec<usize> = read_to_string("input1.txt").unwrap()
        .split('\n').map(|x| x.trim().parse().unwrap())
        .collect();
    let mut output: usize = 0;

    for i in 1..input.len()
    {
        let mut factorial: BigUint = 1.to_biguint().unwrap();
        for f in 2..input[i]+1
        {
            factorial *= f;
        }
        output += factorial.to_string().len();
    }
    println!("Phase 1: {}", output);

    //Phase 2
    let input = read_to_string("input2.txt").unwrap();
    let input: Vec<&str> = input.split('\n').collect();
    let mut output = String::new();

    for i in 1..input.len()
    {
        let line: Vec<usize> = input[i].split(' ').map(|x| x.trim().parse().unwrap()).collect(); //get the 3 integers on the current line
        let mut factorial: BigUint = 1.to_biguint().unwrap();
        for f in 2..line[0]+1
        {
            factorial *= f;
        }
        let factorial = factorial.to_string();
        output += &factorial[(factorial.len() - line[2]) .. (factorial.len() - line[1] + 1)];
    }
    println!("Phase 2: {}", output);
}
