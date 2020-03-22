use std::io;
use std::fs;
use std::cmp;

fn main() {
    //Get input
    let mut sequence: Vec<i32>;
    println!("Enter input file:");
    loop
    {
        let input: String;
        loop
        {
            //Get file name
            let mut filename = String::new();
            loop
            {
                match io::stdin().read_line(&mut filename)
                {
                    Ok(_) => break,
                    Err(_) => println!("Invalid input, try again:"),
                };
            }

            //Read input file
            match fs::read_to_string(filename.trim())
            {
                Ok(s) => {
                    input = s;
                    break;
                },
                Err(_) => println!("File couldn't be read, try again:"),
            };
        }

        //Turn input into Vec<i32>
        let mut error: bool = false;
        sequence = input.trim().split(' ')
            .map(|x| match x.parse::<i32>() 
            {
                Ok(n) => n,
                Err(_) => 
                {
                    println!("Error encountered during parsing of {}", x);
                    error = true;
                    0
                }
            }).collect();
        if !error 
        {
            break;
        }
        else
        {
            println!("Try again:");
        };
    }
    
    //Solving Problem 1 Phase 1
    let mut count: i32 = 0; //length of current sequence
    let mut longest: i32 = 0; //highest length found
    let mut last: i32 = sequence[0] - 1; //last element

    for n in sequence.iter()
    {
        if (last & 1 != n & 1) && last < *n
        {
            count += 1;
        }
        else 
        {
            longest = cmp::max(longest, count);
            count = 1;
        }
        last = *n;
    }
    longest = cmp::max(longest, count);

    println!("Longest sequence is {}", longest);
}
