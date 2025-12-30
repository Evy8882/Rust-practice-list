use rand::Rng;

fn get_number(msg: &str) -> Result<u32, std::num::ParseIntError> {
    println!("{}",msg);
    let mut inp = String::new();
    std::io::stdin()
        .read_line(&mut inp)
        .expect("Falha ao ler entrada");
    inp.trim().parse::<u32>().into()
}

fn generate_random_number(max: u32) -> u32{
    // let mut rng = rand::thread_rng();
    let n = rand::rng().random_range(1..=max);
    n
}

fn main() {
    println!("{}","=".repeat(30));
    println!("Iniciando Jogo de Adivinhação");
    println!("{}","=".repeat(30));

    let max = match get_number("Insira o valor máximo, quanto maior, mais difícil:") {
        Ok(v) => v,
        Err(_) => {
            println!("Número inválido\no número máximo foi definido como 100");
            100
        }
    };

    let r_num = generate_random_number(max);
    let mut guesses = 0;

    println!("Jogo iniciado");
    let mut guess = 0;
    while guess != r_num {
        guess = match get_number("Insira sua tentativa:") {
            Ok(v) => v,
            Err(_) => {println!("Número inválido"); 0}
        };
        if guess > r_num {println!("Número muito ALTO ⬇\n")}
        if guess < r_num {println!("Número muito BAIXO ⬆\n")}
        guesses += 1;
    }
    println!("Você adivinhou, parabéns");
    println!("Tentativas: {}",guesses)



}
