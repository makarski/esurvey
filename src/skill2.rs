use std::collections::HashMap;
use std::error::Error;

use crate::config::QuestionConfig;

#[derive(Debug)]
pub struct Responses {
    pub assessment_kind: String,
    pub category_name: String,
    vals: Vec<String>,
}

impl Responses {
    fn new(assessment_kind: String, category_name: String) -> Self {
        Responses {
            assessment_kind: assessment_kind,
            category_name: category_name,
            vals: Vec::new(),
        }
    }

    fn write(&mut self, response: &str) {
        self.vals.push(String::from(response))
    }

    pub fn read(&self) -> &Vec<String> {
        &self.vals
    }
}

pub struct Survey<'a> {
    templates: &'a Vec<QuestionConfig>,
}

impl<'a> Survey<'a> {
    pub fn new(templates: &'a Vec<QuestionConfig>) -> Result<Self, Box<dyn Error>> {
        Ok(Survey {
            templates: templates,
        })
    }

    pub fn scan(&self, raw_data: &Vec<Vec<String>>) -> Result<Vec<Responses>, Box<dyn Error>> {
        let answers = raw_data.into_iter().skip(2);
        let mut category_map: HashMap<&str, Responses> = HashMap::new();
        let mut ord_categories: Vec<&str> = Vec::new();

        for answer in answers {
            let mut per_category = answer.into_iter();
            let qst_stmt = per_category.next().ok_or(format!(
                "error scanning category question in: {:#?}",
                per_category
            ))?;

            // todo: optimize here
            let template = match self.find_config_template(qst_stmt) {
                Some(t) => t,
                None => {
                    eprintln!("> template not found for: {}", qst_stmt);
                    continue;
                }
            };

            for grade_in in per_category {
                let processed_answer = template
                    .eval_answer(grade_in.as_ref())
                    .expect(format!("failed evalling: {}", grade_in).as_ref());

                category_map
                    .entry(template.category.as_ref())
                    .and_modify(|ctgr_data| {
                        ctgr_data.write(&processed_answer);
                    })
                    .or_insert_with(|| {
                        let mut ctgr_data = Responses::new(
                            template.assessment_kind.clone(),
                            template.category.clone(),
                        );
                        ctgr_data.write(&processed_answer);
                        ord_categories.push(template.category.as_ref());
                        ctgr_data
                    });
            }
        }

        let mut category_data: Vec<Responses> = Vec::new();
        for category_name in ord_categories {
            if let Some(scanned) = category_map.remove(category_name) {
                category_data.push(scanned);
            }
        }

        Ok(category_data)
    }

    fn find_config_template(&self, input_question: &String) -> Option<&QuestionConfig> {
        self.templates
            .iter()
            .find(|tmplt| tmplt.match_template(input_question))
    }
}
