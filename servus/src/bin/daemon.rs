use servus::entity::User;

fn main() {
    println!("This is daemon.");
    let user = User { id: None, name: "malky", email: Some("vladimir.malky@gmail.com") };
    println!("{:?}", user);
}