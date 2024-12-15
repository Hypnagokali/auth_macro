use macros::show;
use types::{AuthToken, Session, SessionAuthToken};


// after transformation
pub fn do_stuff(session: Session) {
    let token: Box<dyn AuthToken> = Box::new(SessionAuthToken::new(session));
    
    if token.is_authenticated() {
        println!("Yes, you are authenticated !!!");
    } else {
        println!("Oh no, you are not authenticated :(");
    }
}

// input
#[show]
pub fn before_do_stuff(token: Box<dyn AuthToken>) {
    if token.is_authenticated() {
        println!("Yes, you are authenticated !!!");
    } else {
        println!("Oh no, you are not authenticated :(");
    }
}



fn main() {
    println!("Running");
}


