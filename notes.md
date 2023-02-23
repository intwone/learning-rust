# Escalares (scalar types)

- Representam um único valor contido dentro de uma escala conhecida.
- Permitem a comparação direta entre valores.

Mais informações sobre scalar types: <https://doc.rust-lang.org/book/ch03-02-data-types.html>

## Tipos

- Inteiros (integer) ex: 5
- Fluantes (floating point) ex: 42.1
- Booleano (bool) ex: true, false
- Caracter (char) ex: 'a', '%'

# Compostos (Compound Types)

- Servem para agregar multiplos valores

## Tipos

- Tupla (tuple) ex: (5, true, 12.1, 'q')
- Matriz (array) ex: [1, 2, 3, 4, 5]

# Strings

- Aspas simples -> caracteres
- Aspas duplas -> strings

# Ownership & Borrowing

Com base nos tipos escalares (Copy) do Rust, nosso exemplo abaixo irá utilizar a memória `Stack`

```rust
fn main() {
  let a: i32 = 1;
  let b = a;
}
```

Nesse caso acima, o Rust não fará como em algumas linguagens, que
fará a variável "a" e "b" apontarem para o mesmo local de memória que está o valor 1. Diferente disso, o Rust criará um outro espaço na memória para a variável "b", ou seja, "a" e "b" tem referências distintas na memória e isso faz com que mutação de uma não afete o resultado da outra. Caso o programador não queria ter esse comportamento, então devemos criar explicitamente uma referência da variável "b" para a variável "a", conforme o exemplo abaixo:

```rust
fn main() {
  let a: i32 = 1;
  let b = &a;
}
```

Nesse caso, para ultizar a variável "b", teremos que utilizar um ponteiro, pois "b" não tem o valor 1 e sim a referência da variável "a"

```rust
fn main() {
  let a: i32 = 1;
  let b = &a;

  minha_funcao() {
    a + *b
  }
}
```

Com base nos tipos não escalares, nosso exemplo abaixo irá utilizar a memória `Heap`

```rust
fn main() {
  let a = String::from("Cassio"); // Alocação na memória heap, ou seja, "a" é dona (owner) dessa string "Cassio"
  let b = a;

  println!("{a}")
  println!("{b}")
}
```

Nesse caso acima, a String não é um valor "Copy" por default, ou seja, não será criado um novo espaço na memória para a variável "b".
Essa decisão foi tomada pois se as duas variáveis (a e b) apontasse para o mesmo espaço de memória, não teria como o Rust saber qual variável seria responsável pela "limpeza" desse espaço de memória.
Para resolver isso, o Rust faz um processo de transferência de owner.
Ex.:

- let a é responsável pelo espaço de memória da String "Cassio"
- quando a execução do programa chega na instrução `let b = a`, a variável "b" passa a ser a owner desse espaço e por consequencia ficará
responsável por limpar essa informação da memória.

Agora, temos um problema.
Como o owner da String "Cassio" foi passado para a variável "b", a variável "a" passa a ser inválida, fazendo com que ocorra um erro durante o `println!("{a}")`, pois "a" não existe mais.

Para resolver esse problema, criamos uma referência da variável "b" para a variável "a", ou seja, estamos "emprestando" o valor de "a" para "b", assim:

```rust
fn main() {
  let a = String::from("Cassio"); // Alocação na memória heap, ou seja, "a" é dona (owner) dessa string "Cassio"
  let b = &a;

  println!("{a}")
  println!("{b}")
}
```

Regras:

- Cada valor tem um dono (owner)
- Só pode ter um único dono
- Quando o dono sai de escopo o valor é limpo
- A posse (ownership) pode ser movida para outro dono

Utilizando os mesmos exemplos, porém com funções:

```rust
fn say_hello(name: &str) {
  println!("Hello {name}")
}

fn say_goodbye(name: &str) {
  println!("Goodbye {name}")
}

fn main() {
  let name = "Cassio";
  say_hello(name);
  say_goodbye(name);
}
```

Nesse exemplo acima, estamos utilizando a memória Stack (parâmetros das funções são do tipo escalar (Copy Values)) e não temos problema com isso, pois seguimos o mesmo raciocínio do exemplo utilizando variáveis.
Nesse caso, tanto para a função `say_hello` quanto para a função `say_goodbye` a variável name irá ter dois espaços diferentes de memória.

Agora, utilizando a memória Heap (parâmetros das funções são do tipo não escalar (No Copy Values))

```rust
fn say_hello(name: String) {
  println!("Hello {name}")
}

fn say_goodbye(name: String) {
  println!("Goodbye {name}")
}

fn main() {
  let name = "Cassio".to_string(); // to_string() transforma o tipo para String
  say_hello(name);
  say_goodbye(name);
}
```

Nesse caso acima, quando chamarmos say_hello(name), a função say_hello terá a posse (ownership) da variável name. Ao chamar a função say_goodbye, iremos receber um erro do compilador, pois a variável "name" foi movida para o contexto da função say_hello.

Para resolver esse problema, temos:

```rust
fn say_hello(name: String) {
  println!("Hello {name}")
}

fn say_goodbye(name: String) {
  println!("Goodbye {name}")
}

fn main() {
  let name = "Cassio".to_string(); // to_string() transforma o tipo para String
  say_hello(name.clone());
  say_goodbye(name);
}
```

No caso acima, antes de chamar a função say_hello, faremos um clone da variável name e assim teremos duas referências de memória distantas entre o parâmetro da função say_hello e say_goodbye.

Podemos também, utilizar referencias de valores (emprestimo) ao utilizar parâmetros de funções.

Ex.:

```rust
fn say_hello(name: &String) {
  println!("Hello {name}")
}

fn say_goodbye(name: &String) {
  println!("Goodbye {name}")
}

fn main() {
  let name = "Cassio".to_string(); // to_string() transforma o tipo para String
  say_hello(&name);
  say_goodbye(&name);
}
```

Resumo:

- Usando o `Clone` (.clone()) , temos uma alocação "desnecessária" de memória, pois teremos 2 alocações da variável "name" em espaços diferentes de memória.
- Usando `Referência` (&), não temos uma alocação "desnecessária" de memória, estamos "emprestando" o valor da variável name tanto para say_hello quanto para say_goodbye

Regras de Borrowing

- Podemos ter uma única referência caso ela seja mutável
- Podemos ter várias quando são todas imutáveis

Utilizando funções que mudam o valor de uma variável

```rust
fn to_uppercase(text: &mut String) {
  *text = text.to_uppercase()
}

fn add_prefix(text: &mut String) {
  *text = format("FOO_{text}")
}

fn main() {
  let name = "Cassio".to_string(); // to_string() transforma o tipo para String
  to_uppercase(&mut name);
  add_prefix(&mut name);
}
```

Nesse caso acima, estamos fazendo com que o valor recebido por parâmetro seja mutável, mas não precisamos criar varias cópias em memória do "text", pois ela será mutável. Tendo o valor mutável, poderemos utilizar a sintaxe `*text = text.to_uppercase()` para mudar o valor do text. 

Perceba que temos o '*' antes do 'text', pois 'text' sendo um valor mutável, precisaremos referenciar como um ponteiro para que possamos fazer essa mutação.

Existe alguns casos que não iremos precisar referenciar a variável com o '*', pois o próprio Rust fará algumas coisas por debaixo dos panos, como o exemplo abaixo:

```rust
fn to_uppercase(text: &mut String) {
  *text = text.to_uppercase()
}

fn add_prefix(text: &mut String) {
  text.push_str("_FOO")
}

fn main() {
  let name = "Cassio".to_string(); // to_string() transforma o tipo para String
  to_uppercase(&mut name);
  add_prefix(&mut name);
}
```

Nesse caso acima, o Rust usa a referência implicita que acontece quando usamos o ponto (.) para chamar um método de uma variável

# Ver depois

- Diferença de static, stack e heap em rust
- Min sized rust github
- RAII
