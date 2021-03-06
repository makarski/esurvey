Setup
-----

1. Prerequisite: the feedback form results have been exported to a Google Spreadsheet.
   1. [ ] **TODO** check [Apps Script API](https://developers.google.com/apps-script/api/) to automatically export data to google spreadsheets.
2. The Spreadsheet **MUST** have two sheets (tabs) named `team-feedback`, `self-assessment`.
3. You have configured an app from [Google API Console](https://console.developers.google.com/apis/credentials):
   1. Enter the application name
   2. Set type to `Other`
   3. Download json configuration file
   4. Set `OAUTH_CFG_FILE` env var to hold the path to the downloaded `JSON` file, i.e. `export OAUTH_CFG_FILE=/mypath/oauth-credentials.json`

Running
-------

#### Generate [Apps Script](https://developers.google.com/apps-script/api) for automatic form generation

```sh
OAUTH_CFG_FILE=/mypath/credentials.json esurvey \
    --kind=Team-Feedback \
    --first-name=John \
    --last-name=Smith \
    --occasion=annual-review \
    --dir={google-drive-put-the-generated-form-here} \
    --template=overall-grading.csv \
    --description='my description' \
    generate
```

#### Read and Process the Data

The following command will read the data from the Spreadsheet and create a new Sheet (tab) `Chart and Summary` with processed and categorised data

```sh
OAUTH_CFG_FILE=/mypath/credentials.json esurvey \
    --sheet-id={google_spreadsheet_id} \
    --template=overall-grading.csv \
    --first-name=John \  
    eval
```

#### Survey Configuration

The application is configured in an `csv` file. The name of the file is required to provide under the `-templates` flag.

Templates file structure:

- AssessmentKind (`String`) - Description here
- ResponseKind (`Enum`) - Description here
    - `Grade`
    - `Text`
    - `Discriminator`
- Category (`String`) - Description here
- Template (`String`) - Description here
- Weight (`Float`) - Description here
