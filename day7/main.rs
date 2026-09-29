fn fold_lines((res, arr): (i64, Vec<u8>), line: Vec<u8>) -> (i64, Vec<u8>) {
    let count = (0..line.len()).fold(0, |c, i| if arr[i] == b'S' && line[i] == b'^' { c + 1 } else { c });
    ( res + count, (0..line.len()).map(|i| {
        if line[i] == b'.' && arr[i] == b'S' { b'S' } 
        else if line[i] == b'.' && i > 0 && line[i-1] == b'^' && arr[i-1] == b'S' { b'S' }
        else if line[i] == b'.' && i+1 < line.len() && line[i+1] == b'^' && arr[i+1] == b'S' { b'S' }
        else { b'.' }
        }
        ).collect()
    )
}

fn main() {
    let mut lines = std::io::stdin().lines();
    let first = lines.next().unwrap().unwrap().into_bytes();
    let result = lines
        .map(|line| line.unwrap().into_bytes())
        .fold((0, first), fold_lines);
    
    println!("{}", result.0)
}
