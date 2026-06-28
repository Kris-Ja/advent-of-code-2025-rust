fn max_joltage(s: &str) -> u64 {
    let mut max_pos: i32 = -1;
    let mut sum: u64 = 0;

    for numbers_left in (0..=11).rev() {
        
        let mut max: u8 = 0;

        for i in (max_pos+1) as usize .. s.len()-numbers_left {
            if s.as_bytes()[i] > max {
                max = s.as_bytes()[i];
                max_pos = i as i32;
            }
        }

        max -= '0' as u8;
        sum = sum * 10 + max as u64;
    }
    sum
}

fn main() {
    let mut sum: u64 = 0;
    
    for line in std::io::stdin().lines() {
        sum += max_joltage(&line.unwrap());
    }
    println!("{}", sum);
}
