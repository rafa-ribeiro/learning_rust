## Capítulo 5 - Using Structs to Structure Related Data

Uma struct ou estrutura é um tipo de dado customizado que nos permite empacotar e nomear juntos múltiplos dados relacionados à um contexto específico. Se você tem alguma familiaridade com Orientação à Objetos, uma struct é como uma classe que contém somente atributos.

### Definindo e instanciando Structs

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

Nota importante da Doc do Rust:

```
Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable
```

Logo, se eu pretendo alterar algum dado de User, o objeto todo deve ser mutável.


```rust
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
```