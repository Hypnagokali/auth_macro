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
pub fn before_do_stuff(a_token: Box<dyn AuthToken>, any: AnyOther) -> bool {
    if a_token.is_authenticated() {
        println!("Yes, you are authenticated !!! Any = {}", any.name);
        return true;
    }

    println!("Oh no, you are not authenticated :(");

    false
}

fn main() {
    println!("Running");
}

#[cfg(test)]
mod test {
    use types::Session;

    use crate::{before_do_stuff, AnyOther};

    #[test]
    fn should_mutate_function() {
        let s = Session::new("admin");
        let a = AnyOther {
            name: "any other stuff".to_string(),
        };

        assert!(before_do_stuff(s, a));
    }
}
