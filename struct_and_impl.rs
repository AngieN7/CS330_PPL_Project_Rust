pub struct Contact{
    name: String,
    number: int,
    email: string
}
impl Contact {
    pub fn get_name(&self) {
        println("Name;{}", self.name);
        }
}

fn main {
let my_contact = Contact {
        name: Rusty,
        number: 1112223333,
        email: String::from("rusty@gmail.com")
   };
}
