use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::{Error as io_err, ErrorKind as io_err_kind};
use std::str::FromStr;

const SELF_ASSESSMENT_STR: &str = "self-assessment";
const TEAM_FEEDBACK_STR: &str = "team-feedback";

pub enum AssessmentKind {
    TeamFeedback,
    SelfAssessment,
}

impl AssessmentKind {
    pub fn config_grades(&self) -> Vec<(Skill, u32)> {
        self.config().0
    }

    pub fn config_texts(&self) -> Vec<(Skill, u32)> {
        self.config().1
    }

    fn config(&self) -> (Vec<(Skill, u32)>, Vec<(Skill, u32)>) {
        match self {
            AssessmentKind::SelfAssessment => (
                vec![
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
                vec![
                    (Skill::NewSkill, 1),
                    (Skill::LearningOpportunity, 1),
                    (Skill::Strengths, 1),
                    (Skill::Opportunities, 1),
                ],
            ),
            AssessmentKind::TeamFeedback => (
                vec![
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
                vec![
                    (Skill::NewSkill, 1),
                    (Skill::LearningOpportunity, 1),
                    (Skill::Strengths, 1),
                    (Skill::Opportunities, 1),
                    (Skill::FreeText, 1),
                ],
            ),
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
                "AssessemntKind parse error. valid types: `team-feedback`, `self-assessment`",
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

pub enum Skill {
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
    NewSkill,
    LearningOpportunity,
    Strengths,
    Opportunities,
    FreeText,
}

impl Skill {
    pub fn is_graded(&self) -> bool {
        match self {
            Skill::NewSkill
            | Skill::LearningOpportunity
            | Skill::Strengths
            | Skill::Opportunities
            | Skill::FreeText => false,
            _ => true,
        }
    }
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

            // Textual Feedback
            Skill::NewSkill => write!(f, "New Skill"),
            Skill::LearningOpportunity => write!(f, "Skill to Acquire"),
            Skill::Strengths => write!(f, "Strengths"),
            Skill::Opportunities => write!(f, "Improvement Opportunities"),

            Skill::FreeText => write!(f, "Free Form Feedback"),
        }
    }
}

pub struct EmployeeSkill {
    pub name: Skill,
    pub question_count: u32,
    grades: Vec<u32>,
    texts: Vec<String>,
}

pub fn init_employee_skils(cfg: Vec<(Skill, u32)>) -> Vec<EmployeeSkill> {
    let mut skills: Vec<EmployeeSkill> = Vec::with_capacity(cfg.len());

    for (skill, question_count) in cfg {
        skills.push(EmployeeSkill::new(skill, question_count));
    }

    skills
}

impl EmployeeSkill {
    pub fn new(name: Skill, count: u32) -> EmployeeSkill {
        EmployeeSkill {
            name: name,
            question_count: count,
            grades: Vec::with_capacity(count as usize),
            texts: Vec::with_capacity(count as usize),
        }
    }

    pub fn add_response(&mut self, v: &str) {
        // println!("adding response: {}", v.to_owned());

        if self.name.is_graded() {
            self.add_grade(v.parse::<u32>().expect("could not parse the grade"))
        } else {
            self.texts.push(v.to_owned())
        }
    }

    fn add_grade(&mut self, v: u32) {
        self.grades.push(v)
    }

    pub fn avg(&self) -> f32 {
        self.grades.iter().sum::<u32>() as f32 / self.grades.len() as f32
    }

    pub fn txt(&self) -> String {
        self.texts.join("\n")
    }

    pub fn mark_answered(&mut self) {
        self.question_count -= 1
    }

    pub fn not_answered(&self) -> bool {
        self.question_count > 0
    }
}

impl Display for EmployeeSkill {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.name.is_graded() {
            write!(f, "{}: {}", self.name, self.avg())
        } else {
            write!(f, "{}:\n {}", self.name, self.texts.join("\n"))
        }
    }
}
