fn main() {
    let mut vec = Vec::new();
    let mut merged = Vec::new();
    let mut result: u64 = 0;

    for line in std::io::stdin().lines() {
        let line = line.unwrap();

        if let Some((start, end)) = line.split_once('-') {
            vec.push((start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()));
        }
    }

    vec.sort_unstable_by_key(|(start, _end)| *start);
    
    let (mut start, mut end) = vec[0];
    for (s, e) in vec {
        if s > end {
            merged.push((start, end));
            (start, end) = (s, e);
        } else if e > end {
            end = e;
        }
    }
    merged.push((start, end));
    
    for (start, end) in merged {
        result += end - start + 1;
    }

    println!("{}", result);
}
