// use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;

use crate::config::ResponseKind;
use crate::question::Question;

pub struct CategoryData {
    pub category_name: String,
    grades: Vec<f32>,
    reviews: Vec<String>,
}

impl CategoryData {
    pub fn new(name: String) -> Self {
        CategoryData {
            category_name: name,
            grades: Vec::new(),
            reviews: Vec::new(),
        }
    }

    pub fn write_grade(&mut self, v: f32) {
        self.grades.push(v)
    }

    pub fn write_review(&mut self, v: &str) {
        self.reviews.push(String::from(v))
    }

    pub fn avg(&self) -> f32 {
        self.grades.iter().sum::<f32>() / self.grades.len() as f32
    }

    pub fn reviews(&self) -> String {
        self.reviews.join("\n")
    }
}

// impl Ord for CategoryData {
//     fn cmp(&self, other: &CategoryData) -> Ordering {
//         self.category_name.cmp(&other.category_name)
//     }
// }

// impl PartialOrd for CategoryData {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl PartialEq for CategoryData {
//     fn eq(&self, other: &Self) -> bool {
//         self.category_name == other.category_name
//     }
// }

// impl Eq for CategoryData {}

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

    pub fn scan(&self, raw_data: &Vec<Vec<String>>) -> Result<Vec<CategoryData>, Box<dyn Error>> {
        let answers = raw_data.into_iter().skip(2);
        let mut category_map: HashMap<&str, CategoryData> = HashMap::new();

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

            // todo: check for a better loop
            for grade_in in per_category {
                // todo: remove duplicate code
                match self.kind {
                    ResponseKind::Grade => {
                        let f_grade = grade_in.parse::<f32>()?;
                        let score = qst.score(f_grade);

                        category_map
                            .entry(category.name.as_ref())
                            .and_modify(|ctgr_data| ctgr_data.write_grade(score))
                            .or_insert_with(|| {
                                let mut ctgr_data = CategoryData::new(category.name.clone());
                                ctgr_data.write_grade(score);
                                ctgr_data
                            });
                    }
                    ResponseKind::Text => {
                        category_map
                            .entry(category.name.as_ref())
                            .and_modify(|ctgr_data| ctgr_data.write_review(grade_in))
                            .or_insert_with(|| {
                                let mut ctgr_data = CategoryData::new(category.name.clone());
                                ctgr_data.write_review(grade_in);
                                ctgr_data
                            });
                    }
                }
            }
        }

        let mut category_data: Vec<CategoryData> = Vec::new();
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
