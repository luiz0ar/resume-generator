# 📄 Resume Generator

> A local AI-powered resume generator built with **Rust**, **Tauri**, and **Ollama**. Collects candidate information through an interactive CLI, feeds it into a local LLM alongside a structured template, and outputs a LaTeX-formatted resume — fully offline, no external APIs.

[![CI Rust](https://github.com/luiz0ar/resume-generator/actions/workflows/ci.yml/badge.svg)](https://github.com/luiz0ar/resume-generator/actions/workflows/ci.yml)

---

## 🏗️ Architecture

```
┌─────────────────────────────────────┐
│           Tauri Application         │
│  ┌─────────────┐  ┌──────────────┐  │
│  │  CLI Prompt │  │ template_rag │  │
│  │  (main.rs)  │→ │    module    │  │
│  └─────────────┘  └──────┬───────┘  │
└─────────────────────────-│----------┘
                           │ HTTP (port 11434)
                    ┌──────▼───────┐
                    │    Ollama    │
                    │ qwen2.5:7b  │
                    └─────────────┘
```

- **`main.rs`** — Collects user input via CLI questions and assembles the final prompt
- **`template_rag.rs`** — Reads the local resume template (`resume.md`) and sends the prompt to the Ollama agent
- **`resume.md`** — Markdown template that defines the expected resume structure
- **Docker Compose** — Orchestrates the Rust dev environment and the Ollama LLM service

---

## 🚀 Getting Started

### Prerequisites

- [Docker](https://www.docker.com/) and Docker Compose
- [Rust](https://www.rust-lang.org/tools/install) (for local development without Docker)

### Running with Docker Compose

```bash
# Clone the repository
git clone https://github.com/luiz0ar/resume-generator.git
cd resume-generator

# Start Ollama and the dev environment
docker compose up -d

# Pull the LLM model inside the Ollama container
docker exec -it resume_ollama ollama pull qwen2.5:7b

# Enter the dev container and run the application
docker exec -it resume_rust_dev bash
cargo run
```

### Running Locally (without Docker)

> **Note:** Ollama must be running locally on port `11434`.

```bash
# Install Ollama: https://ollama.com
ollama pull qwen2.5:7b
ollama serve

# In another terminal, run the app
cd src-tauri
cargo run
```

---

## 💬 How It Works

When you run the application, the CLI will prompt you with 4 questions:

```
Nome completo e objetivo profissional: ...
Experiencia profissional: ...
Stack: ...
Experiencia Academica: ...
```

After answering, the app:
1. Reads the resume template from `src-tauri/resume.md`
2. Builds a prompt combining the template + your answers
3. Sends it to the local `qwen2.5:7b` model via Ollama's REST API
4. Prints the generated LaTeX-formatted resume to stdout

---

## 🛠️ Tech Stack

| Layer | Technology |
|---|---|
| Application framework | [Tauri](https://tauri.app/) v1.5 |
| Language | [Rust](https://www.rust-lang.org/) 2021 edition |
| Async runtime | [Tokio](https://tokio.rs/) |
| HTTP client | [Reqwest](https://docs.rs/reqwest) |
| JSON | [Serde JSON](https://docs.rs/serde_json) |
| LLM | [Ollama](https://ollama.com/) — `qwen2.5:7b` |
| Containerization | Docker + Docker Compose |
| CI | GitHub Actions |

---

## 📁 Project Structure

```
resume-generator/
├── .docker/
│   └── Dockerfile.dev        # Dev container (Ubuntu 22.04 + Rust + Node.js)
├── .github/
│   └── workflows/
│       └── ci.yml            # CI: cargo check + cargo fmt
├── src-tauri/
│   ├── src/
│   │   ├── main.rs           # Entry point — CLI questions & Tauri setup
│   │   └── template_rag.rs   # Template reader & Ollama HTTP client
│   ├── resume.md             # Resume structure template
│   ├── Cargo.toml            # Rust dependencies
│   └── tauri.conf.json       # Tauri configuration
├── docker-compose.yml        # Ollama + dev environment services
└── README.md
```

---

## 📝 Customizing the Resume Template

Edit `src-tauri/resume.md` to change the structure the LLM will follow:

```markdown
# TEMPLATE DE CURRÍCULO PADRÃO
Estrutura sugerida para tecnologia:
- Dados Pessoais (Nome, Cargo, Contato)
- Resumo Profissional (Curto e focado)
- Experiências Profissionais (Foco em conquistas e stack técnica)
- Competências Técnicas
```

The template is injected directly into the LLM prompt, so the more detailed your template, the more structured and consistent the output will be.

---

## 📄 License

This project is open source. Feel free to use and adapt it.
