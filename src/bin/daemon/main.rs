use servus::entity::User;

fn main() {
    println!("This is daemon.");
    
    let user = User {
        id: None,
        name: "malky".to_owned(),
        email: Some("vladimir.malky@gmail.com".to_owned())
    };

    println!("{:?}", user);
}