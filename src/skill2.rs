use std::collections::HashMap;
use std::error::Error;

use crate::config::{QuestionConfig, ResponseKind};

pub trait CategoryResponse {
    fn name(&self) -> String;
    fn write(&mut self, v: &str) -> Result<(), Box<dyn Error>>;
    fn read(&self) -> Option<String>;
}

struct GradedCategory {
    name: String,
    vals: Vec<String>,
}

impl CategoryResponse for GradedCategory {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn write(&mut self, v: &str) -> Result<(), Box<dyn Error>> {
        self.vals.push(String::from(v));
        Ok(())
    }

    fn read(&self) -> Option<String> {
        let calc = self
            .vals
            .iter()
            .map(|item| {
                item.parse::<f32>()
                    .expect(format!("failed to parse: {}", item).as_ref())
            })
            .sum::<f32>()
            / self.vals.len() as f32;

        Some(calc.to_string())
    }
}

impl GradedCategory {
    fn new(name: String) -> Self {
        GradedCategory {
            name: name,
            vals: Vec::new(),
        }
    }
}

struct ReviewCategory {
    name: String,
    vals: Vec<String>,
}

impl<'a> CategoryResponse for ReviewCategory {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn write(&mut self, v: &str) -> Result<(), Box<dyn Error>> {
        self.vals.push(String::from(v));
        Ok(())
    }

    fn read(&self) -> Option<String> {
        Some(self.vals.join("\n"))
    }
}

impl ReviewCategory {
    fn new(name: String) -> Self {
        ReviewCategory {
            name: name,
            vals: Vec::new(),
        }
    }
}

pub struct Survey<'a> {
    kind: ResponseKind,
    templates: &'a Vec<QuestionConfig>,
}

impl<'a> Survey<'a> {
    pub fn new(
        kind: ResponseKind,
        templates: &'a Vec<QuestionConfig>,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Survey {
            kind: kind,
            templates: templates,
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

            let template = match self.find_config_template(qst_stmt) {
                Some(t) => t,
                None => continue,
            };

            for grade_in in per_category {
                let processed_answer = template
                    .eval_answer(grade_in.as_ref())
                    .expect(format!("failed evalling: {}, {}", grade_in, self.kind).as_ref());

                category_map
                    .entry(template.category.as_ref())
                    .and_modify(|ctgr_data| {
                        ctgr_data
                            .write(&processed_answer)
                            .expect(format!("failed response write: {}", grade_in).as_ref())
                    }) // todo: propogate unwrap
                    .or_insert_with(|| {
                        let mut ctgr_data = self.response_by_kind(template.category.clone());
                        ctgr_data.write(&processed_answer).expect(
                            format!(
                                "failed response write: {}, {}, {}",
                                grade_in,
                                ctgr_data.name(),
                                self.kind
                            )
                            .as_ref(),
                        ); // // todo: propogate unwrap
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

    fn find_config_template(&self, input_question: &String) -> Option<&QuestionConfig> {
        self.templates
            .into_iter()
            .filter(|tmplt| tmplt.response_kind == self.kind)
            .find(|tmplt| tmplt.match_template(input_question))
    }

    fn response_by_kind(&self, name: String) -> Box<dyn CategoryResponse> {
        match self.kind {
            ResponseKind::Grade => Box::new(GradedCategory::new(name)),
            ResponseKind::Text => Box::new(ReviewCategory::new(name)),
        }
    }
}
