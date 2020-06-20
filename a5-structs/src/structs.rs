struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn structs() {
  let _user = build_user(String::from("Email"), String::from("Username"));

  let _user2 = User {
    active: false,
    .._user
  };

  let _black = Color(0, 0, 0);
  let _origin = Point(0, 0, 0);
}
