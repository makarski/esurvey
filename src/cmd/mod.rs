use std::env;
use std::error::Error;
use std::io::{stdin, Error as io_err, ErrorKind as io_err_kind};
use std::path::PathBuf;
use std::str::FromStr;

pub mod evaluate;
pub mod generate;

pub enum Cmd {
    Generate(generate::Generator),
    Evaluate(evaluate::Evaluator),
}

impl Cmd {
    pub fn run(self, args: clap::ArgMatches) -> anyhow::Result<()> {
        match self {
            Cmd::Generate(generate) => generate.run(args),
            Cmd::Evaluate(eval) => eval.run(args),
        }
    }
}

impl FromStr for Cmd {
    type Err = io_err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let auth_client = auth_client().expect("can't init auth client");

        match s.to_lowercase().as_str() {
            "generate" => Ok(Cmd::Generate(generate::Generator::new(auth_client))),
            "eval" => Ok(Cmd::Evaluate(evaluate::Evaluator::new(auth_client))),
            _ => Err(io_err::new(
                io_err_kind::InvalidInput,
                format!("unknown command: {}", s),
            )),
        }
    }
}

fn auth_client() -> Result<gauth::Auth, Box<dyn Error>> {
    let crd_path = env::var("OAUTH_CFG_FILE")?;
    let auth_client = gauth::Auth::new(
        "probation-check",
        &[
            "https://www.googleapis.com/auth/drive",
            "https://www.googleapis.com/auth/drive.readonly",
            "https://www.googleapis.com/auth/drive.file",
            "https://www.googleapis.com/auth/spreadsheets",
            "https://www.googleapis.com/auth/spreadsheets.readonly",
            "https://www.googleapis.com/auth/script.projects",
        ],
        PathBuf::from(crd_path),
    );

    Ok(auth_client)
}

fn handle_auth(consent_uri: String) -> Result<String, Box<dyn Error>> {
    println!("> open the link in browser\n\n{}\n", consent_uri);
    println!("> enter the auth. code\n");

    let mut auth_code = String::new();
    stdin().read_line(&mut auth_code)?;

    Ok(auth_code)
}
