fn remove_rolls(tab: &mut Vec<Vec<bool>>) -> i32 {
    let mut result = 0;

    for i in 0..tab.len() {
        for j in 0..tab[0].len() {
            if tab[i][j] {
                let mut count = 0;

                if i > 0            && j > 0                && tab[i-1][j-1]    { count += 1; } 
                if i > 0                                    && tab[i-1][ j ]    { count += 1; } 
                if i > 0            && j < tab[0].len()-1   && tab[i-1][j+1]    { count += 1; } 
                
                if true             && j > 0                && tab[ i ][j-1]    { count += 1; } 
                if true             && j < tab[0].len()-1   && tab[ i ][j+1]    { count += 1; } 
                
                if i < tab.len()-1  && j > 0                && tab[i+1][j-1]    { count += 1; } 
                if i < tab.len()-1                          && tab[i+1][ j ]    { count += 1; } 
                if i < tab.len()-1  && j < tab[0].len()-1   && tab[i+1][j+1]    { count += 1; } 
                
                if count < 4 {
                    result += 1;
                    tab[i][j] = false;
                }
            }
        }
    }
    result
}

fn main() {
    let map_line = |line: std::io::Result<String>| line.unwrap().as_bytes().iter()
    .map(|&c| if c == b'.' { false } else { true } ).collect();
    
    let mut tab: Vec<Vec<bool>> = std::io::stdin().lines().map(|line| map_line(line)).collect();

    let mut result = 0;
    
    loop {
        let removed = remove_rolls(&mut tab);
        result += removed;
        if removed == 0 { break; }
    }

    println!("{}", result);
}
