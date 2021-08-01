fn main() {
    for i in (1..=9).step_by(2) { 
        for j in 0+i..3+i { 
            println!("I={} J={}", i, 6+i-j+i); 
        }
    }
}
