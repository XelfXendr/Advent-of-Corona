use std::io::stdin;

fn main() {

    loop
    {
        println!("Enter a message to encrypt: ");
        let mut input: String = String::new();
        match stdin().read_line(&mut input)
        {
            Err(_) => {
                println!("Error while reading input.");
                continue;
            },
            _ => {}
        }

        input.retain(|x| x != ' ');
        input = String::from(input.trim());

        let sqrtlen = (input.len() as f32).sqrt();
        let h = sqrtlen.floor() as usize;
        let w = sqrtlen.ceil() as usize;

        let mut output: Vec<u8> = Vec::new();
        let input = input.into_bytes();

        for i in 0..w
        {
            for j in 0..h
            {
                if j*w+i >= input.len()
                {
                    break;
                }
                output.push(input[j*w+i]);
            }
            if i != w-1
            {
                output.push(b' ');
            }
        }

        println!("Your encrypted message: {}", match String::from_utf8(output)
            {
                Ok(x) => x,
                Err(_) => 
                {
                    println!("Error during encryption. Characters other than UTF-8 have probably been used?");
                    continue;
                }
            });
    }
}
