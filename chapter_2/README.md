## Capítulo 2 - Programming a Guessing Game

Criando o projeto com o Cargo

```
cargo new guessing_game
```

**Prelude Rust Module** 

Link: https://doc.rust-lang.org/stable/std/prelude/index.html


Por padrão, todas as variáveis em Rust são imutáveis, para torná-las mutáveis, precisamos declarar a variável assim:

```
let mut number = String::new();
```

No exemplo acima, _new_ é uma função associada (associated function) do tipo String.

The :: syntax in the ::new line indicates that new is an associated function of the String type. An associated function is a function that’s implemented on a type, in this case String. This new function creates a new, empty string. You’ll find a new function on many types, because it’s a common name for a function that makes a new value of some kind.


```
io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
```

No trecho acima, o _&_ indica que o argumento guess é uma *referência* e, assim como variáveis, referências são imutáveis. Para torná-las mutáveis, também precisamos aqui do *mut*


Gera a documentação das dependências do seu projeto rust usando o comando:

```
cargo doc --open
``