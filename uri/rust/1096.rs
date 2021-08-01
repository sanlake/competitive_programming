fn main() {
    for i in (1..=9).step_by(2) { 
        for j in 1..=3 { 
            println!("I={} J={}", i, 8-j); 
        }
    }
}
