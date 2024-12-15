pub struct Session {
    user: String,
}

impl Session {
    pub fn new(user: &str) -> Self {
        Session {
            user: user.to_owned(),
        }
    }
}

pub struct SessionAuthToken {
    session: Session,
}

impl SessionAuthToken {
    pub fn new(session: Session) -> Self {
        SessionAuthToken {
            session,
        }
    }
}

impl AuthToken for SessionAuthToken {
    fn is_authenticated(&self) -> bool {
        self.session.user == "admin"
    }
}

pub trait AuthToken {
    fn is_authenticated(&self) -> bool; 
}