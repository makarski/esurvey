extern crate csv;

use std::env::args;
use std::error::Error as std_err;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::{stdin, stdout, Error as io_err, ErrorKind as io_err_kind, Write};
use std::path::Path;
use std::str::FromStr;

const SELF_ASSESSMENT_STR: &str = "self-assessment";
const TEAM_FEEDBACK_STR: &str = "team-feedback";

enum AssessmentKind {
    TeamFeedback,
    SelfAssessment,
}

impl AssessmentKind {
    fn config(&self) -> Vec<(Skill, u32)> {
        match self {
            AssessmentKind::SelfAssessment => vec![
                (Skill::Adaptability, 2),
                (Skill::Attitude, 2),
                (Skill::Communication, 3),
                (Skill::CrossFunctionalKnowledge, 2),
                (Skill::Dependability, 3),
                (Skill::Initiative, 2),
                (Skill::Leadership, 3),
                (Skill::Organization, 3),
                (Skill::Responsibility, 2),
                (Skill::SelfImprovement, 2),
                (Skill::Teamwork, 3),
                (Skill::TechExpertise, 2),
            ],
            AssessmentKind::TeamFeedback => vec![
                (Skill::Adaptability, 2),
                (Skill::Attitude, 2),
                (Skill::Communication, 3),
                (Skill::CrossFunctionalKnowledge, 2),
                (Skill::Dependability, 3),
                (Skill::Initiative, 2),
                (Skill::Leadership, 4),
                (Skill::Organization, 3),
                (Skill::Responsibility, 2),
                (Skill::SelfImprovement, 2),
                (Skill::Teamwork, 2),
                (Skill::TechExpertise, 2),
            ],
        }
    }
}

impl FromStr for AssessmentKind {
    type Err = io_err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            TEAM_FEEDBACK_STR => Ok(AssessmentKind::TeamFeedback),
            SELF_ASSESSMENT_STR => Ok(AssessmentKind::SelfAssessment),
            _ => Err(io_err::new(
                io_err_kind::InvalidInput,
                "InAssessemntType parse error. valid types: `team-feedback`, `self-assessment`",
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

enum Skill {
    Adaptability,
    Attitude,
    Communication,
    CrossFunctionalKnowledge,
    Dependability,
    Initiative,
    Leadership,
    Organization,
    Responsibility,
    SelfImprovement,
    Teamwork,
    TechExpertise,
}

impl Display for Skill {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Skill::Adaptability => write!(f, "Adaptability"),
            Skill::Attitude => write!(f, "Attitude"),
            Skill::Communication => write!(f, "Communication"),
            Skill::CrossFunctionalKnowledge => write!(f, "Cross-functional Knowledge"),
            Skill::Dependability => write!(f, "Dependability"),
            Skill::Initiative => write!(f, "Initiative"),
            Skill::Leadership => write!(f, "Leadership"),
            Skill::Organization => write!(f, "Organization"),
            Skill::Responsibility => write!(f, "Responsibility"),
            Skill::SelfImprovement => write!(f, "Self-Improvement"),
            Skill::Teamwork => write!(f, "Teamwork"),
            Skill::TechExpertise => write!(f, "Tech. Expertise"),
        }
    }
}

struct EmployeeSkill {
    name: Skill,
    questions: u32,
    grades: Vec<u32>,
}

impl EmployeeSkill {
    fn new(n: Skill, q: u32) -> EmployeeSkill {
        EmployeeSkill {
            name: n,
            questions: q,
            grades: Vec::with_capacity(q as usize),
        }
    }

    fn add_grade(&mut self, v: u32) {
        self.grades.push(v)
    }

    fn avg(&self) -> f32 {
        self.grades.iter().sum::<u32>() as f32 / self.grades.len() as f32
    }
}

impl Display for EmployeeSkill {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.avg())
    }
}

fn main() {
    let mut writer = stdout();

    let (empl_name, feedback_kind) = parse_flags().expect("could not parse input flags");
    let cfg_questions = feedback_kind.config().into_iter();

    let mut questions: Vec<EmployeeSkill> = Vec::with_capacity(cfg_questions.len());
    for (skill, question_count) in cfg_questions {
        questions.push(EmployeeSkill::new(skill, question_count));
    }

    let mut rdr = csv::Reader::from_reader(stdin());

    for (index, colleague) in rdr.records().enumerate() {
        let feedback = colleague.expect("could not read colleague's feedback");
        let mut feedback_iter = feedback.iter().enumerate();

        writeln!(writer, ">> scanning response: {}\n", index).unwrap();

        for q in &mut questions {
            writeln!(writer, "scanning '{}'...", q.name).unwrap();
            let mut counter: u32 = 0;

            loop {
                let (index, answer) = feedback_iter
                    .next()
                    .expect("could not retrieve the next value");

                if index < 2 {
                    // skip email and timestamp fields
                    continue;
                }

                let grade: u32 = answer.parse().expect("could not parse grade");
                q.add_grade(grade);

                // break to the next category
                counter = counter + 1;
                if counter == q.questions {
                    break;
                }
            }
        }
    }

    let filename = out_filename(&empl_name, feedback_kind);
    save_to_file(&filename, &questions).expect("failed to save output file");

    writeln!(writer, "\ngenerated file: {}", filename).unwrap();
}

fn out_filename(empl_name: &str, fdb_type: AssessmentKind) -> String {
    format!("{}_{}.csv", empl_name, fdb_type.to_string())
}

fn save_to_file<P: AsRef<Path>>(
    filename: P,
    questions: &Vec<EmployeeSkill>,
) -> Result<(), Box<dyn std_err>> {
    let mut wrt = csv::WriterBuilder::new().from_path(&filename)?;

    for i in 0..2 {
        for skill in questions {
            if i == 0 {
                wrt.write_field(skill.name.to_string())?;
                continue;
            }
            wrt.write_field(format!("{}", skill.avg()))?;
        }
        wrt.write_record(None::<&[u8]>)?;
    }

    wrt.flush()?;

    Ok(())
}

fn parse_flags() -> Result<(String, AssessmentKind), Box<dyn std_err>> {
    let mut empl_name = String::new();
    let mut assessment_type = String::new();

    for arg in args().collect::<Vec<String>>() {
        if arg.contains("-name=") {
            empl_name = arg.trim_start_matches("-name=").parse::<String>()?;
        }

        if arg.contains("-type=") {
            assessment_type = arg.trim_start_matches("-type=").parse::<String>()?;
        }
    }

    if empl_name.is_empty() || assessment_type.is_empty() {
        return Err(Box::new(io_err::new(
            io_err_kind::InvalidInput,
            "empty `-name` or `-type` flags provided",
        )));
    }

    let feedback_kind = AssessmentKind::from_str(assessment_type.as_str());
    match feedback_kind {
        Ok(t) => Ok((empl_name, t)),
        Err(err) => Err(Box::new(err)),
    }
}
