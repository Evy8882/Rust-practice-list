use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};

#[derive(Serialize, Deserialize)]
struct TodoItem {
    id: u32,
    description: String,
    done: bool,
}

fn get_json_data() -> Option<Vec<TodoItem>> {
    let file_content = match std::fs::read_to_string("todo.json") {
        Ok(v) => v,
        Err(_) => String::new(),
    };
    let todo_list = from_str::<Vec<TodoItem>>(&file_content).ok()?;
    Some(todo_list)
}

fn main() {
    loop {
        println!("{}", "=".repeat(50));
        println!("LISTA DE TAREFAS");
        println!("{}", "=".repeat(50));

        let mut current_data: Vec<TodoItem> = match get_json_data() {
            Some(v) => v,
            None => vec![],
        };

        for i in &current_data {
            println!("{} - {}", i.done, i.description);
        }

        println!("{}", "=".repeat(50));
        println!("AÇÕES (digite apenas o número)");
        print!("{}", "=".repeat(50));
        println!(
            "
            1 - Adicionar Tarefa
            2 - (Des)Marcar como concluída
            3 - Excluir tarefa
            4 - Sair"
        );

        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Erro ao ler entrada");

        println!("{}", "=".repeat(50));
        match choice.trim() {
            "1" => {
                println!("Escreva qual será a nova tarefa:");
                let mut new_task = String::new();
                std::io::stdin()
                    .read_line(&mut new_task)
                    .expect("Erro ao ler entrada");
                let id: u32 = current_data.len() as u32;
                let new_task = new_task.trim().to_string();
                current_data.push(TodoItem {
                    id: id,
                    description: new_task,
                    done: false,
                });
            }
            "2" => {}
            "3" => {}
            "4" => {
                break;
            }
            _ => {
                println!("Opção inválida!");
            }
        }
        let new_json_data: String = to_string_pretty(&current_data).expect("Falha ao salvar dados");
        std::fs::write("todo.json", new_json_data).expect("Erro ao salvar arquivo");
    }
}
