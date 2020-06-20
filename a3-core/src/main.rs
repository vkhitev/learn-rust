fn mutable_var() {
  let mut x = 5;
  println!("The value of x is: {}", x);
  x = 6;
  println!("The value of x is: {}", x);
}

fn constant() {
  const X: u32 = 5;
  println!("The value of X is: {}", X);
}

fn shadowing() {
  let spaces = "   ";
  let spaces = spaces.len();
  println!("The value of x is: {}", spaces);
}

fn numeric_operations() -> (u32, f64, u8, f32, u64) {
  let sum = 5 + 10;
  let difference = 95.5 - 4.3;
  let product = 4 * 30;
  let quotient = 56.7 / 32.2;
  let remainder = 43 % 5;
  return (sum, difference, product, quotient, remainder);
}

fn arrays() -> [[i32; 5]; 2] {
  let a: [i32; 5] = [1, 2, 3, 4, 5];
  let b = [3; 5];
  return [a, b];
}

fn invalid_access() -> u32 {
  let a = [1, 2, 3, 4, 5];
  let index = 10;
  let element = a[index];
  return element;
}

fn func_with_param(param: u32) -> u32 {
  return param;
}

fn implicit_return() -> String {
  "Hello".to_string()
}

fn cond() {
  let number = 3;

  if number < 5 {
    println!("condition was true");
  } else {
    println!("condition was false");
  }

  if number < 5 {
    'a'
  } else {
    'b'
  };
}

fn loop_example() {
  let mut counter = 0;

  let result = loop {
    counter += 1;

    if counter == 10 {
      break counter * 2;
    }
  };

  println!("The result is {}", result);
}

fn while_example() {
  let mut number = 3;

  while number != 0 {
    println!("{}!", number);

    number -= 1;
  }

  println!("Done");
}

fn for_example() {
  let a = [10, 20, 30, 40, 50];

  let mut index = 0;

  while index < 5 {
    println!("the value is: {}", a[index]);
    index += 1;
  }

  for element in a.iter() {
    println!("the value is: {}", element);
  }

  for number in (1..4).rev() {
    println!("{}!", number);
  }
}

fn main() {
  mutable_var();
  constant();
  shadowing();
  numeric_operations();
  arrays();
  invalid_access();
  func_with_param(30);
  implicit_return();
  cond();
  loop_example();
  while_example();
  for_example();
}
