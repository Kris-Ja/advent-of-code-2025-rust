fn main() {
    let mut dial = 50;
    let mut counter = 0;
    
    for line in std::io::stdin().lines() {

        let s = line.unwrap();

        let number: i32 = s[1..].trim().parse().unwrap(); 
        
        if s.starts_with("L") {
            if dial == 0 { dial = 100; }
            dial -= number;
        }
        if s.starts_with("R") { dial += number; }
        
        if dial == 0 { counter += 1; }
        counter += dial.div_euclid(100).abs();
        if dial%100 == 0 && dial < 0 { counter += 1; }
        
        dial = dial.rem_euclid(100);
    }
    println!("{}", counter);
}
