use std::error::Error;

pub struct Evaluator {
    _auth_client: gauth::Auth,
}

impl Evaluator {
    pub fn new(auth_client: gauth::Auth) -> Self {
        Evaluator {
            _auth_client: auth_client,
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        println!("running evaluator command");
        Ok(())
    }
}
