/**
 * Por padrão o rust não imprimi structs pois elas podem ter vários formatos,
 * para isso, podemos implementar nossa própria forma de mostrar os valores de uma determinada struct,
 * podemos usar o atributo #[derive(Debug)] acima da struct para que possamos mostrá-la na tela ou usar
 * a macro dbg!(&variable)
 */

impl std::fmt::Display for User {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "{},{},{},{}",
      self.name, self.surname, self.is_active, self.access_count
    )
  }
}

struct User {
  name: String,
  surname: String,
  is_active: bool,
  access_count: i32,
}

// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like structs
struct Sometring;

fn main() {
  let user = User {
    name: "Cassio".to_string(),
    surname: "Oliveira".to_string(),
    is_active: false,
    access_count: 1,
  };

  // usando valores da variável user para criar a variável user2
  let user2 = User {
    name: user.name,
    surname: user.surname,
    is_active: true,
    access_count: user.access_count,
  };

  // short sintaxe: usando valores da variável user2 para criar a variável user3
  let user3 = User {
    is_active: false,
    ..user2
  };

  println!("{}", user3);

  // usando tuple structs
  let black = Color(0, 0, 0);
  let origin = Point(1, 2, 4);

  // usando unit-like struct
  let thing = Sometring;
}

fn build_user(name: String, surname: String) -> User {
  User {
    name: name,
    surname: surname,
    is_active: true,
    access_count: 0,
  }
}

fn build_user_with_shorthand(name: String, surname: String) -> User {
  User {
    name,
    surname,
    is_active: true,
    access_count: 0,
  }
}
