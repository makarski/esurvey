use std::error::Error;
use std::io::Error as io_err;
use std::io::ErrorKind as io_err_kind;

pub mod template;

pub struct ProjectsClient {
  _http_client: reqwest::Client,
}

impl ProjectsClient {
  pub fn new() -> Self {
    ProjectsClient {
      _http_client: reqwest::Client::new(),
    }
  }

  // Creates a new, empty script project with no script files and a base manifest file.
  pub fn create_project(
    &self,
    access_token: &str,
    title: String,
  ) -> Result<Project, Box<dyn Error>> {
    let url = format!(
      "https://script.googleapis.com/v1/projects?access_token={}",
      access_token
    );

    let mut resp: reqwest::Response = self
      ._http_client
      .post(url.as_str())
      .body(serde_json::to_vec(&Project {
        title: Some(title),
        ..Default::default()
      })?)
      .send()?;

    match resp.status().is_success() {
      true => Ok(resp.json::<Project>()?),
      _ => Err(Box::new(io_err::new(io_err_kind::Other, resp.text()?))),
    }
  }

  // Updates the content of the specified script project.
  pub fn update_content(
    &self,
    access_token: &str,
    script_id: &str,
    source: String,
  ) -> Result<Content, Box<dyn Error>> {
    let url = format!(
      "https://script.googleapis.com/v1/projects/{}/content?access_token={}",
      script_id, access_token
    );

    let manifest_source =
      r#"{"timeZone":"Europe/Berlin","dependencies":{},"exceptionLogging":"STACKDRIVER"}"#
        .to_owned();

    let mut resp: reqwest::Response = self
      ._http_client
      .put(url.as_str())
      .body(serde_json::to_vec(&Content {
        script_id: script_id.to_owned(),
        files: vec![
          File {
            name: "create_survey".to_owned(),
            file_type: FileType::ServerJs,
            source: source,
            ..Default::default()
          },
          File {
            name: "appsscript".to_owned(),
            file_type: FileType::Json,
            source: manifest_source,
            ..Default::default()
          },
        ],
      })?)
      .send()?;

    match resp.status().is_success() {
      true => Ok(resp.json::<Content>()?),
      _ => Err(Box::new(io_err::new(io_err_kind::Other, resp.text()?))),
    }
  }
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Project {
  // The script project's Drive ID.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub script_id: Option<String>,

  // The title for the project.
  pub title: Option<String>,

  // The parent's Drive ID that the script will be attached to.
  // This is usually the ID of a Google Document or Google Sheet.
  // This filed is optional, and if not set, a stand-alone script will be created.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub parent_id: Option<String>,

  // When the script was created.
  // A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds.
  // Example: "2014-10-02T15:01:23.045123456Z".
  #[serde(skip_serializing_if = "Option::is_none")]
  pub create_time: Option<String>,

  // When the script was last updated.
  // A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds.
  // Example: "2014-10-02T15:01:23.045123456Z".
  #[serde(skip_serializing_if = "Option::is_none")]
  pub update_time: Option<String>,

  // User who originally created the script.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub creator: Option<User>,

  // User who last modified the script.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub last_modify_user: Option<User>,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct User {
  // The user's domain.
  pub domain: String,

  // The user's identifying email address.
  pub email: String,

  // The user's display name.
  pub name: String,

  // The user's photo.
  pub photo_url: String,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct File {
  // The name of the file.
  // The file extension is not part of the file name, which can be identified from the type field.
  pub name: String,

  // The type of the file.
  #[serde(rename = "type")]
  pub file_type: FileType,

  // The file content.
  pub source: String,

  // The user who modified the file most recently.
  // This read-only field is only visible to users who have WRITER permission for the script project.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub last_modify_user: Option<User>,

  // Creation date timestamp.
  // This read-only field is only visible to users who have WRITER permission for the script project.
  // A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds.
  //
  // # Example: "2014-10-02T15:01:23.045123456Z".
  #[serde(skip_serializing_if = "Option::is_none")]
  pub create_time: Option<String>,

  // Last modified date timestamp.
  // This read-only field is only visible to users who have WRITER permission for the script project.
  // A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds.
  //
  // # Example: "2014-10-02T15:01:23.045123456Z"
  #[serde(skip_serializing_if = "Option::is_none")]
  pub update_time: Option<String>,

  // The defined set of functions in the script file, if any.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub function_set: Option<FunctionSet>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FileType {
  // Undetermined file type; never actually used.
  EnumTypeUnspecified,

  // An Apps Script server-side code file.
  ServerJs,

  // A file containing client-side HTML.
  Html,

  // A file in JSON format.
  // This type is only used for the script project's manifest.
  // The manifest file content must match the structure of a valid ScriptManifest
  Json,
}

impl Default for FileType {
  fn default() -> Self {
    FileType::EnumTypeUnspecified
  }
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
// TODO: implememt
pub struct FunctionSet {}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
  //  The script project's Drive ID.
  pub script_id: String,

  //  The list of script project files.
  // One of the files is a script manifest; it must be named "appsscript",
  // must have type of JSON, and include the manifest configurations for the project.
  pub files: Vec<File>,
}
