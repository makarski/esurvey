use std::error::Error as std_err;
use std::fmt;
use std::fmt::Display;
use std::io::{Error as io_err, ErrorKind as io_err_kind};
use std::str::FromStr;

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
    ProblemSolving,
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
            "Improvement Opportunities" => Ok(Skill::Opportunities),
            "Free Form Feedback" => Ok(Skill::FreeText),
            "Problem Solving" => Ok(Skill::ProblemSolving),
            _ => Err(io_err::new(
                io_err_kind::InvalidInput,
                format!("invalid skill category provided: {}", s),
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
            Skill::ProblemSolving => write!(f, "Problem Solving"),

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
    pub fn new(raw_cfg: &Vec<Vec<String>>) -> Result<Self, Box<dyn std_err>> {
        let mut skill_map: HashMap<Skill, EmployeeSkill> = HashMap::new();

        println!("> raw_cfg: {:?}", raw_cfg);

        for row in raw_cfg.into_iter() {
            let category = row
                .get(0)
                .ok_or(format!("failed config category: {:?}", row).as_str())?;

            let template = row
                .get(1)
                .ok_or(format!("failed to retrieve config template from: {:?}", row).as_str())?;

            let skill = Skill::from_str(category.as_str())?;
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

        Ok(EmployeeSkills {
            skills: skills,
            // skill_map: skill_map,
            max: None,
            min: None,
        })
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

    pub fn scan(
        &mut self,
        skip: usize,
        from_raw: &Vec<Vec<String>>,
    ) -> Result<usize, Box<dyn std_err>> {
        let answers = from_raw.into_iter().skip(skip);
        let mut answered_count: usize = 0;

        for answer in answers {
            let mut per_category = answer.into_iter();
            let question_stmt = per_category
                .next()
                .ok_or("error scanning category question")?;

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

                skill.add_response(grade_str)?;
            }

            answered_count += 1;
        }

        Ok(answered_count)
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
    pub question_count: usize,

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

    pub fn add_response(&mut self, v: &str) -> Result<(), Box<dyn std_err>> {
        if self.name.is_graded() {
            v.parse::<u32>()
                .map(|grade| self.add_grade(grade))
                .map_err(|err| format!("error parsing grade: {}", err.to_string()))?;
        }

        self.texts.push(v.to_owned());
        Ok(())
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
