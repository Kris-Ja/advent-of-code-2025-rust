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

fn sum_in_range(start_str: &str, end_str: &str) -> u128 {
    let mut sum: u128 = 0;
    let len = start_str.len();
    let mut number: u128 = 
        if len % 2 == 1 {
            // ("1".to_owned() + &"0".repeat(len)).parse().unwrap()
            format!("1{}", "0".repeat(len/2)).parse().unwrap()
        } else {
            if start_str[0..len/2] >= start_str[len/2..len] {
                start_str[0..len/2].parse::<u128>().unwrap()
            } else {
                start_str[0..len/2].parse::<u128>().unwrap() + 1
            }
        };

    while number.to_string().repeat(2).as_str().less_or_eq(end_str) {
        sum += number.to_string().repeat(2).parse::<u128>().unwrap();
        number += 1;
    }
    
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
