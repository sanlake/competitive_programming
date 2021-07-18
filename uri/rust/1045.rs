fn get_float_vec() -> Vec<f64> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    let split = buffer.split_whitespace();
    let vec: Vec<&str> = split.collect();
    vec.iter().map(|x| x.trim().parse::<f64>().unwrap()).collect()
}

fn main() {
    let mut l: Vec<f64> = get_float_vec();

    l.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let fi = (l[2]).powf(2.0);
    let la = (l[1]).powf(2.0) + (l[0]).powf(2.0);

    if l[2] >= l[1]+l[0] {
        println!("NAO FORMA TRIANGULO");
    } else {
        if fi == la {
            println!("TRIANGULO RETANGULO");	
        } else if fi>la {
            println!("TRIANGULO OBTUSANGULO");	
        } else {
            println!("TRIANGULO ACUTANGULO");
        }
        
        let (mut q, mut bi): (i64, i64);

        bi=0;

        for x in l.iter() {
            q = l.iter().filter(|&n| *n == *x).count() as i64;
            if q>bi { bi = q; }
        }

        if bi == 3 {
            println!("TRIANGULO EQUILATERO");
        } else if bi==2 {
            println!("TRIANGULO ISOSCELES");
        }
    }
}
