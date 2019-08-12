use csv;
use std::error::Error as std_err;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Error as io_err, ErrorKind as io_err_kind};
use std::path::Path;
use std::str::FromStr;

const SELF_ASSESSMENT_STR: &str = "self-assessment";
const TEAM_FEEDBACK_STR: &str = "team-feedback";

#[derive(PartialEq)]
pub enum AssessmentKind {
    TeamFeedback,
    SelfAssessment,
}

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

            // todo: move validations to a fn
            let assmt_cfg = template_cfgs.next().and_then(|assmt_kind_str| {
                let is_matching_kind = match AssessmentKind::from_str(assmt_kind_str) {
                    Ok(t) => t == *self,
                    Err(_) => false,
                };

                if is_matching_kind {
                    return Some(assmt_kind_str);
                }

                return None;
            });

            let grade_cfg = template_cfgs.next().and_then(|response_kind_str| {
                let is_matching_resp_kind = match ResponseKind::from_str(response_kind_str) {
                    Ok(t) => t == resp_kind,
                    Err(_) => false,
                };

                if is_matching_resp_kind {
                    return Some(response_kind_str);
                }

                return None;
            });

            if assmt_cfg.and(grade_cfg).is_none() {
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
        match s.to_lowercase().as_str() {
            TEAM_FEEDBACK_STR => Ok(AssessmentKind::TeamFeedback),
            SELF_ASSESSMENT_STR => Ok(AssessmentKind::SelfAssessment),
            _ => Err(io_err::new(
                io_err_kind::InvalidInput,
                format!("parse error. unknown assessment kind: {}", s),
            )),
        }
    }
}

impl Display for AssessmentKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            AssessmentKind::SelfAssessment => write!(f, "self-assessment"),
            AssessmentKind::TeamFeedback => write!(f, "team-feedback"),
        }
    }
}

#[derive(PartialEq)]
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
