use std::fs::read_to_string;
use std::collections::{VecDeque};
use std::io::stdin;

fn main() {
    const N: usize = 100;

    println!("Starting column: ");
    let mut console_in: String = String::new();
    stdin().read_line(&mut console_in).expect("Error while reading console.");
    let sx: usize = console_in.trim().parse().expect("Error while parsing.");
    println!("Starting row: ");
    console_in = String::new();
    stdin().read_line(&mut console_in).expect("Error while reading console.");
    let sy: usize = console_in.trim().parse().expect("Error while parsing.");
    println!("Input map file: ");
    let mut input_file = String::new();
    stdin().read_line(&mut input_file).expect("Error while reading console.");

    let input = read_to_string(input_file.trim()).expect("Error during reading.");
    let input: Vec<&str> = input.trim().split('\n').map(|x| x.trim()).collect();
    let mut map: [[Cell; N]; N] = [[Cell {discovered: false, blocked: false, is_c: false, distance: 0, hop: 0}; N]; N];

    for j in 0..100
    {
        let line = input[j].as_bytes();
        for i in 0..100
        {
            match line[i] {
                b'X' => map[i][j].blocked = true,
                b'I' => 
                {
                    map[i-1][j].blocked = true;
                    map[i][j-1].blocked = true;
                    map[i+1][j].blocked = true;
                    map[i][j+1].blocked = true;

                    map[i][j].blocked = true;
                },
                b'C' => map[i][j].is_c = true,
                b'H' => map[i][j].is_c = true,
                b'U' => map[i][j].hop = 1,
                b'D' => map[i][j].hop = 2,
                b'L' => map[i][j].hop = 3,
                b'R' => map[i][j].hop = 4,
                _ => {}
            }
        }
    }

    //BFS
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    map[sx][sy].discovered = true;
    q.push_back((sx, sy));

    while q.len() > 0
    {
        let (cx, cy) = q.pop_front().unwrap();

        //found target
        if map[cx][cy].is_c
        {
            println!("Found target at distance {}, position {}, {}", map[cx][cy].distance, cx, cy);
            return;
        }
        //discover neighbours
        if cx > 0 && !map[cx-1][cy].discovered && !map[cx-1][cy].blocked
        {
            q.push_back((cx - 1, cy));
            map[cx-1][cy].distance = map[cx][cy].distance + 1;
            map[cx-1][cy].discovered = true;
        }
        if cx < 99 && !map[cx+1][cy].discovered && !map[cx+1][cy].blocked
        {
            q.push_back((cx + 1, cy));
            map[cx+1][cy].distance = map[cx][cy].distance + 1;
            map[cx+1][cy].discovered = true;
        }
        if cy > 0 && !map[cx][cy-1].discovered && !map[cx][cy-1].blocked
        {
            q.push_back((cx, cy - 1));
            map[cx][cy-1].distance = map[cx][cy].distance + 1;
            map[cx][cy-1].discovered = true;
        }
        if cy < 99 && !map[cx][cy+1].discovered && !map[cx][cy+1].blocked
        {
            q.push_back((cx, cy + 1));
            map[cx][cy+1].distance = map[cx][cy].distance + 1;
            map[cx][cy+1].discovered = true;
        }

        //shortcuts
        match map[cx][cy].hop
        {
            1 => if !map[cx][cy-3].blocked && !map[cx][cy-3].discovered
            {
                q.push_back((cx, cy - 3));
                map[cx][cy-3].distance = map[cx][cy].distance + 1;
                map[cx][cy-3].discovered = true;
            },
            2 => if !map[cx][cy+3].blocked && !map[cx][cy+3].discovered
            {
                q.push_back((cx, cy + 3));
                map[cx][cy+3].distance = map[cx][cy].distance + 1;
                map[cx][cy+3].discovered = true;
            },
            3 => if !map[cx-3][cy].blocked && !map[cx-3][cy].discovered
            {
                q.push_back((cx - 3, cy));
                map[cx - 3][cy].distance = map[cx - 3][cy].distance + 1;
                map[cx - 3][cy].discovered = true;
            },
            4 => if !map[cx + 3][cy].blocked && !map[cx +3][cy].discovered
            {
                q.push_back((cx+3, cy));
                map[cx+3][cy].distance = map[cx+3][cy].distance + 1;
                map[cx+3][cy].discovered = true;
            },
            _ => {}
        }
    }
    println!("Haven't found any target");
}

#[derive(Clone, Copy)]
struct Cell
{
    discovered: bool,
    blocked: bool,
    is_c: bool,
    distance: u32,
    hop: u8,
}
