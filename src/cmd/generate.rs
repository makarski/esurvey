use std::default::Default;
use std::env::args;

use anyhow::{anyhow, bail};

use crate::appsscript::{template::Template, ProjectsClient};
use crate::config::{self, QuestionConfig, ResponseKind};

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

pub struct Generator {
    _auth_client: gauth::Auth,
}

impl Generator {
    pub fn new(auth_client: gauth::Auth) -> Self {
        Generator {
            _auth_client: auth_client,
        }
    }

    pub fn run(&self) -> anyhow::Result<()> {
        let flags = parse_flags()?;
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

fn parse_flags() -> anyhow::Result<Flags> {
    let mut flags = Flags::default();

    for pair in args().skip(2).collect::<Vec<String>>() {
        let pairs = pair.split_terminator('=').collect::<Vec<&str>>();

        if pairs.len() < 2 {
            continue;
        }

        let v = pairs[1].to_owned();

        match pairs[0] {
            "-kind" => flags.assessment_kind = v,
            "-first-name" => flags.first_name = v,
            "-last-name" => flags.last_name = v,
            "-occasion" => flags.occasion = v,
            "-dir-id" => flags.drive_dir_id = v,
            "-templates" => flags.template_file = v,
            "-description" => flags.description = v,
            _ => {}
        };
    }

    let mut empty_fields: Vec<&str> = Vec::new();

    for (flag_name, flag_entry) in [
        ("-kind", &flags.assessment_kind),
        ("-first-name", &flags.first_name),
        ("-last-name", &flags.last_name),
        ("-occasion", &flags.occasion),
        ("-dir-id", &flags.drive_dir_id),
        ("-templates", &flags.template_file),
        ("-description", &flags.description),
    ]
    .iter()
    {
        if flag_entry.is_empty() {
            empty_fields.push(flag_name);
        }
    }

    if empty_fields.is_empty() {
        Ok(flags)
    } else {
        bail!("missing flag args: {:#?}", empty_fields.join(", "))
    }
}
