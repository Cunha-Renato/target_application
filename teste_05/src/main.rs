fn reverse(string: String) -> String {
    let mut result = String::with_capacity(string.capacity());
    for i in (0..string.len()).rev() {
        if let Some(chr) = string.chars().nth(i) {
            result.push(chr)
        }
    }
    
    result
}

fn main() {
    println!("{}", reverse(String::from(".😊 metartnoc em rovaf roP")));
}
