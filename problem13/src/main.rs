use std::time::Instant;

fn main() {
    let now = Instant::now();

    let mut count: u128 = 0;
    let mut rows = 133701337;

    let mut last_exp = (rows as f32).log(7f32).floor() as u32;

    let div = 7;
    let brick: u128 = div * (div + 1) / 2;

    let mut brick_mult = 0;
    let mut add = 1;

    while rows > 0
    {
        let n = (rows as f32).log(7f32).floor() as u32;

        brick_mult += add;

        if n < last_exp
        {
            add = brick_mult;
            last_exp = n;
        }
        
        count += brick.pow(n) * brick_mult;
        rows -= 7u32.pow(n);
    }    

    println!("Answer: {} computed in {:?}", count, now.elapsed());
}

