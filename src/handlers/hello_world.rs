
use rocket_contrib::json::Json;

#[derive(Serialize, Deserialize, Debug)]
pub struct User{
  name: String,
  last_name: String,
  age: u8,
}

#[post("/hello", format = "application/json", data = "<user>")]
pub fn hello_world(user: Json<User>) -> Json<User> {
  user
}