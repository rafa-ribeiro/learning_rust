## Capítulo 3 - Common Programming Concepts

### Variáveis e mutabilidade

__1.__ Variáveis são por padrão imutáveis, para torná-la mutáveis, devemos usar a palavra chave mut na declaração da variável. Usar o mut permite alterar o valor da variáve, mas NÃO permite alterar o tipo do dado nela inserida, exemplo substituir o conteúdo de uma variável do tipo string para um valor do tipo inteiro (u32)

__2.__ Constantes são sempre imutáveis, ou seja, não podemos usar mut com constantes.

__3.__ Declaração de constantes:

```rust
fn main() {
	const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}
```

__4.__ Constantes só podem ser inicializadas a partir de uma expressão constante e não do resultado de um código a ser executado, por exemplo.


### Tipos de dados

__1.__ Tipo de dados escalar -> representa um único valor, exemplo: integer, float-point numbers, booleans e characters.

__2.__ Tipo de dados compostos -> podem agrupar múltiplos valores em 1 tipo. Rust possui 2 tipos de dados compostos: tuples e arrays

__3.__ Os tipos de dados dentro de uma tupla, não precisam obrigatoriamente ser o mesmo tipo, exemplo:

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

__4.__ Tupla permite o destructure de seus valores graças ao pattern matching


__5.__ Também podemos acessar o elemento de uma tupla diretamente do seu índice, porém a síntaxe é diferente da que você pode estar acostumado:

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
```


__6.__ Diferentemente da tupla, o array só pode armazenar dados de mesmo tipo


__7.__ Arrays possuem um tamanho fixo


__8.__ Declarando arrays:

```rust
fn main() {
   let a = [1, 2, 3, 4, 5];
}
```

__9.__ Declarando arrays com tipo e tamanho:

```rust
	fn main() {
		let a: [i32; 5] = [1, 2, 3, 4, 5];
	}
```

__10.__ Arrays x Vector -> Arrays possuem tamanho fixo enquanto que Vectors possuem tamanho variável, podem ser acrescido ou encolhidos conforme necessidade. Arrays devem ser usados quando sabemos a quantidade de elementos que estamos trabalhando, exemplo, meses do ano, dias da semana, etc

__11.__ Inicializando um array com o mesmo valor para todos os elementos:

```rust
fn main() {
	let a = [3; 5];
}
```

### Funções

__1.__ Declarando uma função em Rust:

```rust
	fn my_funct() {

	}
```

__2.__ A ordem de declaração das funções não importa para o compilador Rust

__3.__ Statements são instruções que executam alguma ação e não retornam um valor

```
	Statements are instructions that perform some action and do not return a value
```

__4.__ Expressions são avaliadas e resultam em um valor

```
Expressions evaluate to a resulting value.
```

__5.__ Dito o que são Statements e Expressions, em Rust NÃO podemos escrever algo como:

```
x = y = 6
```

__6.__ Expressions não incluem ";" em sua última linha, se incluso, o bloco vira um Statement (sem retorno)

``` 
Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value. Keep this in mind as you explore function return values and expressions next.
```

__7.__ Rust não requer explícitamente que a palavra return seja utilizada. O retorno será a última expressão avaliada:

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
```

__8.__ MAS, a palavra chave return pode ser utilizada para a função retornar antes se necessário, ao invés de executar todo o bloco


### Fluxos de controle

__1.__ Em Rust, a expressão IF sempre espera que um valor booleano seja usado para a condição

Ou seja, um código como esse, comum em Python, não serve para Rust

```
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}
```

É preciso substituir por algo que retorne um booleano, como:

```rust
fn main() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}
```

__2.__ **loop** é a palavra chave para usarmos loops infinitos

__3.__ Podemos usar loops aninhados e diferenciar cada loop usando rótulos, como no exemplo abaixo:

```rust
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
```

__4.__ loops podem retornar valores usando a palavra **break**:

```rust
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
```

__5.__ Exemplo de loop usando **For**:

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }
}
```

__6.__ For reverso usando **Range**:

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```
