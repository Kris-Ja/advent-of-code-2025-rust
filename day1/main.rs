fn main() {
    let mut dial = 50;
    let mut counter = 0;
    
    for line in std::io::stdin().lines() {

        let s = line.unwrap();

        let number: i32 = s[1..].trim().parse().unwrap(); 
        
        if s.starts_with("L") { dial -= number; }
        if s.starts_with("R") { dial += number; }
        
        dial %= 100;

        if dial == 0 { counter += 1; }
    }
    println!("{}", counter);
}
