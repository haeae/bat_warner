use bat3::*;




fn main() {
    let level = percentage();
    if level >= limit(){
        println!("over limit");
        alert();
    }
}
