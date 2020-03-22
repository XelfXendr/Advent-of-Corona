use std::cmp::{min, max};

fn main() {
    //part 1
    let mut cube: [[[u128; 11]; 11]; 11] = [[[0; 11]; 11]; 11];
    cube[0][0][0] = 1;

    for i in 1..31
    {
        for x in max(0, i as i32 - 20) as usize .. min(i, 10)+1
        {
            for y in max(0, (i-x) as i32 - 10) as usize .. min(i-x, 10)+1
            {
                let z = i - x - y;
                if x > 0
                {
                    cube[x][y][z] += cube[x-1][y][z];
                }
                if y > 0
                {
                    cube[x][y][z] += cube[x][y-1][z];
                }
                if z > 0
                {
                    cube[x][y][z] += cube[x][y][z-1];
                }
            }
        }
    }
    println!("Part 1: {}", cube[10][10][10]);

    //part2
    let mut cube: [[[[[u128; 6]; 6]; 6]; 6]; 6] = [[[[[0; 6]; 6]; 6]; 6]; 6];
    cube[0][0][0][0][0] = 1;

    for i in 1..26 as usize
    {
        for x in max(0, i as i32 - 20) as usize .. min(i, 5)+1
        {
            for y in max(0, (i-x) as i32 - 15) as usize .. min(i-x, 5)+1
            {
                for z in max(0, (i-x-y) as i32 - 10) as usize .. min(i-x-y, 5)+1
                {
                    for w in max(0, (i-x-y-z) as i32 - 5) as usize .. min(i-x-y-z, 5)+1
                    {
                        let l = i - x - y - z - w;
                        
                        if x > 0
                        {
                            cube[x][y][z][w][l] += cube[x-1][y][z][w][l];
                        }
                        if y > 0
                        {
                            cube[x][y][z][w][l] += cube[x][y-1][z][w][l];
                        }
                        if z > 0
                        {
                            cube[x][y][z][w][l] += cube[x][y][z-1][w][l];
                        }
                        if w > 0
                        {
                            cube[x][y][z][w][l] += cube[x][y][z][w-1][l];
                        }
                        if l > 0
                        {
                            cube[x][y][z][w][l] += cube[x][y][z][w][l-1];
                        }   
                    }
                }
            }
        }
    }
    println!("Part 2: {}", cube[5][5][5][5][5]);
}
