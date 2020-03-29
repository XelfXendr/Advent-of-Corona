use std::fs::read_to_string;
use std::str;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let input: Vec<String> = input.trim().split('\n').map(|x| 
        {
            let mut a = String::from(x);
            a.retain(|c| c != '\r');
            a
        }).collect();

    let mut offset: usize = 1;
    let cases: usize = input[0].parse().unwrap();
    let specials = [b'!', b'@', b'#', b'$', b'%', b'&', b' '];
    let mut output: String = String::new();
    for _c in 0..cases
    {
        //matrix to string
        let size: Vec<&str> = input[offset].split(' ').collect();
        let height: usize = size[0].parse().unwrap();
        let width: usize = size[1].parse().unwrap();
        offset += 1;

        let mut text: Vec<u8> = Vec::new(); 
        for x in 0..width
        {
            for y in offset..offset+height
            {
                text.push(input[y].as_bytes()[x]);
            }
        }

        //remove unnecessary characters
        let mut i: i32 = 0;
        let mut seenfirst = false;
        while (i as usize) < text.len()
        {
            if i as usize == text.len() - 1
            {
                break;
            }

            if specials.contains(&text[i as usize])
            {
                if !seenfirst
                {
                    i += 1;
                    continue;
                }
                if !specials.contains(&text[(i+1) as usize])
                {
                    text[i as usize] = b' ';
                }
                else
                {
                    for g in (i+1) as usize..text.len()
                    {
                        if !specials.contains(&text[g])
                        {
                            text.remove(i as usize);
                            i -= 1;
                            break;
                        }
                    }
                }
            }
            else
            {
                seenfirst = true;
            }
            
            i += 1;
        }
        let text = String::from_utf8(text).unwrap() + ",";
        output += &text;
        offset += height;
    }
    //save output to file
    let mut file = File::create("output.txt").unwrap();
    file.write_all(&(output.as_bytes()[0..output.len()-1])).unwrap();
}
