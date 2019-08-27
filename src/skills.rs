use std::collections::HashMap;
use std::error::Error as std_err;
use std::fmt;
use std::fmt::Display;

use crate::config::ResponseKind;

pub struct EmployeeSkills {
    pub skills: Vec<EmployeeSkill>,
}

impl EmployeeSkills {
    pub fn new(
        raw_cfg: &Vec<Vec<String>>,
        response_kind: ResponseKind,
    ) -> Result<Self, Box<dyn std_err>> {
        let mut skill_map: HashMap<&str, EmployeeSkill> = HashMap::new();

        println!("> raw_cfg: {:?}", raw_cfg);

        for row in raw_cfg.into_iter() {
            let category = row
                .get(0)
                .ok_or(format!("failed config category: {:?}", row))?;

            let template = row
                .get(1)
                .ok_or(format!("failed to retrieve config template from: {:?}", row).as_str())?;

            let cat_ref: &str = category.as_ref();

            if let Some(employee_skill) = skill_map.get_mut(cat_ref) {
                println!(">> adding config template: {}: {}", category, template);

                employee_skill.add_template(template.to_string());
            } else {
                let mut employee_skill =
                    EmployeeSkill::new(category.to_owned(), response_kind.clone());

                employee_skill.add_template(template.to_string());
                println!(">> adding config template: {}: {}", category, template);

                skill_map.insert(category.as_ref(), employee_skill);
            }
        }

        let mut skills: Vec<EmployeeSkill> = Vec::new();
        skill_map.drain().for_each(|(_, es)| skills.push(es));
        skills.sort_by(|a, b| a.cmp(b));

        Ok(EmployeeSkills { skills: skills })
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
}

#[derive(Eq)]
pub struct EmployeeSkill {
    pub name: String,
    response_kind: ResponseKind,
    question_templates: Vec<String>,
    grades: Vec<u32>,
    texts: Vec<String>,
}

impl EmployeeSkill {
    pub fn new(name: String, response_kind: ResponseKind) -> EmployeeSkill {
        EmployeeSkill {
            name: name,
            response_kind: response_kind,
            question_templates: Vec::new(),
            grades: Vec::new(),
            texts: Vec::new(),
        }
    }

    fn add_template(&mut self, v: String) {
        self.question_templates.push(v);
    }

    fn is_graded(&self) -> bool {
        match self.response_kind {
            ResponseKind::Grade => true,
            ResponseKind::Text => false,
        }
    }

    pub fn add_response(&mut self, v: &str) -> Result<(), Box<dyn std_err>> {
        if self.is_graded() {
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
}

impl Display for EmployeeSkill {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_graded() {
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
