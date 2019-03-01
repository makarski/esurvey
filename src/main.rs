extern crate csv;

use std::cell::RefCell;
use std::fmt;
use std::io;

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

impl Skill {
    fn description(&self) -> &str {
        match self {
            Skill::Adaptability => "Adaptability",
            Skill::Attitude => "Attitude",
            Skill::Communication => "Communication",
            Skill::CrossFunctionalKnowledge => "Cross-functional Knowledge",
            Skill::Dependability => "Dependability",
            Skill::Initiative => "Initiative",
            Skill::Leadership => "Leadership",
            Skill::Organization => "Organization",
            Skill::Responsibility => "Responsibility",
            Skill::SelfImprovement => "Self-Improvement",
            Skill::Teamwork => "Teamwork",
            Skill::TechExpertise => "Tech. Expertise",
        }
    }
}

impl fmt::Display for Skill {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Skill::Adaptability => write!(f, "{}", self.description()),
            Skill::Attitude => write!(f, "{}", self.description()),
            Skill::Communication => write!(f, "{}", self.description()),
            Skill::CrossFunctionalKnowledge => write!(f, "{}", self.description()),
            Skill::Dependability => write!(f, "{}", self.description()),
            Skill::Initiative => write!(f, "{}", self.description()),
            Skill::Leadership => write!(f, "{}", self.description()),
            Skill::Organization => write!(f, "{}", self.description()),
            Skill::Responsibility => write!(f, "{}", self.description()),
            Skill::SelfImprovement => write!(f, "{}", self.description()),
            Skill::Teamwork => write!(f, "{}", self.description()),
            Skill::TechExpertise => write!(f, "{}", self.description()),
        }
    }
}

struct EmployeeSkill {
    name: Skill,
    questions: u32,
    grades: RefCell<Vec<u32>>,
}

impl EmployeeSkill {
    fn new(n: Skill, q: u32) -> EmployeeSkill {
        EmployeeSkill {
            name: n,
            questions: q,
            grades: RefCell::new(Vec::with_capacity(q as usize)),
        }
    }

    fn avg(&self) -> f32 {
        self.grades.borrow().iter().sum::<u32>() as f32 / self.grades.borrow().len() as f32
    }
}

impl fmt::Display for EmployeeSkill {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.avg())
    }
}

fn main() {
    let mut config_questions: Vec<u32>;

    let feedback_type_in = std::env::args()
        .nth(1)
        .expect("no feedback type provided! valid values: `team-feedback` or `self-assessment`");

    match feedback_type_in.as_str() {
        "team-feedback" => config_questions = vec![2, 2, 3, 2, 3, 2, 4, 3, 2, 2, 2, 2],
        "self-assessment" => config_questions = vec![2, 2, 3, 2, 3, 2, 3, 3, 2, 2, 3, 2],
        _ => {
            panic!("no feedback type provided! valid values: `team-feedback` or `self-assessment`")
        }
    }

    let mut cfg_iter = config_questions.into_iter();

    let questions: Vec<EmployeeSkill> = vec![
        EmployeeSkill::new(Skill::Adaptability, cfg_iter.next().unwrap()),
        EmployeeSkill::new(Skill::Attitude, cfg_iter.next().unwrap()),
        EmployeeSkill::new(Skill::Communication, cfg_iter.next().unwrap()),
        EmployeeSkill::new(Skill::CrossFunctionalKnowledge, cfg_iter.next().unwrap()),
        EmployeeSkill::new(Skill::Dependability, cfg_iter.next().unwrap()),
        EmployeeSkill::new(Skill::Initiative, cfg_iter.next().unwrap()),
        EmployeeSkill::new(Skill::Leadership, cfg_iter.next().unwrap()),
        EmployeeSkill::new(Skill::Organization, cfg_iter.next().unwrap()),
        EmployeeSkill::new(Skill::Responsibility, cfg_iter.next().unwrap()),
        EmployeeSkill::new(Skill::SelfImprovement, cfg_iter.next().unwrap()),
        EmployeeSkill::new(Skill::Teamwork, cfg_iter.next().unwrap()),
        EmployeeSkill::new(Skill::TechExpertise, cfg_iter.next().unwrap()),
    ];

    let mut rdr = csv::Reader::from_reader(io::stdin());
    for colleague in rdr.records() {
        let feedback = colleague.expect("could not read colleague's feedback");
        let mut feedback_iter = feedback.iter();

        for q in &questions {
            println!("\nScanning '{}'...", q.name);
            let mut counter: u32 = 0;

            loop {
                let answer = feedback_iter
                    .next()
                    .expect("could not retrieve the next value");
                let grade: u32 = answer.parse().expect("could not parse grade");

                println!(">> recording grade: {}, counter is: {}", grade, counter);

                q.grades.borrow_mut().push(grade);

                // break to the next category
                counter = counter + 1;

                println!("counter: {}; questions: {}", counter, q.questions);
                if counter == q.questions {
                    println!("i break {} == {}", counter, q.questions);
                    break;
                }
            }
        }
    }

    let mut wrt = csv::WriterBuilder::new()
        .from_path(format!("{}.csv", feedback_type_in))
        .unwrap();

    for i in 0..2 {
        println!("> i value is {}", i);

        for skill in &questions {
            if i == 0 {
                wrt.write_field(skill.name.description()).unwrap();
            } else {
                wrt.write_field(format!("{}", skill.avg())).unwrap();
            }
        }
        wrt.write_record(None::<&[u8]>).unwrap();
    }
}
