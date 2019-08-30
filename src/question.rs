#[derive(PartialEq, Debug)]
pub struct Question {
    template: String,
    weight: f32,
}

impl Question {
    pub fn new(template: String, weight: f32) -> Self {
        Question {
            template: template,
            weight: weight,
        }
    }

    pub fn score(&self, input: f32) -> f32 {
        input * self.weight
    }

    pub fn match_template(&self, template: &String) -> bool {
        template.contains(&self.template)
    }
}

impl Eq for Question {}
