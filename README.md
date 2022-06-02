## Learning rust


- The Rust Programming Language (https://doc.rust-lang.org/stable/book/ch00-00-introduction.html)

- GitHub: https://github.com/rust-lang/book/tree/main/src
 

- [x] Capítulo 1
- [x] Capítulo 2
- [x] Capítulo 3
- [ ] Capítulo 4
- [ ] Capítulo 5
- [ ] Capítulo 6
- [ ] Capítulo 7
- [ ] Capítulo 8
- [ ] Capítulo 9
- [ ] Capítulo 10
- [ ] Capítulo 11
- [ ] Capítulo 12
- [ ] Capítulo 13
- [ ] Capítulo 14
- [ ] Capítulo 15
- [ ] Capítulo 16
- [ ] Capítulo 17
- [ ] Capítulo 18
- [ ] Capítulo 19
- [ ] Capítulo 20


### Capítulo 1 - Installing Rust

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


### Capítulo 2 - Programming a Guessing Game

Criando o projeto

| cargo new guessing_game


Prelude Rust Module

| https://doc.rust-lang.org/stable/std/prelude/index.html


Por default, todas as variáveis em Rust são imutáveis, para torná-las mutáveis, precisamos declarar a variável assim:

let mut number = String::new();

No exemplo acima, new é uma função associada (associated function) do tipo String.

The :: syntax in the ::new line indicates that new is an associated function of the String type. An associated function is a function that’s implemented on a type, in this case String. This new function creates a new, empty string. You’ll find a new function on many types, because it’s a common name for a function that makes a new value of some kind.


'
	io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
'

No trecho acima, o & indica que o argumento guess é uma *referência*, e assim como variáveis, referências são imutáveis, assim para 
torná-las mutáveis, também precisamos aqui do *mut*


Gere a documentação das dependências do seu projeto rust usando o comando:

'
cargo doc --open
'


### Capítulo 3 - Common Programming Concepts

#### Variáveis e mutabilidade

1. Variáveis são por padrão imutáveis, para torná-la mutáveis, devemos usar a palavra chave mut na declaração da variável. Usar o mut permite alterar o valor da variáve, mas NÃO permite alterar o tipo do dado nela inserida, exemplo substituir o conteúdo de uma variável do tipo string para um valor do tipo inteiro (u32)

2. Constantes são sempre imutáveis, ou seja, não podemos usar mut com constantes.

3. Declaração de constantes:

'
#![allow(unused)]
fn main() {
	const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}
'

4. Constantes só podem ser inicializadas a partir de uma expressão constante e não do resultado de um código a ser executado, por exemplo.


#### Tipos de dados

1. Tipo de dados escalar -> representa um único valor, exemplo: integer, float-point numbers, booleans e characters.

2. Tipo de dados compostos -> podem agrupar múltiplos valores em 1 tipo. Rust possui 2 tipos de dados compostos: tuples e arrays

3. Os tipos de dados dentro de uma tupla, não precisam obrigatoriamente ser o mesmo tipo, exemplo:

'
    fn main() {
        let tup: (i32, f64, u8) = (500, 6.4, 1);
    }
'

4. Tupla permite o destructure de seus valores graças ao pattern matching


5. Também podemos acessar o elemento de uma tupla diretamente do seu índice, porém a síntaxe é diferente da que você pode estar acostumado:

'
    fn main() {
        let x: (i32, f64, u8) = (500, 6.4, 1);
        let five_hundred = x.0;
        let six_point_four = x.1;
        let one = x.2;
    }
'


6. Diferentemente da tupla, o array só pode armazenar dados de mesmo tipo


7. Arrays possuem um tamanho fixo


8. Declarando arrays:

'
    fn main() {
       let a = [1, 2, 3, 4, 5];
    }
'

9. Declarando arrays com tipo e tamanho:

'
	fn main() {
		let a: [i32; 5] = [1, 2, 3, 4, 5];
	}
'

10. Arrays x Vector -> Arrays possuem tamanho fixo enquanto que Vectors possuem tamanho variável, podem ser acrescido ou encolhidos conforme necessidade. Arrays devem ser usados quando sabemos a quantidade de elementos que estamos trabalhando, exemplo, meses do ano, dias da semana, etc

11. Inicializando um array com o mesmo valor para todos os elementos:

'
	fn main() {
		let a = [3; 5];
	}
'

#### Funções

1. Declarando uma função em Rust:

'
	fn my_funct() {

	}
'

2. A ordem de declaração das funções não importa para o compilador Rust

3. Statements são instruções que executam alguma ação e não retornam um valor

| Statements are instructions that perform some action and do not return a value

4. Expressions são avaliadas e resultam em um valor

| Expressions evaluate to a resulting value. Let’s look at some examples.

5. Dito o que são Statements e Expressions, em Rust NÃO podemos escrever algo como:

'
x = y = 6
'

6. Expressions não incluem ";" em sua última linha, se incluso, o bloco vira um Statement (sem retorno)

| Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value. Keep this in mind as you explore function return values and expressions next.

7. Rust não requer explícitamente que a palavra return seja utilizada. O retorno será a última expressão avaliada:

'
	fn five() -> i32 {
	    5
	}

	fn main() {
	    let x = five();

	    println!("The value of x is: {}", x);
	}
'

8. MAS, a palavra chave return pode ser utilizada para a função retornar antes se necessário, ao invés de executar todo o bloco


#### Fluxos de controle

1. Em Rust, a expressão IF sempre espera que um valor booleano seja usado para a condição

Ou seja, um código como esse, comum em Python, não serve para Rust

'
	fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}
'

É preciso substituir por algo que retorne um booleano, como:

'
	fn main() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}
'

2. loop é a palavra chave para usarmos loops infinitos

3. Podemos usar loops aninhados e diferenciar cada loop usando rótulos, como no exemplo abaixo:

'
	fn main() {
	    let mut count = 0;
	    'counting_up: loop {
	        println!("count = {}", count);
	        let mut remaining = 10;
	        loop {
	            println!("remaining = {}", remaining);
	            if remaining == 9 {
	                break;
	            }
	            if count == 2 {
	                break 'counting_up;
	            }
	            remaining -= 1;
	        }
	        count += 1;
	    }
	    println!("End count = {}", count);
	}
'

4. loops podem retornar valores usando a palavra break:

'
	fn main() {
	    let mut counter = 0;

	    let result = loop {
	        counter += 1;

	        if counter == 10 {
	            break counter * 2;
	        }
	    };

	    println!("The result is {}", result);
	}
'

5. Exemplo de loop usando For:

'
	fn main() {
	    let a = [10, 20, 30, 40, 50];

	    for element in a {
	        println!("the value is: {}", element);
	    }
	}
'

6. For reverso usando Range:

'
	fn main() {
	    for number in (1..4).rev() {
	        println!("{}!", number);
	    }
	    println!("LIFTOFF!!!");
	}
'

### Capítulo 4 - Understanding Ownership

#### O que significa o termo Ownership em Rust?

Ownership é um conjunto de regras que governam como Rust faz o gerenciamento da memória. Em algumas linguagens de programação temos os conhecidos 'coletores de lixo' ou garbage collectors, 
que vasculham a memória utilizada pelo processo procurando por espaços de memória não mais utilizados e os liberam para novos usos, enquanto que outras linguagens obrigam que o próprio
desenvolvedor faça a alocação e a liberação de memória explícitamente.

Rust possui uma terceira abordagem: a memória é gerenciada por um sistema de Ownership, que possui um conjunto de regras que são seguidas pelo Compilador. Se qualquer uma dessas regras é violada, o 
programan não irá compilar. 

'
Both the stack and the heap are parts of memory available to your code to use at runtime, but they are structured in different ways. The stack stores values in the order it gets them and removes the values in the opposite order. This is referred to as last in, first out. Think of a stack of plates: when you add more plates, you put them on top of the pile, and when you need a plate, you take one off the top. Adding or removing plates from the middle or bottom wouldn’t work as well! Adding data is called pushing onto the stack, and removing data is called popping off the stack. All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead.

The heap is less organized: when you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. This process is called allocating on the heap and is sometimes abbreviated as just allocating (pushing values onto the stack is not considered allocating). Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer. Think of being seated at a restaurant. When you enter, you state the number of people in your group, and the staff finds an empty table that fits everyone and leads you there. If someone in your group comes late, they can ask where you’ve been seated to find you.

Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack. Comparatively, allocating space on the heap requires more work, because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.

Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there. Contemporary processors are faster if they jump around less in memory. Continuing the analogy, consider a server at a restaurant taking orders from many tables. It’s most efficient to get all the orders at one table before moving on to the next table. Taking an order from table A, then an order from table B, then one from A again, and then one from B again would be a much slower process. By the same token, a processor can do its job better if it works on data that’s close to other data (as it is on the stack) rather than farther away (as it can be on the heap). Allocating a large amount of space on the heap can also take time.

'

#### Ownership Rules

- Cada valor em Rust possui uma variábel que é o seu Dono
- Pode haver apenas um Dono por vez
- Quando o Dono sair de escopo, o valor será descartado da memória




#### Memory and Allocation

- Uma variável em Rust tem seu valor desalocado quando seu escopo de execução é finalizado

'
	{
		let s = String::from("hello"); // s é válido a partir desse ponto
		// faz algo com s
	} // esse escopo foi finalizado, logo s não é mais uma variável válida	
'

- Essa desalocação de memória é feita pelo próprio Rust chamando a função drop

'
Take a look at Figure 4-1 to see what is happening to String under the covers. A String is made up of three parts, shown on the left: a pointer to the memory that holds the contents of the string, a length, and a capacity. This group of data is stored on the stack. On the right is the memory on the heap that holds the contents.
'

- Uma String é feita de 3 partes:
	- Um ponteiro para o endereço de memória onde essa string começa a ser armazenada
	- Um dado que representa seu Tamanho (tamanho corrente da string em bytes)
	- E um dado que representa sua Capacidade (tamanho máximo da string em bytes que ela recebeu do memory allocator)
	- Todas essas informações ficam armazenada na Stack

- O conteúdo da String fica armazenadado na memória Heap.

- Exemplo de código que não compila. 
'
	{
		let s1 = String::from("hello");
	    let s2 = s1;

	    println!("{}, world!", s1);
	}
'

 Após s1 ser movido para s2, Rust entende que s1 não é mais válido. Fazendo isso Rust evita um problema conhecido como double free error, que ocorre quando há a tentativa de liberar o mesmo endereço de memória mais de uma vez.

- Então para copiar um dado de um local para o outro, podemos usar o método clone que executa um deeply copy:
'
	{
		let s1 = String::from("hello");
	    let s2 = s1.clone();

	    println!("s1 = {}, s2 = {}", s1, s2);
	}
'

#### Ownership and Functions

- Exemplo de Ownership com funções na prática:

'
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
'

- Exemplo com funções que retornam valor (transferem ownership):

'
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
'

O exemplo acima deixa claro que em Rust, quando passamos uma variável dessa forma para a função, a variável é transferida para um novo dono, logo ela só existira dentro da função para a qual foi passada, e não mais no escopo atual. Isso é legal porque evita que uma função altere algum valor (estado) da variável parâmetro de forma implícita. Se no escopo principal, ainda precisarmos do valor da variável fornecida no parâmetro, a função deve explícitamente retornar o valor da variável e atribuirmos a uma variável novamente.

- Se quisermos passar uma variável para uma função sem fazermos a transferência de Ownership, podemos fazer isso usando uma passagem de variável por referência


#### References and Borrowing