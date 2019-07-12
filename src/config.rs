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
    pub fn config_grades(&self) -> Vec<Vec<String>> {
        self.config().0
    }

    pub fn config_texts(&self) -> Vec<Vec<String>> {
        self.config().1
    }

    fn config(&self) -> (Vec<Vec<String>>, Vec<Vec<String>>) {
        match self {
            AssessmentKind::SelfAssessment => (
                vec![
                    vec!["Category".to_owned(), "Template".to_owned()],
                    vec!["Adaptability".to_owned(), "I constructively approach new and unexpected events or announcements.".to_owned()],
                    vec!["Adaptability".to_owned(), "I am open to work on the tasks not entirely related to my position.".to_owned()],
                    vec!["Attitude".to_owned(), "I aim to create a positive working environment around myself.".to_owned()],
                    vec!["Attitude".to_owned(), "I react constructively after making a mistake.".to_owned()],
                    vec!["Communication".to_owned(), "I communicate ideas well verbally.".to_owned()],
                    vec!["Communication".to_owned(), "I don't have misunderstandings with my teammates.".to_owned()],
                    vec!["Communication".to_owned(), "I communicate ideas well in writing.".to_owned()],
                    vec!["Cross-functional Knowledge".to_owned(), "I am open to learn new things and expand my professional expertise.".to_owned()],
                    vec!["Cross-functional Knowledge".to_owned(), "I have knowledge and experience that goes beyond my job responsibilities, i.e., tools, methodologies, frameworks.".to_owned()],
                    vec!["Dependability".to_owned(), "I achieve my objectives and goals.".to_owned()],
                    vec!["Dependability".to_owned(), "I follow team standards and processes.".to_owned()],
                    vec!["Dependability".to_owned(), "I pay attention to detail.".to_owned()],
                    vec!["Initiative".to_owned(), "I am proactive in suggesting solutions and improvements, i.e., code quality, process, etc.".to_owned()],
                    vec!["Initiative".to_owned(), "I take fast and informed decisions.My teammates trust the choices that I make.".to_owned()],
                    vec!["Leadership".to_owned(), "I constructively handle (critical) feedback.".to_owned()],
                    vec!["Leadership".to_owned(), "I provide constructive feedback to individual teammates and the team.".to_owned()],
                    vec!["Leadership".to_owned(), "I value contributions and opinions of other teammates.".to_owned()],
                    vec!["Organization".to_owned(), "I am well organized.".to_owned()],
                    vec!["Organization".to_owned(), "I can organize teamwork.".to_owned()],
                    vec!["Organization".to_owned(), "I am a productive employee.".to_owned()],
                    vec!["Responsibility".to_owned(), "I keep personal involvement high and support the team when the project gets delayed or stuck.".to_owned()],
                    vec!["Responsibility".to_owned(), "I always finish my initiatives.".to_owned()],
                    vec!["Self-Improvement".to_owned(), "I set clear and measurable goals for myself.".to_owned()],
                    vec!["Self-Improvement".to_owned(), "I use means at my disposal (conferences, books, webinars) for continuous self-improvement.".to_owned()],
                    vec!["Teamwork".to_owned(), "I offer support to the other team members when needed.".to_owned()],
                    vec!["Teamwork".to_owned(), "I am a team player.".to_owned()],
                    vec!["Teamwork".to_owned(), "I collaborate and build a relationship with the team.".to_owned()],
                    vec!["Tech. Expertise".to_owned(), "I possess all required (tech.) skills for doing my job.".to_owned()],
                    vec!["Tech. Expertise".to_owned(), "I can teach to other teammates in some/any technical discipline.".to_owned()],
                ],
                vec![
                    vec!["Category".to_owned(), "Template".to_owned()],
                    vec!["New Skill".to_owned(), "This is what my team or individual teammates have learned from me.".to_owned()],
                    vec!["Skill to Acquire".to_owned(), "This is what I still could learn from my team.".to_owned()],
                    vec!["Strengths".to_owned(), "Strengths".to_owned()],
                    vec!["Improvement Oppotunities".to_owned(), "Improvements".to_owned()],
                ]
            ),
            AssessmentKind::TeamFeedback => (
                vec![
                    vec!["Category".to_owned(), "Template".to_owned()],
                    vec!["Adaptability".to_owned(), "constructively approaches new and unexpected events or announcements.".to_owned()],
                    vec!["Adaptability".to_owned(), "is open to work on the tasks not entirely related to his/her position.".to_owned()],
                    vec!["Attitude".to_owned(), "aims to create a positive working environment around him/herself.".to_owned()],
                    vec!["Attitude".to_owned(), "reacts constructively after making a mistake.".to_owned()],
                    vec!["Communication".to_owned(), "communicates ideas well verbally.".to_owned()],
                    vec!["Communication".to_owned(), "I don't have misunderstandings with".to_owned()],
                    vec!["Communication".to_owned(), "communicates ideas well in writing.".to_owned()],
                    vec!["Cross-functional Knowledge".to_owned(), "is open to learn new things and expand his/her professional expertise.".to_owned()],
                    vec!["Cross-functional Knowledge".to_owned(), "has knowledge and experience that goes beyond their job responsibilities, i.e. tools, methodologies, frameworks.".to_owned()],
                    vec!["Dependability".to_owned(), "achieves his/her objectives and goals.".to_owned()],
                    vec!["Dependability".to_owned(), "follows team standards and processes.".to_owned()],
                    vec!["Dependability".to_owned(), "pays attention to detail.".to_owned()],
                    vec!["Initiative".to_owned(), "is proactive suggesting solutions and improvements, .e. code quality, process, etc.".to_owned()],
                    vec!["Initiative".to_owned(), "takes fast and informed decisions.".to_owned()],
                    vec!["Leadership".to_owned(), "I trust the decisions that".to_owned()],
                    vec!["Leadership".to_owned(), "constructively handles (critical) feedback.".to_owned()],
                    vec!["Leadership".to_owned(), "provides constructive feedback to the team and me.".to_owned()],
                    vec!["Leadership".to_owned(), "values contributions and opinions of other teammates.".to_owned()],
                    vec!["Organization".to_owned(), "is well organized.".to_owned()],
                    vec!["Organization".to_owned(), "can organize teamwork.".to_owned()],
                    vec!["Organization".to_owned(), "is a productive employee.".to_owned()],
                    vec!["Responsibility".to_owned(), "keeps personal involvement high and supports the team when the project gets delayed/stuck.".to_owned()],
                    vec!["Responsibility".to_owned(), "always finishes his/her initiatives.".to_owned()],
                    vec!["Self-Improvement".to_owned(), "sets clear and measurable goals for him/her-self.".to_owned()],
                    vec!["Self-Improvement".to_owned(), "uses means at his/her disposal (conferences, books, webinars) for continuous self-improvement.".to_owned()],
                    vec!["Teamwork".to_owned(), "offers support to the other team members when needed.".to_owned()],
                    vec!["Teamwork".to_owned(), "is a team player, collaborates and builds a relationship with the team.".to_owned()],
                    vec!["Tech. Expertise".to_owned(), "possesses all required (tech.) skills for doing his/her job.".to_owned()],
                    vec!["Tech. Expertise".to_owned(), "can teach other teammates in some/any technical discipline.".to_owned()],
                ],
                vec![
                    vec!["Category".to_owned(), "Template".to_owned()],
                    vec!["New Skill".to_owned(), "This is what I have learned from".to_owned()],
                    vec!["Skill to Acquire".to_owned(), "This is what I could share knowledge wise with".to_owned()],
                    vec!["Strengths".to_owned(), "Strengths".to_owned()],
                    vec!["Improvement Oppotunities".to_owned(), "Improvements".to_owned()],
                    vec!["Free Form Feedback".to_owned(), "Text that will be shared".to_owned()],
                ]
            )
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

        let rows = raw_cfg.into_iter().skip(1);

        for row in rows {
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
