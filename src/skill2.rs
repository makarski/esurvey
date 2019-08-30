use std::collections::HashMap;
use std::error::Error;

use crate::config::ResponseKind;
use crate::question::Question;

pub trait CategoryResponse {
    fn name(&self) -> String;
    fn write(&mut self, q: &Question, v: &str) -> Result<(), Box<dyn Error>>;
    fn read(&self) -> Option<String>;
}

struct GradedCategory {
    name: String,
    grades: Vec<f32>,
}

impl CategoryResponse for GradedCategory {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn write(&mut self, q: &Question, v: &str) -> Result<(), Box<dyn Error>> {
        let f = v.parse::<f32>()?;
        self.grades.push(q.score(f));
        Ok(())
    }

    fn read(&self) -> Option<String> {
        let calc = self.grades.iter().sum::<f32>() / self.grades.len() as f32;
        Some(calc.to_string())
    }
}

impl GradedCategory {
    fn new(name: String) -> Self {
        GradedCategory {
            name: name,
            grades: Vec::new(),
        }
    }
}

struct ReviewCategory {
    name: String,
    reviews: Vec<String>,
}

impl CategoryResponse for ReviewCategory {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn write(&mut self, _: &Question, v: &str) -> Result<(), Box<dyn Error>> {
        self.reviews.push(String::from(v));
        Ok(())
    }

    fn read(&self) -> Option<String> {
        Some(self.reviews.join("\n"))
    }
}

impl ReviewCategory {
    fn new(name: String) -> Self {
        ReviewCategory {
            name: name,
            reviews: Vec::new(),
        }
    }
}

pub struct Survey {
    kind: ResponseKind,
    categories: Vec<Category>,
}

impl Survey {
    pub fn new(kind: ResponseKind, raw_cfg: &Vec<Vec<String>>) -> Result<Self, Box<dyn Error>> {
        let mut category_map: HashMap<&str, Category> = HashMap::new();

        for row in raw_cfg.into_iter() {
            let (category_in, template, weight_in) = (
                row.get(0)
                    .ok_or(format!("failed config category: {:?}", row))?,
                row.get(1).ok_or(
                    format!("failed to retrieve config template from: {:?}", row).as_str(),
                )?,
                row.get(2)
                    .ok_or(format!("failed to retrieve config weight: {:?}", row))?,
            );

            let weight = weight_in.parse::<f32>()?;
            let category_ref: &str = category_in.as_ref();

            category_map
                .entry(category_ref)
                .and_modify(|category| category.add_question(template.to_string(), weight))
                .or_insert_with(|| -> Category {
                    let mut category = Category::new(category_in.to_string());
                    category.add_question(template.to_string(), weight);
                    category
                });
        }

        let mut categories: Vec<Category> = Vec::new();
        category_map.drain().for_each(|(_, c)| categories.push(c));

        Ok(Survey {
            kind: kind,
            categories: categories,
        })
    }

    pub fn scan(
        &self,
        raw_data: &Vec<Vec<String>>,
    ) -> Result<Vec<Box<dyn CategoryResponse>>, Box<dyn Error>> {
        let answers = raw_data.into_iter().skip(2);
        let mut category_map: HashMap<&str, Box<dyn CategoryResponse>> = HashMap::new();

        for answer in answers {
            let mut per_category = answer.into_iter();
            let qst_stmt = per_category.next().ok_or(format!(
                "error scanning category question in: {:?}",
                per_category
            ))?;

            let (category, qst) = match self.find_category(qst_stmt) {
                Some(t) => t,
                None => continue,
            };

            for grade_in in per_category {
                category_map
                    .entry(category.name.as_ref())
                    .and_modify(|ctgr_data| ctgr_data.write(qst, grade_in).unwrap()) // todo: propogate unwrap
                    .or_insert_with(|| {
                        let mut ctgr_data = self.response_by_kind(category.name.clone());
                        ctgr_data.write(qst, grade_in).unwrap(); // // todo: propogate unwrap
                        ctgr_data
                    });
            }
        }

        let mut category_data: Vec<Box<dyn CategoryResponse>> = Vec::new();
        category_map
            .drain()
            .for_each(|(_, v)| category_data.push(v));

        Ok(category_data)
    }

    fn find_category(&self, input_question: &String) -> Option<(&Category, &Question)> {
        for category in &self.categories {
            if let Some(qst) = category.find_question(input_question) {
                return Some((&category, qst));
            }
        }
        None
    }

    fn response_by_kind(&self, name: String) -> Box<dyn CategoryResponse> {
        match self.kind {
            ResponseKind::Grade => Box::new(GradedCategory::new(name)),
            ResponseKind::Text => Box::new(ReviewCategory::new(name)),
        }
    }
}

#[derive(Debug)]
pub struct Category {
    name: String,
    questions: Vec<Question>,
}

impl Category {
    pub fn new(name: String) -> Self {
        Category {
            name: name,
            questions: Vec::new(),
        }
    }

    pub fn find_question(&self, input_question: &String) -> Option<&Question> {
        for qst in &self.questions {
            if qst.match_template(input_question) {
                return Some(&qst);
            }
        }
        None
    }

    pub fn add_question(&mut self, template: String, weight: f32) {
        self.questions.push(Question::new(template, weight))
    }
}
