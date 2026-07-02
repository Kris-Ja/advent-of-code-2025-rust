fn main() {
    let map_line = |line: std::io::Result<String>| line.unwrap().split_whitespace()
        .map(|s| if let Ok(n) = s.parse::<i64>() { n } else { s.as_bytes()[0].into() } ).collect();
    
    let tab: Vec<Vec<i64>> = std::io::stdin().lines().map(|line| map_line(line)).collect();

    let mut result: i64 = 0;

    for j in 0..tab[0].len() {
        let operator = tab[tab.len()-1][j];
        let mut res = tab[0][j];
        for i in 1..tab.len()-1 {
             match operator as u8 {
                 b'*' => res *= tab[i][j],
                 b'+' => res += tab[i][j],
                 _ => println!("Unknown operator: {}", operator),
            }           
        }
        result += res;
    }
    println!("{}", result);
}
