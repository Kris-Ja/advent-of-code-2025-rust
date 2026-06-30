fn main() {
    let mut vec = Vec::new();
    let mut merged = Vec::new();
    let mut is_merged = false;
    let mut result = 0;

    for line in std::io::stdin().lines() {
        let line = line.unwrap();

        if let Some((start, end)) = line.split_once('-') {
            vec.push((start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()));
        } else {
            if !is_merged {
                vec.sort_unstable_by_key(|(start, _end)| *start);
                
                let (mut start, mut end) = vec[0];
                for &(s, e) in &vec {
                    if s > end {
                        merged.push((start, end));
                        (start, end) = (s, e);
                    } else if e > end {
                        end = e;
                    }
                }
                merged.push((start, end));

                is_merged = true;
            }

            if let Ok(number) = line.parse::<u64>() {
                let i = merged.partition_point(|(start, _end)| number >= *start);
                if i != 0 {
                    let (_, end) = merged[i-1];
                    if number <= end { result += 1; }
                }
            }
        }
    }
    
    println!("{}", result);
}
