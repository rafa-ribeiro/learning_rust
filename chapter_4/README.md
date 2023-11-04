## Capítulo 4 - Understanding Ownership

### O que significa o termo Ownership em Rust?

Ownership é um conjunto de regras que governam como Rust faz o gerenciamento da memória. Em algumas linguagens de programação temos os conhecidos 'coletores de lixo' ou garbage collectors, 
que vasculham a memória utilizada pelo processo procurando por espaços de memória não mais utilizados e os liberam para novos usos, enquanto que outras linguagens obrigam que o próprio
desenvolvedor faça a alocação e a liberação de memória explícitamente.

Rust possui uma terceira abordagem: a memória é gerenciada por um sistema de Ownership, que possui um conjunto de regras que são seguidas pelo Compilador. Se qualquer uma dessas regras é violada, o 
programan não irá compilar. 

```
Both the stack and the heap are parts of memory available to your code to use at runtime, but they are 
structured indifferent ways. The stack stores values in the order it gets them and removes the values in 
the opposite order. This is referred to as last in, first out. Think of a stack of plates: when you add 
more plates, you put them on top of the pile, and when you need a plate, you take one off the top. 
Adding or removing plates from the middle or bottom wouldn’t work as well! Adding data is called pushing
onto the stack, and removing data is called popping off the stack. All data stored on the stack must 
have a known, fixed size. Data with an unknown size at compile time or a size that might change must 
be stored on the heap instead.

The heap is less organized: when you put data on the heap, you request a certain amount of space. 
The memory allocatorfinds an empty spot in the heap that is big enough, marks it as being in use,
and returns a pointer, which is the address of that location. This process is called allocating 
on the heap and is sometimes abbreviated as just allocating (pushing values onto the stack is not
considered allocating). Because the pointer to the heap is a known, fixed size, you can store 
the pointer on the stack, but when you want the actual data, you must follow the pointer. 
Think of being seated at a restaurant. When you enter, you state the number of people in your 
group, and the staff finds an empty table that fits everyone and leads you there. If someone in 
your group comes late, they can ask where you’ve been seated to find you.

Pushing to the stack is faster than allocating on the heap because the allocator never has to search
for a place to store new data; that location is always at the top of the stack. Comparatively, allocating
space on the heap requires more work, because the allocator must first find a big enough space to hold the
data and then perform bookkeeping to prepare for the next allocation.

Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer
to get there. Contemporary processors are faster if they jump around less in memory. Continuing the analogy,
consider a server at a restaurant taking orders from many tables. It’s most efficient to get all the orders 
at one table before moving on to the next table. Taking an order from table A, then an order from table B, 
then one from A again, and then one from B again would be a much slower process. By the same token, 
a processor can do its job better if it works on data that’s close to other data (as it is on the stack) 
rather than farther away (as it can be on the heap). Allocating a large amount of space on the heap can 
also take time.
```

### Ownership Rules

- Cada valor em Rust possui uma variábel que é o seu Dono
- Pode haver apenas um Dono por vez
- Quando o Dono sair de escopo, o valor será descartado da memória


### Memory and Allocation

- Uma variável em Rust tem seu valor desalocado quando seu escopo de execução é finalizado

```rust
{
    let s = String::from("hello"); // s é válido a partir desse ponto
    // faz algo com s
} // esse escopo foi finalizado, logo s não é mais uma variável válida	
```

- Essa desalocação de memória é feita pelo próprio Rust chamando a função **drop**

```
Take a look at Figure 4-1 to see what is happening to String under the covers. A String is made up of three parts, 
shownon the left: a pointer to the memory that holds the contents of the string, a length, and a capacity. This 
group of datais stored on the stack. On the right is the memory on the heap that holds the contents.
```

- Uma String é feita de 3 partes:
 - Um ponteiro para o endereço de memória onde essa string começa a ser armazenada
 - Um dado que representa seu Tamanho (tamanho corrente da string em bytes)
 - E um dado que representa sua Capacidade (tamanho máximo da string em bytes que ela recebeu do memory allocator)
 - Todas essas informações ficam armazenada na Stack

- O conteúdo da String fica armazenadado na memória Heap.

- Exemplo de código que não compila. 
```rust
{
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
}
```

Após s1 ser movido para s2, Rust entende que s1 não é mais válido. Fazendo isso Rust evita um problema conhecido como double free error, que ocorre quando há a tentativa de liberar o mesmo endereço de memória mais de uma vez.

- Então para copiar um dado de um local para o outro, podemos usar o método clone que executa um deeply copy:

```rust
{
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
```

### Ownership and Functions

- Exemplo de Ownership com funções na prática:

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

- Exemplo com funções que retornam valor (_transferem ownership_):

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

O exemplo acima deixa claro que em Rust, quando passamos uma variável dessa forma para a função, a variável é transferida para um novo dono, logo ela só existira dentro da função para a qual foi passada, e não mais no escopo atual. Isso é legal porque evita que uma função altere algum valor (estado) da variável parâmetro de forma implícita. Se no escopo principal, ainda precisarmos do valor da variável fornecida no parâmetro, a função deve explícitamente retornar o valor da variável e atribuirmos a uma variável novamente.

- Se quisermos passar uma variável para uma função sem fazermos a transferência de Ownership, podemos fazer isso usando uma passagem de variável por referência


### References and Borrowing

- Exemplo de uso de passagem por referência ao invés de transferência de ownership:

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

```

Reference é algo como um ponteiro em que é um endereço que podemos seguir para acessar o dado armazenado nele, que por sua vez esse dado pertence à uma variável.

No entanto, reference e ponteiro são coisas diferentes. Diferente de um ponteiro, uma reference é garantida de apontar para um endereço de um valor válido de um tipo específico. 
Como no exemplo acima que podemos ver que a function _calculate_length_ possui um parâmetro que é uma referencia para um endereço de memória do tipo String.

> O oposto de _referencing_ usando _&_ é o de dereferencing, que é simbolizado pelo operador de __dereference__ com __*__

Código comentado:

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.
```

> __Borrowing__ é a ação de criar uma _reference_


- Borrowing é o termo em inglês para emprestar, para fazer uma alusão de que quando passamos um parâmetro por referência, a variável da função não é dona do valor,
mas sim ela o pegou emprestado, logo se a função pega ele emprestado, ao terminar de usá-lo, a função deve devolvê-lo para o seu dono.

O exemplo abaixo é de um código que __não funciona__ pois ao se passar emprestado um valor, não é possível modificá-lo, pois assim como as variáveis, as references são por padrão imutáveis.

```rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

Erro:

```
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
 --> src/main.rs:8:5
  |
7 | fn change(some_string: &String) {
  |                        ------- help: consider changing this to be a mutable reference: `&mut String`
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable

For more information about this error, try `rustc --explain E0596`.
error: could not compile `ownership` due to previous error
```

#### Mutable References

Para fazer o código acima funcionar, devemos explicitar que a nossa referência é mutável.

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

> Referências mutáveis possuem uma grande restrição: só é possível existir 1 única referência mutável para um mesmo dado por vez.

O código abaixo irá falhar pois tenta criar duas mutable references para a mes variável __s__:

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}
```

Esse comportamento evita que exista o que em Rust é conhecido por __data race__. Um data race é similar à um __race condition__ e acontece
quando esses 3 comportamentos ocorrem:

1. Dois ou mais ponteiros acessam o mesmo dado ao mesmo tempo
2. Pelo menos 1 dos ponteiros está sendo usado para escrever no endereço do dado
3. Não há um mecanismo utilizado para sincronizar o acesso ao dado

```
The restriction preventing multiple mutable references to the same data at the same time allows for mutation but in a very controlled fashion. It’s something that new Rustaceans struggle with, because most languages let you mutate whenever you’d like. The benefit of having this restriction is that Rust can prevent data races at compile time. A data race is similar to a race condition and happens when these three behaviors occur:

Two or more pointers access the same data at the same time.
At least one of the pointers is being used to write to the data.
There’s no mechanism being used to synchronize access to the data.
Data races cause undefined behavior and can be difficult to diagnose and fix when you’re trying to track them down at runtime; Rust prevents this problem by refusing to compile code with data races!
```

No entanto, podemos contornar o efeito acima iniciando um novo escopo usando chaves. Exemplo:

```rust
fn main() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}
```

O mesmo erro acima, ocorre ao combinarmos referências mutáveis e imutáveis do mesmo dado:

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
}
```

Desde que um novo escopo se inicie, podemos criar novas referências do dado, como no exemplo abaixo:

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}
```

No exemplo acima:

- são criadas as variáveis r1 e r2 que são imutable references de _s_ e são utiliadas na macro println
- Após usadas, elas saem do escopo (o compilador as descartam)
- Logo é possível criar r3 que é uma mutable reference


### The Slice Type

Slice ou fatia permite com que se faça referência à porções/sequências de uma coleção, em vez de à coleção inteira. Um Slice é um tipo de referência, portanto não possui ownership.

Vamos tomar esse pequeno programa para entender o uso do Slice:
- Escreva uma função que recebe uma string de palavras separada por espaço e retorna a primeira palavra encontra nessa string. Se a função não tiver nenhum espaço, significa que a string toda deve ser uma única palavra e então retornada:

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
```

1. Como nós precisamos iterar a string elemento por elemento para verificar se é um espaço em branco, nós convertemos nossa string para um array de bytes usando o método as_bytes.
2. Usamos o método iter() que retorna cada elemento na coleção e o enumerate() que obtém esse resultado e o envolve em uma Tupla que contém o índice do elemento no array e uma referência para o próprio elemento, nessa ordem.
3. O for aqui funciona como no Python, podemos fazer o desestruturar a tupla em váriaveis.


#### String Slices

Usando string slices:

```rust
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
```

O Exemplo acima mostra como podemos fazer o slice de uma string usando os índices de início e fim. Esses índices são parecidos com o do Python também, o índice de início aponta para o índice exato do array, enquanto que o índice de fim aponta para o valor fim + 1, ou seja, o intervalo &s[0..5] compreende os índices 0, 1, 2, 3 e 4.

Essas regrinhas também se assemelham a como usamos no Python:

```rust
    let s = String::from("hello world");

    let slice = &s[..2]; // Quando início omitido, entende que o start é o 0
    let slice = &s[3..]; // Quando fim omitido, entende que o fim é o len de s
    let slice = &s[..]; // Quando omite ambos, pega a string toda
```

Sabendo isso agora, vamos reescrever o pequeno programa acima usando a String Slice. Identificamos uma string slice pelo tipo &str:

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

Usando o String Slice, um possível erro de consistência no caso de alterarmos a string original após a execução da função first_word, pode ser pega ainda em tempo de compilação, saca só:

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}
```

E ao executar:

```
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
  --> src/main.rs:18:5
   |
16 |     let word = first_word(&s);
   |                           -- immutable borrow occurs here
17 |
18 |     s.clear(); // error!
   |     ^^^^^^^^^ mutable borrow occurs here
19 |
20 |     println!("the first word is: {}", word);
   |                                       ---- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `ownership` due to previous error
```

Ainda tem uma última melhoria que podemos fazer:

Ao declarar uma string literal, fazemos: 

```rust
let s = "Hello, world!";
```

Assim, s é do tipo &str, ou seja, uma referência que aponta para um ponto específico no binário, isso explica também porque string literals são imutáveis, porque &str é uma referência imutável.

Um programador Rust mais experiente, ao invés de escrever: 

```rust
fn first_word(s: &String) -> &str {}
```

Poderia escrever:

```rust
fn first_word(s: &str) -> &str {}
```

Escrita dessa forma, o parâmetro s aceita tanto valores do tipo &String quanto &str

```rust
fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
```

#### Outros tipos de Slice

Slice também podem ser usados em outras coleções que não somente uma string:

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```

A variável slice é do tipo &[i32].