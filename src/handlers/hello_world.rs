
use rocket_contrib::json::Json;

#[derive(Serialize, Deserialize, Debug)]
pub struct User{
  name: String,
  last_name: String,
  age: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserNoLastName{
  name: String,
  age: u8,
}

#[post("/hello", format = "application/json", data = "<user>", rank = 2)]
pub fn hello_world(user: Json<User>) -> Json<User> {
  user
}

#[post("/hello", format = "application/json", data = "<user>")]
pub fn hello_world_no_last_name(user: Json<UserNoLastName>) -> Json<UserNoLastName> {
  user
}