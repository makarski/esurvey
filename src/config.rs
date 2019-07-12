use csv;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Error as io_err, ErrorKind as io_err_kind};
use std::path::Path;
use std::str::FromStr;

const SELF_ASSESSMENT_STR: &str = "self-assessment";
const TEAM_FEEDBACK_STR: &str = "team-feedback";

pub enum AssessmentKind {
    TeamFeedback,
    SelfAssessment,
}

impl AssessmentKind {
    pub fn config_grades(&self) -> Vec<Vec<String>> {
        self.config(format!("{}-grades.csv", self.to_string()))
    }

    pub fn config_texts(&self) -> Vec<Vec<String>> {
        self.config(format!("{}-text.csv", self.to_string()))
    }

    fn config<P: AsRef<Path>>(&self, filename: P) -> Vec<Vec<String>> {
        let file = File::open(filename).expect("failed to open config file");

        let mut rdr = csv::Reader::from_reader(file);
        let mut out: Vec<Vec<String>> = Vec::new();
        for result in rdr.records() {
            let record = result.expect("failed parsing the entry");

            let mut collected: Vec<String> = Vec::with_capacity(2);
            for entry in record.iter() {
                collected.push(entry.to_owned());
            }

            out.push(collected);
        }

        out
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

#[derive(Clone, PartialEq, Ord, PartialOrd, Eq, Hash)]
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

impl FromStr for Skill {
    type Err = io_err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Adaptability" => Ok(Skill::Adaptability),
            "Attitude" => Ok(Skill::Attitude),
            "Communication" => Ok(Skill::Communication),
            "Cross-functional Knowledge" => Ok(Skill::CrossFunctionalKnowledge),
            "Dependability" => Ok(Skill::Dependability),
            "Initiative" => Ok(Skill::Initiative),
            "Leadership" => Ok(Skill::Leadership),
            "Organization" => Ok(Skill::Organization),
            "Responsibility" => Ok(Skill::Responsibility),
            "Self-Improvement" => Ok(Skill::SelfImprovement),
            "Teamwork" => Ok(Skill::Teamwork),
            "Tech. Expertise" => Ok(Skill::TechExpertise),
            "New Skill" => Ok(Skill::NewSkill),
            "Skill to Acquire" => Ok(Skill::LearningOpportunity),
            "Strengths" => Ok(Skill::Strengths),
            "Improvement Oppotunities" => Ok(Skill::Opportunities),
            "Free Form Feedback" => Ok(Skill::FreeText),
            _ => Err(io_err::new(
                io_err_kind::InvalidInput,
                "unknown skill category input.",
            )),
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

pub struct EmployeeSkills {
    pub skills: Vec<EmployeeSkill>,
    // skill_map: HashMap<Skill, EmployeeSkill>,
    max: Option<EmployeeSkill>,
    min: Option<EmployeeSkill>,
}

use std::collections::HashMap;

impl EmployeeSkills {
    pub fn new(raw_cfg: &Vec<Vec<String>>) -> Self {
        let mut skill_map: HashMap<Skill, EmployeeSkill> = HashMap::new();

        for row in raw_cfg.into_iter() {
            let category = row
                .get(0)
                .expect(format!("failed to retrieve config category from: {:?}", row).as_str());
            let template = row
                .get(1)
                .expect(format!("failed to retrieve config template from: {:?}", row).as_str());

            let skill = Skill::from_str(category.as_str()).expect("failed category input");
            if let Some(employee_skill) = skill_map.get_mut(&skill) {
                println!(">> adding config template: {}: {}", &skill, template);

                employee_skill.add_template(template.to_string());
            } else {
                let mut employee_skill = EmployeeSkill::new(skill.clone());

                employee_skill.add_template(template.to_string());
                println!(">> adding config template: {}: {}", &skill, template);

                skill_map.insert(skill, employee_skill);
            }
        }

        let mut skills: Vec<EmployeeSkill> = Vec::new();
        skill_map.drain().for_each(|(_, es)| skills.push(es));
        skills.sort_by(|a, b| a.cmp(b));

        EmployeeSkills {
            skills: skills,
            // skill_map: skill_map,
            max: None,
            min: None,
        }
    }

    fn find_skill(&mut self, question: &String) -> Option<&mut EmployeeSkill> {
        for skill in self.skills.as_mut_slice() {
            for template in &skill.question_templates {
                if question.contains(template) {
                    return Some(skill);
                }
            }
        }

        println!("\n>>> not found skill: {}|\n", question);

        None
    }

    pub fn scan(&mut self, skip: usize, from_raw: &Vec<Vec<String>>) -> usize {
        let answers = from_raw.into_iter().skip(skip);
        let mut answered_count: usize = 0;

        for answer in answers {
            let mut per_category = answer.into_iter();
            let question_stmt = per_category.next().unwrap();

            let skill = match self.find_skill(question_stmt) {
                Some(t) => t,
                None => continue,
            };

            println!(">> scanning '{}: {}'", skill.name, &question_stmt);

            for grade_str in per_category {
                println!(
                    "> {}: adding grade for category: {}",
                    &question_stmt, grade_str
                );

                skill.add_response(grade_str);
            }

            answered_count += 1;
        }

        answered_count
    }

    // pub fn highest(mut self) -> EmployeeSkill {
    //     match self.max {
    //         Some(t) => t,
    //         None => {
    //             for es in self.skills.into_iter() {
    //                 if self.max.as_ref().is_none() || es.avg() > self.max.as_ref().unwrap().avg() {
    //                     self.max = Some(es)
    //                 }
    //             }

    //             self.max.unwrap()
    //         }
    //     }
    // }

    // pub fn lowest(mut self) -> EmployeeSkill {
    //     match self.min {
    //         Some(t) => t,
    //         None => {
    //             for es in self.skills.into_iter() {
    //                 if self.min.as_ref().is_none() || self.min.as_ref().unwrap().avg() < es.avg() {
    //                     self.min = Some(es)
    //                 }
    //             }

    //             self.min.unwrap()
    //         }
    //     }
    // }
}

#[derive(Eq)]
pub struct EmployeeSkill {
    pub name: Skill,
    pub question_count: u32,

    question_templates: Vec<String>,

    grades: Vec<u32>,
    texts: Vec<String>,
}

impl EmployeeSkill {
    pub fn new(name: Skill) -> EmployeeSkill {
        EmployeeSkill {
            name: name,
            question_templates: Vec::new(),
            question_count: 0,
            grades: Vec::new(),
            texts: Vec::new(),
        }
    }

    fn add_template(&mut self, v: String) {
        self.question_templates.push(v);
        self.question_count += 1;
    }

    pub fn add_response(&mut self, v: &str) {
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

use std::cmp::Ordering;

impl Ord for EmployeeSkill {
    fn cmp(&self, other: &EmployeeSkill) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for EmployeeSkill {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for EmployeeSkill {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
