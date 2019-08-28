use std::default::Default;
use std::env::args;
use std::error::Error;
use std::io::{Error as io_err, ErrorKind as io_err_kind};
use std::str::FromStr;

use crate::appsscript;
use appsscript::template::Template;
use appsscript::ProjectsClient;

use crate::config::{AssessmentKind, ResponseKind};

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

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let flags = parse_flags()?;
        let token = self._auth_client.access_token(super::handle_auth)?;

        let title = format!(
            "{}-{}-{}-{}",
            &flags.assessment_kind, &flags.first_name, &flags.last_name, &flags.occasion
        );

        // todo: incorporate or rething text based questions
        let graded_questions = self.read_config(&flags.assessment_kind, &flags.template_file)?;

        let code_template = Template::new(
            flags.assessment_kind.as_ref(),
            flags.first_name,
            flags.last_name,
            flags.occasion,
            flags.drive_dir_id,
            flags.description,
            graded_questions,
            Vec::new(), // add text-based questions
        );

        let projects_client = ProjectsClient::new();
        let project = projects_client.create_project(&token.access_token, title)?;
        let script_id = project.script_id.ok_or("could not retrieve script_id")?;

        projects_client.update_content(
            &token.access_token,
            script_id.as_ref(),
            code_template.code(),
        )?;

        Ok(())
    }

    fn read_config(
        &self,
        assessment_kind: &str,
        template_file: &str,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        let kind = AssessmentKind::from_str(assessment_kind)?;
        let kind_question = kind.config(template_file, ResponseKind::Grade)?;

        Ok(kind_question
            .into_iter()
            .map(|item| item[1].clone())
            .collect::<Vec<String>>())
    }
}

fn parse_flags() -> Result<Flags, Box<dyn Error>> {
    let mut flags = Flags::default();

    for pair in args().skip(2).collect::<Vec<String>>() {
        let pairs = pair.split_terminator("=").collect::<Vec<&str>>();

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
            "-template" => flags.template_file = v,
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
        ("-template", &flags.template_file),
        ("-description", &flags.description),
    ]
    .iter()
    {
        if flag_entry.is_empty() {
            empty_fields.push(flag_name);
        }
    }

    if empty_fields.len() == 0 {
        return Ok(flags);
    }

    return Err(Box::new(io_err::new(
        io_err_kind::InvalidInput,
        format!("missing flag args: {:?}", empty_fields.join(", ")),
    )));
}
