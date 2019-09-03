use csv;
use std::error::Error as std_err;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Error as io_err;
use std::path::Path;
use std::str::FromStr;

#[derive(PartialEq, Clone)]
pub struct AssessmentKind(pub String);

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

#[derive(Eq, PartialEq, Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResponseKind {
    Grade,
    Text,
}

impl ResponseKind {
    pub fn process_data(&self, responses: &Vec<String>) -> Option<String> {
        match self {
            ResponseKind::Grade => self.process_grades(responses),
            ResponseKind::Text => self.process_reviews(responses),
        }
    }

    fn process_grades(&self, grades: &Vec<String>) -> Option<String> {
        if grades.len() == 0 {
            return None;
        }
        let calc = grades
            .iter()
            .map(|item| {
                item.parse::<f32>()
                    .expect(format!("failed to parse: {}", item).as_ref())
                // todo: remove expect
            })
            .sum::<f32>()
            / grades.len() as f32;

        Some(calc.to_string())
    }

    fn process_reviews(&self, reviews: &Vec<String>) -> Option<String> {
        if reviews.len() == 0 {
            return None;
        }
        Some(reviews.join("\n"))
    }
}

impl Display for ResponseKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ResponseKind::Grade => write!(f, "grade"),
            ResponseKind::Text => write!(f, "text"),
        }
    }
}

pub fn read<P: AsRef<Path>>(
    filename: P,
    replace_with: Vec<(&str, &str)>,
) -> Result<Vec<QuestionConfig>, Box<dyn std_err>> {
    let file = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut out: Vec<QuestionConfig> = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let mut question_config = record.deserialize::<QuestionConfig>(None)?;

        question_config.fill_template(&replace_with);
        out.push(question_config);
    }

    Ok(out)
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct QuestionConfig {
    pub assessment_kind: String,
    pub response_kind: ResponseKind,
    pub category: String,

    #[serde(rename = "template")]
    pub template_raw: String,

    #[serde(skip_deserializing)]
    pub template_final: String,
    pub weight: f32,
}

impl QuestionConfig {
    fn fill_template(&mut self, replacers: &Vec<(&str, &str)>) {
        self.template_final = self.template_raw.clone();
        for (from, to) in replacers {
            self.template_final = self.template_final.replace(from, to);
        }
    }

    pub fn eval_answer(&self, input: &str) -> Result<String, Box<dyn std_err>> {
        match self.response_kind {
            ResponseKind::Grade => Ok((input.parse::<f32>()? * self.weight).to_string()),
            ResponseKind::Text => Ok(input.to_string()),
        }
    }

    pub fn match_template(&self, input: &String) -> bool {
        input.contains(&self.template_final) || input.contains(&self.template_raw)
    }
}
