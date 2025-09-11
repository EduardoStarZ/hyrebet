use common::crypto::hash;

fn main() {
    common::test(); 
    let test : String = String::from("bolo");

    let hashed : String = hash(test);
    println!("{hashed}");
}
