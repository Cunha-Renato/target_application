const NUMBER: u32 = 34;

fn fibonacci(number: u32) -> bool {
    let mut num1 = 0;
    let mut num2 = 1;
    let mut result = false;

    if number <= 1 { return true; }

    while num2 < number {
        let aux = num2;
        num2 = num1 + num2;                
        num1 = aux;
        
        result = num2 == number;
    }
    
    result
}

fn main() {
    println!("Numero: {NUMBER}.");
    println!("Resultado: {}", fibonacci(NUMBER));
}
