## Capítulo 1 - Installing Rust

Comando usado para carregar o rust na sessão ativa no terminal:

```
source ~/.cargo/env
```

__1.__ Compilando um arquivo Rust:

```
rustc <file>.rs
```

*Ao compilar, será gerado um novo arquivo de mesmo nome contendo o executável*

__2.__ Criando um projeto usando cargo

```
cargo new <project_name>
```

__3.__ Buildando um projeto com cargo

```
cargo build
```

__4.__ Builda e executa o projeto com somente um comando

```
cargo run
```

__5.__ Checando se o arquivo compila, mas sem gerar o executável

```
cargo check
```

__6.__ Buildando o projeto para release

```
cargo build --release
```
