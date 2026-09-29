fn main() {
    let tab: Vec<Vec<u8>> = std::io::stdin()
        .lines()
        .map(|line| line.unwrap().as_bytes().to_vec())
        .collect();

    let mut result: i64 = 0;
    let mut res = 0;
    let mut op = b' ';

    let nums : Vec<Vec<Option<i64>>> = tab[..tab.len()-1].iter().map(|row|
        row.iter().map(|c|
            if *c == b' ' { None }
            else { Some((c - b'0') as i64) }
            )
            .collect()
        ).collect();

    let numbers : Vec<Option<i64>> = (0..tab[0].len())
        .map(|j| {
            nums.iter().filter_map(|row| row[j])
                .fold(None, |acc, v| Some(match acc { None => v, Some(x) => x * 10 + v,} ))
        }).collect();

    for j in 0..tab[0].len() {

        let operator = tab[tab.len()-1][j];
        match operator as u8 {
             b'*' => { result += res; res = 1; op = operator as u8; },
             b'+' => { result += res; res = 0; op = operator as u8; },
             _ => {}
        }
        match numbers[j] {
            None => {},
            Some(number) => res = match op {
                b'*' => res * number,
                b'+' => res + number,
                _ => res,
            },
        }
    }
    
    result += res;
    println!("{}", result);
}
