trait StrExt {
    fn less_or_eq(&self, number: &str) -> bool;
}

impl StrExt for str {
    fn less_or_eq(&self, number: &str) -> bool {
        if self.len() < number.len() { return true; }
        else if self.len() == number.len() { return self <= number; }
        else { return false; }
    }
}

fn sum_in_range_for_pattern_len(start_str: &str, end_str: &str, pattern_len: usize) -> u128 {
    let mut sum: u128 = 0;
    let len = start_str.len();
    let i = pattern_len;

    let mut number: u128 = 
        if len % i != 0 {
            format!("1{}", "0".repeat(len/i)).parse().unwrap()
        } else {
            if start_str[0..len/i].repeat(i).as_str() >= start_str {
                start_str[0..len/i].parse::<u128>().unwrap()
            } else {
                start_str[0..len/i].parse::<u128>().unwrap() + 1
            }
        };

    while number.to_string().repeat(i).as_str().less_or_eq(end_str) {
        sum += number.to_string().repeat(i).parse::<u128>().unwrap();
        number += 1;
    }
    sum
}

fn sum_in_range(start_str: &str, end_str: &str) -> u128 {
    let mut sum: u128 = 0;
    let primes = [2, 3, 5, 7, 11, 13, 17, 19];

    for i in &primes {
        sum += sum_in_range_for_pattern_len(start_str, end_str, *i);
    }
    for i in &primes {
        for j in &primes {
            if i >= j { continue; }
            if i*j > 38 { break; }
            sum -= sum_in_range_for_pattern_len(start_str, end_str, i * j);
        }
    }
    sum += sum_in_range_for_pattern_len(start_str, end_str, 30);

    sum
}

fn main() {
    let mut sum: u128 = 0;

    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            for s in input.split(',') {
                let (first, second) = s.split_once('-').unwrap();
                sum += sum_in_range(first, second.trim());
            }
        }
        Err(error) => println!("error: {error}"),
    }

    println!("{}", sum);
}
