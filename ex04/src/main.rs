fn get_number(msg: &str) -> Result<i32, std::num::ParseIntError> {
    println!("{}", msg.to_string());
    let mut inp = String::new();
    std::io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read line");
    inp.trim().parse::<i32>().into()
}

fn main() {
    match get_number("Insira o número inicial") {
        Ok(start_num) => match get_number("Insira o número final") {
            Ok(end_num) => match get_number("Insira a quantidade de passos") {
                Ok(steps_num) => {
                    let pos_steps_num = steps_num.abs();
                    if pos_steps_num == 0 {
                        println!("Os passos não podem ser 0")
                    } else {
                        let mut i = start_num;
                        if i <= end_num {
                            while i <= end_num {
                                println!("{}", i);
                                i += pos_steps_num
                            }
                        } else {
                            while i >= end_num {
                                println!("{}", i);
                                i -= pos_steps_num
                            }
                        }
                    }
                }
                Err(_) => println!("Por favor, insira apenas valores válidos"),
            },
            Err(_) => println!("Por favor, insira apenas valores válidos"),
        },
        Err(_) => println!("Por favor, insira apenas valores válidos"),
    }
}
