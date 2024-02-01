use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Number guesser");

    // gera o número aleatório no range passado
    let secret = rand::thread_rng().gen_range(1..=100);

    //continua eternamente até o break ou falha
    loop {
        println!("Take a guess");
        //declarando a variavel guess
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading the guess");
        //transforma de string para int u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            //se encontra erro não para o script
            Err(_) => continue,
        };

        println!("Your guess was {guess}");

        //retorna o texto baseado no valor da comparação
        //se muito pequeno ou grande.
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("YOU WIN!!!");
                println!("The secret number was {secret}");
                break;
            }
        }
    }
}
