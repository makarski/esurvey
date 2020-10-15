use std::default::Default;

use anyhow::{anyhow, bail};

use crate::appsscript::{template::Template, ProjectsClient};
use crate::config::{self, QuestionConfig, ResponseKind};

pub struct Generator {
    _auth_client: gauth::Auth,
}

impl Generator {
    pub fn new(auth_client: gauth::Auth) -> Self {
        Generator {
            _auth_client: auth_client,
        }
    }

    pub fn run(&self, args: clap::ArgMatches) -> anyhow::Result<()> {
        let flags = Flags::default().parse(args)?;
        let token = self._auth_client.access_token(super::handle_auth)?;

        let title = format!(
            "{}-{}-{}-{}",
            &flags.assessment_kind, &flags.first_name, &flags.last_name, &flags.occasion
        );

        let templates = config::read(&flags.template_file, vec![("{name}", &flags.first_name)])?;

        let graded_questions =
            self.config_questions(&templates, ResponseKind::Grade, &flags.assessment_kind)?;
        let text_questions =
            self.config_questions(&templates, ResponseKind::Text, &flags.assessment_kind)?;

        // println!("{:#?}", graded_questions);
        // println!("{:#?}", text_questions);

        let code_template = Template::new(
            flags.assessment_kind.as_ref(),
            flags.first_name,
            flags.last_name,
            flags.occasion,
            flags.drive_dir_id,
            flags.description,
            graded_questions,
            text_questions,
        );

        let projects_client = ProjectsClient::new();
        let project = projects_client.create_project(&token.access_token, title)?;
        let script_id = project
            .script_id
            .ok_or_else(|| anyhow!("could not retrieve script_id"))?;

        projects_client.update_content(
            &token.access_token,
            script_id.as_ref(),
            code_template.code(),
        )?;

        Ok(())
    }

    fn config_questions(
        &self,
        templates: &[QuestionConfig],
        response_kind: ResponseKind,
        assessment_kind: &str,
    ) -> anyhow::Result<Vec<String>> {
        Ok(templates
            .iter()
            .filter(|question| {
                (question.response_kind == response_kind)
                    && (question.assessment_kind.to_lowercase() == *assessment_kind.to_lowercase())
            })
            .map(|question| question.template_final.to_string())
            .collect::<Vec<String>>())
    }
}

#[derive(Default, Debug)]
struct Flags {
    assessment_kind: String,
    first_name: String,
    last_name: String,
    occasion: String,
    drive_dir_id: String,
    template_file: String,
    description: String,
}

impl Flags {
    fn parse(mut self, args: clap::ArgMatches) -> anyhow::Result<Self> {
        let keys = [
            "kind",
            "first-name",
            "last-name",
            "occasion",
            "dir",
            "template",
            "description",
        ];

        for key in keys.iter() {
            if let Some(v) = args.value_of(key) {
                let v = v.to_owned();

                match *key {
                    "kind" => self.assessment_kind = v,
                    "first-name" => self.first_name = v,
                    "last-name" => self.last_name = v,
                    "occasion" => self.occasion = v,
                    "dir" => self.drive_dir_id = v,
                    "template" => self.template_file = v,
                    "description" => self.description = v,
                    _ => {}
                };
            } else {
                bail!("Argument `{}` not found", key);
            }
        }

        Ok(self)
    }
}
