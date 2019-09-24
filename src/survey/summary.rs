use crate::config::ResponseKind;
use crate::survey::Responses;
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Debug, Default)]
pub struct Summary {
    texts: Vec<Responses>,
    grades: Vec<Responses>,
}

impl Summary {
    pub fn new() -> Self {
        Summary::default()
    }

    pub fn set_by_kind(&mut self, response_kind: &ResponseKind, v: Vec<Responses>) {
        match response_kind {
            &ResponseKind::Grade => self.grades = v,
            &ResponseKind::Text => self.texts = v,
            &ResponseKind::Discriminator => {}
        }
    }

    pub fn generate_rows(self) -> Vec<SummaryRows> {
        let mut all_rows = Vec::with_capacity(2);
        let response_kinds = [ResponseKind::Grade, ResponseKind::Text].into_iter();

        for (response_kind, data) in response_kinds.zip([self.grades, self.texts].into_iter()) {
            if let Some(rows) = generate_summary_rows(response_kind, data) {
                all_rows.push(rows);
            }
        }

        all_rows
    }
}

fn generate_summary_rows(
    response_kind: &ResponseKind,
    data: &Vec<Responses>,
) -> Option<SummaryRows> {
    let assessment_kind = |r: &Responses| -> String { r.assessment_kind.clone() };
    let category_name = |r: &Responses| -> String { r.category_name.clone() };

    match response_kind {
        ResponseKind::Grade => Some(fill_summary_rows(
            response_kind,
            data,
            Box::new(category_name),
            Box::new(assessment_kind),
        )),
        ResponseKind::Text => Some(fill_summary_rows(
            response_kind,
            data,
            Box::new(assessment_kind),
            Box::new(category_name),
        )),
        ResponseKind::Discriminator => None,
    }
}

fn fill_summary_rows(
    response_kind: &ResponseKind,
    by_category: &Vec<Responses>,
    header: Box<dyn Fn(&Responses) -> String>,
    cell_key: Box<dyn Fn(&Responses) -> String>,
) -> SummaryRows {
    let mut rows = SummaryRows::new();
    for category in by_category {
        rows.add_header("Data", header(&category).as_ref());

        rows.add_cell(
            cell_key(&category).as_ref(),
            response_kind
                .process_data(category.read())
                .unwrap() // todo: check unwrap
                .as_ref(),
        );
    }

    rows
}

pub struct SummaryRows {
    base: HashMap<String, Vec<String>>,
    ordered_keys: Vec<String>,
}

impl SummaryRows {
    fn new() -> Self {
        SummaryRows {
            base: HashMap::new(),
            ordered_keys: Vec::new(),
        }
    }

    fn add_header(&mut self, group_key: &str, v: &str) {
        if let Some(_) = self.unique_entry_exists(group_key, v) {
            return;
        }
        self.add_cell(group_key, v);
    }

    fn add_cell(&mut self, group_key: &str, v: &str) {
        let exists = self.cell_entry_exists(group_key);

        // pattern matching is used as a workaround for borrow checker
        // since we need to append to ordered_keys that needs mutable access to self
        // that results in 2 mut borrows for inserting into map and a vector
        match exists {
            Some(_) => self.append(group_key, v),
            None => self.insert(group_key, v),
        };
    }

    // as opposed to cell_entry_exists this function allows
    // to have only unique header values
    fn unique_entry_exists(&self, group_key: &str, v: &str) -> Option<&String> {
        if let Some(unique_vals) = self.cell_entry_exists(group_key) {
            return unique_vals.iter().find(|&name| name == v);
        }

        None
    }

    fn cell_entry_exists(&self, group_key: &str) -> Option<&Vec<String>> {
        self.base.get(group_key)
    }

    fn append(&mut self, group_key: &str, v: &str) {
        self.base
            .entry(String::from(group_key))
            .and_modify(|e| e.push(String::from(v)));
    }

    fn insert(&mut self, group_key: &str, v: &str) {
        self.base.insert(
            String::from(group_key),
            vec![String::from(group_key), String::from(v)],
        );

        self.ordered_keys.push(String::from(group_key));
    }

    pub fn rows(&self) -> Vec<Vec<String>> {
        let mut out: Vec<Vec<String>> = Vec::new();
        for key in &self.ordered_keys {
            let key_str: &str = key.as_ref();
            if let Some(v) = self.base.get(key_str) {
                out.push(v.deref().to_vec());
            }
        }

        out
    }
}
