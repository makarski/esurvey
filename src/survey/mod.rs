use std::collections::HashMap;

use anyhow::anyhow;

use crate::config::{QuestionConfig, ResponseKind};
use crate::sheets::spreadsheets_values::SpreadsheetValueRange;

pub mod summary;

#[derive(Debug)]
pub struct Responses {
    pub assessment_kind: String,
    pub category_name: String,
    vals: Vec<String>,
}

impl Responses {
    fn new(assessment_kind: String, category_name: String) -> Self {
        Responses {
            assessment_kind,
            category_name,
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
    templates: &'a [QuestionConfig],
}

impl<'a> Survey<'a> {
    pub fn new(templates: &'a [QuestionConfig]) -> Self {
        Survey { templates }
    }

    pub fn scan_all(
        &self,
        from_sheets: &[SpreadsheetValueRange],
    ) -> anyhow::Result<Vec<Responses>> {
        let raw_data: Vec<&Vec<String>> = from_sheets
            .iter()
            .flat_map(|sheet_raw_data| &sheet_raw_data.values)
            .collect();

        self.scan(raw_data)
    }

    // todo:
    //   - optimize against clones
    //   - discriminator config right now works with the assumption that it is the first entry:
    //      solution: first collect discriminators, then process responses
    fn scan(&self, raw_data: Vec<&Vec<String>>) -> anyhow::Result<Vec<Responses>> {
        let answers = raw_data.into_iter().skip(2);
        let mut category_map: HashMap<String, Responses> = HashMap::new();
        let mut ord_categories: Vec<String> = Vec::new();

        let mut discriminators: Vec<String> = Vec::new();

        for answer in answers {
            let mut per_category = answer.iter();
            let qst_stmt = per_category.next().ok_or_else(|| {
                anyhow!("error scanning category question in: {:#?}", per_category)
            })?;

            // todo: optimize here
            let template = match self.find_config_template(qst_stmt) {
                Some(t) => t,
                None => {
                    eprintln!("> template not found for: {}", qst_stmt);
                    continue;
                }
            };

            for (index, grade_in) in per_category.enumerate() {
                let processed_answer = template
                    .eval_answer(grade_in.as_ref())
                    .unwrap_or_else(|_| panic!("failed evalling: {}", grade_in));

                if let ResponseKind::Discriminator = template.response_kind {
                    discriminators.push(processed_answer);
                    continue;
                };

                let assessement_title: String = match discriminators.get(index) {
                    Some(t) => t.clone(),
                    None => template.assessment_kind.clone(),
                };

                let discriminator: String = format!("{}:{}", assessement_title, template.category);

                category_map
                    .entry(discriminator.clone())
                    .and_modify(|ctgr_data| {
                        ctgr_data.write(&processed_answer);
                    })
                    .or_insert_with(|| {
                        let mut ctgr_data =
                            Responses::new(assessement_title.clone(), template.category.clone());
                        ctgr_data.write(&processed_answer);
                        ord_categories.push(discriminator);
                        ctgr_data
                    });
            }
        }

        let mut category_data: Vec<Responses> = Vec::new();
        for category_name in ord_categories {
            if let Some(scanned) = category_map.remove(&category_name) {
                category_data.push(scanned);
            }
        }

        Ok(category_data)
    }

    fn find_config_template(&self, input_question: &str) -> Option<&QuestionConfig> {
        self.templates
            .iter()
            .find(|tmplt| tmplt.match_template(input_question))
    }
}
