#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod template_rag;

#[tokio::main]
async fn main() {
    // MOCK RESPONSES
    let responses = "Nome: Luiz Felipe\nCargo: Software Engineer\nStack: Node.js, NestJS, Next.js, Docker, Rust";

    match template_rag::read_local_template() {
        Ok(template) => {
            println!("Ok!");

            let final_prompt = format!(
                "Você é um especialista em recrutamento técnico.\n\
                 Use o seguinte TEMPLATE OBRIGATÓRIO para estruturar as informações:\n\n\
                 {}\n\n\
                 Aqui estão os dados do candidato:\n\n\
                 {}\n\n\
                 Gere o currículo final formatado estritamente no padrão solicitado:", 
                template, responses
            );

            println!("Sending data to agent. Please wait...");
            
            match template_rag::send_to_agent(final_prompt).await {
                Ok(resume) => {
                    println!("{}", resume);
                },
                Err(e) => {
                    println!("Agent processing failed: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to read template: {}", e);
        }
    }
    
    println!("---------------------------\n");

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("Error starting Tauri application");
}