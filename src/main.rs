use std::io::{stdin, stdout, Write};
fn main() {
    let mut s = String::new();
    let mut sum = 0;
    let  mut i = 10;
    let mut flag = true;
    println!("Inserisci ISBN da verificare:");
    stdin().read_line(&mut s).unwrap();
    let v: Vec<&str> = s.trim().split("-").collect();
    let mut boh: Vec<char> = Vec::new();
    for str in &v{
        for token in str.chars(){
            boh.push(token);
        }
    }
    if boh.len() == 10 {
        for token in boh {
            match token {
                'X' => {
                    if i == 1 {
                        sum = sum + 10;
                    } else {
                        flag = false;
                    }
                },
                x if x.is_digit(10) => sum += x.to_digit(10).unwrap() * i ,
                _ => flag = false ,
            }
            i-=1;
        }
    }
    else{
        flag = false
    }
    if sum % 11 == 0 && flag == true{
        println!("ISBN valido");
    }
    else{
        println!("ISBN non valido");
    }
}
