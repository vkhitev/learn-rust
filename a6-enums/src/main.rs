enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

impl Message {
  fn call(&self) {}
}

#[derive(Debug)]
enum UsState {
  Alabama,
  Alaska,
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
      println!("State quarter from {:?}!", state);
      25
    }
  }
}

fn if_let() {
  let some_u8_value = Some(3);

  if let Some(3) = some_u8_value {
    println!("three");
  }

  let coin = Coin::Nickel;
  let mut count = 0;
  if let Coin::Quarter(ref state) = coin {
    println!("State quarter from {:?}!", state);
  } else {
    count += 1;
  }

  match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
  }
}

fn main() {
  let m = Message::Write(String::from("hello"));
  m.call();

  let x = Some(2);

  let y = value_in_cents(Coin::Nickel);

  if_let();
}
