/// O resultado será de 91;

fn main() {
    let indice = 13;
    let mut sum = 0;
    let mut k = 0;
    
    while k < indice {
        k += 1;
        sum += k;
    }
    
    println!("{sum}");
}