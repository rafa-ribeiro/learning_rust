## Capítulo 3 - Common Programming Concepts

### Variáveis e mutabilidade

__1.__ Variáveis são por padrão imutáveis, para torná-la mutáveis, devemos usar a palavra chave mut na declaração da variável. Usar o mut permite alterar o valor da variáve, mas NÃO permite alterar o tipo do dado nela inserida, exemplo substituir o conteúdo de uma variável do tipo string para um valor do tipo inteiro (u32)

__2.__ Constantes são sempre imutáveis, ou seja, não podemos usar mut com constantes.

__3.__ Declaração de constantes:

```rust
#![allow(unused)]
fn main() {
	const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}
```

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
