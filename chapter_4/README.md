## Capítulo 4 - Understanding Ownership

### O que significa o termo Ownership em Rust?

Ownership é um conjunto de regras que governam como Rust faz o gerenciamento da memória. Em algumas linguagens de programação temos os conhecidos 'coletores de lixo' ou garbage collectors, 
que vasculham a memória utilizada pelo processo procurando por espaços de memória não mais utilizados e os liberam para novos usos, enquanto que outras linguagens obrigam que o próprio
desenvolvedor faça a alocação e a liberação de memória explícitamente.

Rust possui uma terceira abordagem: a memória é gerenciada por um sistema de Ownership, que possui um conjunto de regras que são seguidas pelo Compilador. Se qualquer uma dessas regras é violada, o 
programan não irá compilar. 

```
Both the stack and the heap are parts of memory available to your code to use at runtime, but they are structured in different ways. The stack stores values in the order it gets them and removes the values in the opposite order. This is referred to as last in, first out. Think of a stack of plates: when you add more plates, you put them on top of the pile, and when you need a plate, you take one off the top. Adding or removing plates from the middle or bottom wouldn’t work as well! Adding data is called pushing onto the stack, and removing data is called popping off the stack. All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead.

The heap is less organized: when you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. This process is called allocating on the heap and is sometimes abbreviated as just allocating (pushing values onto the stack is not considered allocating). Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer. Think of being seated at a restaurant. When you enter, you state the number of people in your group, and the staff finds an empty table that fits everyone and leads you there. If someone in your group comes late, they can ask where you’ve been seated to find you.

Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack. Comparatively, allocating space on the heap requires more work, because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.

Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there. Contemporary processors are faster if they jump around less in memory. Continuing the analogy, consider a server at a restaurant taking orders from many tables. It’s most efficient to get all the orders at one table before moving on to the next table. Taking an order from table A, then an order from table B, then one from A again, and then one from B again would be a much slower process. By the same token, a processor can do its job better if it works on data that’s close to other data (as it is on the stack) rather than farther away (as it can be on the heap). Allocating a large amount of space on the heap can also take time.
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
Take a look at Figure 4-1 to see what is happening to String under the covers. A String is made up of three parts, shown on the left: a pointer to the memory that holds the contents of the string, a length, and a capacity. This group of data is stored on the stack. On the right is the memory on the heap that holds the contents.
```

- Uma String é feita de 3 partes:
 - Um ponteiro para o endereço de memória onde essa string começa a ser armazenada
 - Um dado que representa seu Tamanho (tamanho corrente da string em bytes)
 - E um dado que representa sua Capacidade (tamanho máximo da string em bytes que ela recebeu do memory allocator)
 - Todas essas informações ficam armazenada na Stack

- O conteúdo da String fica armazenadado na memória Heap.

- Exemplo de código que não compila. 
```
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
{
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
}
```

O exemplo acima deixa claro que em Rust, quando passamos uma variável dessa forma para a função, a variável é transferida para um novo dono, logo ela só existira dentro da função para a qual foi passada, e não mais no escopo atual. Isso é legal porque evita que uma função altere algum valor (estado) da variável parâmetro de forma implícita. Se no escopo principal, ainda precisarmos do valor da variável fornecida no parâmetro, a função deve explícitamente retornar o valor da variável e atribuirmos a uma variável novamente.

- Se quisermos passar uma variável para uma função sem fazermos a transferência de Ownership, podemos fazer isso usando uma passagem de variável por referência


### References and Borrowing