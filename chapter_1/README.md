## Capítulo 1 - Installing Rust

Comando usado para carregar o rust na sessão ativa no terminal:

| source ~/.cargo/env


1. Compilando um arquivo rust:

| rustc <file>.rs

Ao compilar, será gerado um novo arquivo de mesmo nome contendo o executável

2. Criando um projeto usando cargo

| cargo new <project_name>


3. Buildando um projeto com cargo

| cargo build

4. Builda e executa o projeto com somente um comando

| cargo run

5. Checando se o arquivo compila, mas sem gerar o executável

| cargo check

6. Buildando o projeto para release

| cargo build --release
