fn get_int_vec() -> Vec<i64> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    let split = buffer.split_whitespace();
    let vec: Vec<&str> = split.collect();
    vec.iter().map(|x| x.trim().parse::<i64>().unwrap()).collect()
}

fn main() {
    let v: Vec<i64> = get_int_vec();

    let (hi,mi,mut hf,mut mf): (i64,i64,i64,i64) = (v[0],v[1],v[2],v[3]);

    if hf < hi { hf+=24; }
    if hf == hi && mf <= mi { hf+=24; }
    if mf < mi { hf-=1; mf+=60; }

    println!("O JOGO DUROU {} HORA(S) E {} MINUTO(S)", hf-hi, mf-mi);
}
