use csv;
use std::error::Error as std_err;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Error as io_err, ErrorKind as io_err_kind};
use std::path::Path;
use std::str::FromStr;

#[derive(PartialEq, Clone)]
pub struct AssessmentKind(pub String);

impl AssessmentKind {
    pub fn config<P: AsRef<Path>>(
        &self,
        filename: P,
        resp_kind: ResponseKind,
    ) -> Result<Vec<Vec<String>>, Box<dyn std_err>> {
        let file = File::open(filename)?;

        let mut rdr = csv::Reader::from_reader(file);
        let mut out: Vec<Vec<String>> = Vec::new();
        for result in rdr.records() {
            let record = result?;
            let mut template_cfgs = record.iter().take(2);

            let valid_resp_kind = template_cfgs.nth(1).and_then(|response_kind_str| {
                if let Ok(t) = ResponseKind::from_str(response_kind_str) {
                    if t == resp_kind {
                        return Some(());
                    }
                }
                return None;
            });

            if valid_resp_kind.is_none() {
                continue;
            }

            let mut collected: Vec<String> = Vec::with_capacity(2);
            for entry in record.iter().skip(2) {
                collected.push(entry.to_owned());
            }

            out.push(collected);
        }

        Ok(out)
    }
}

impl FromStr for AssessmentKind {
    type Err = io_err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(AssessmentKind(String::from(s)))
    }
}

impl Display for AssessmentKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Eq, PartialEq, Clone)]
pub enum ResponseKind {
    Grade,
    Text,
}

impl FromStr for ResponseKind {
    type Err = io_err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "grade" => Ok(ResponseKind::Grade),
            "text" => Ok(ResponseKind::Text),
            _ => Err(io_err::new(
                io_err_kind::InvalidInput,
                format!("parse error. unknown response kind: {}", s),
            )),
        }
    }
}
