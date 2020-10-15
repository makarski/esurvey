use anyhow::bail;

use super::handle_auth;
use crate::chart;
use crate::config::{self, ResponseKind};
use crate::drive;
use crate::sheets;
use crate::survey::{summary::Summary, Survey};

const SUMMARY_SHEET_NAME: &str = "Chart and Summary";
const CHART_NAME: &str = "Chart Results";

pub struct Evaluator {
    _auth_client: gauth::Auth,
}

impl Evaluator {
    pub fn new(auth_client: gauth::Auth) -> Self {
        Evaluator {
            _auth_client: auth_client,
        }
    }

    pub fn run(&self, args: clap::ArgMatches) -> anyhow::Result<()> {
        let token = self._auth_client.access_token(handle_auth)?;
        let flags = Flags::default().parse(args)?;

        println!("entered id: {}", flags.spreadsheet_id);
        println!("entered templates file: {}", flags.config_file);

        let client = sheets::Client::new();
        let spreadsheet = client.get_spreadsheet(&token.access_token, &flags.spreadsheet_id)?;
        let spreadsheet_client = drive::SpreadsheetClient::new(&client, &token.access_token);

        let templates = config::read(
            flags.config_file,
            vec![("{name}", flags.first_name.as_ref())],
        )?;

        let spreadsheet_data =
            spreadsheet_client.retrieve_sheet_data(&spreadsheet.sheets, &flags.spreadsheet_id)?;

        let mut summary = Summary::new();

        for response_kind in [ResponseKind::Grade, ResponseKind::Text].iter() {
            let templates_by_kind = templates
                .iter()
                .cloned()
                .filter(|tmplt| {
                    // todo: search for discriminators separately
                    tmplt.response_kind == *response_kind
                        || tmplt.response_kind == ResponseKind::Discriminator
                })
                .collect::<Vec<config::QuestionConfig>>();

            println!("> scanning for: {}", response_kind);

            let survey = Survey::new(&templates_by_kind);
            let responses = survey.scan_all(&spreadsheet_data)?;

            summary.set_by_kind(response_kind, responses);
        }

        let summary_sheet_id =
            spreadsheet_client.add_summary_sheet(SUMMARY_SHEET_NAME, &flags.spreadsheet_id)?;

        spreadsheet_client.save_summary(SUMMARY_SHEET_NAME, &flags.spreadsheet_id, summary)?;

        chart::add_summary_chart(
            &client,
            &token.access_token,
            &flags.spreadsheet_id,
            summary_sheet_id,
            String::from(CHART_NAME),
        )?;

        Ok(())
    }
}

#[derive(Default)]
struct Flags {
    spreadsheet_id: String,
    config_file: String,
    first_name: String,
}

impl Flags {
    fn parse(mut self, args: clap::ArgMatches) -> anyhow::Result<Self> {
        let keys = ["sheet-id", "template", "first-name"];
        for key in keys.iter() {
            if let Some(v) = args.value_of(key) {
                let v = v.to_owned();
                match *key {
                    "sheet-id" => self.spreadsheet_id = v,
                    "template" => self.config_file = v,
                    "first-name" => self.first_name = v,
                    _ => {}
                }
            } else {
                bail!("Argument `{}` not found", key);
            }
        }
        Ok(self)
    }
}
