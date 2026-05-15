struct User {
    name: String, 
    email: String,
    age: u32, 
    active: bool, 
}

impl User {
    fn new(name: String, email: String, age: u32) -> User {
        User {
            name,
            email,
            age,
            active: true, 
        }
    }

    fn greet(&self) {
        println!("こんにちは、{}さん！", self.name);
    }
}

fn main() {
    let user = User::new(
        String::from("小原幸"),
        String::from("21070@yonago.kosen-ac.jp"),
        30
    );
    user.greet();  //こんにちは、小原幸さん！と表示される
}