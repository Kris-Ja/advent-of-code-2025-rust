fn main() {
    let mut sum = 0;
    
    for line in std::io::stdin().lines() {
        let s = line.unwrap();
        let mut max_pos = 0;
        let mut max: u8 = 0;

        for (i, &number) in s[..s.len()-1].as_bytes().iter().enumerate() {
            if number > max {
                max = number;
                max_pos = i;
            }
        }

        let mut max2: u8 = 0;

        for &number in s[max_pos+1..].as_bytes() {
            if number > max2 { max2 = number; }
        }

        max -= '0' as u8;
        max2 -= '0' as u8;

        sum += max as i32 * 10 + max2 as i32;
    }
    println!("{}", sum);
}
