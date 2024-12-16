use macros::authenticated;
use types::{AuthToken, Session, SessionAuthToken};

pub struct AnyOther {
    name: String,
}


// after transformation
// pub fn do_stuff(session: Session) {
//     let token: Box<dyn AuthToken> = Box::new(SessionAuthToken::new(session));
    
//     if token.is_authenticated() {
//         println!("Yes, you are authenticated !!!");
//     } else {
//         println!("Oh no, you are not authenticated :(");
//     }
// }

// input
#[authenticated]
pub fn before_do_stuff(token: Box<dyn AuthToken>, any: AnyOther) {
    if token.is_authenticated() {
        println!("Yes, you are authenticated !!! Any = {}", any.name);
    } else {
        println!("Oh no, you are not authenticated :(");
    }
}



fn main() {
    let s = Session::new("admin");
    let a = AnyOther { name: "any other stuff".to_string() };

    before_do_stuff(s, a);
    println!("Running");
}


