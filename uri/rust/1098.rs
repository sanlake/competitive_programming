fn main() {
    let mut i: f64;

    for k in 0..=10 { 
        i = k as f64 * 0.2;

        for j in 1..=3 { 
            if i % 1.0 == 0.0 {
                println!("I={} J={}", i,j as f64 + i); 
            } else {
                println!("I={:.1} J={:.1}", i,j as f64 + i); 
            }
        }
    }
}
