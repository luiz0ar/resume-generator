#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::{self, BufRead, Write};

mod template_rag;

fn collect_responses() -> String {
    let questions = [
        "Nome completo e objetivo profissional",
        "Experiencia profissional",
        "Stack",
        "Experiencia Academica",
    ];

    let stdin = io::stdin();
    let mut responses = String::new();

    for question in &questions {
        print!("\n{}: ", question);
        io::stdout().flush().expect("Failed to flush stdout");

        let answer = stdin
            .lock()
            .lines()
            .next()
            .expect("Failed to read line")
            .expect("Failed to parse line");

        responses.push_str(&format!("{}: {}\n", question, answer));
    }

    responses
}

#[tokio::main]
async fn main() {
    let responses = collect_responses();

    match template_rag::read_local_template() {
        Ok(template) => {
            let final_prompt = format!(
                "Você é um especialista em recrutamento técnico.\n\
                 Use o seguinte TEMPLATE OBRIGATÓRIO para estruturar as informações:\n\n\
                 {}\n\n\
                 Aqui estão os dados do candidato:\n\n\
                 {}\n\n\
                 Gere o currículo final formatado em lateX e estritamente no padrão solicitado:",
                template, responses
            );

            println!("\nSending data to agent. Please wait...");

            match template_rag::send_to_agent(final_prompt).await {
                Ok(resume) => println!("{}", resume),
                Err(e) => println!("Agent processing failed: {}", e),
            }
        }
        Err(e) => println!("Failed to read template: {}", e),
    }

    println!("---------------------------\n");

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("Error starting Tauri application");
}
