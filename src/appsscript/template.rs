pub struct Template {
    assessment_kind: String,
    first_name: String,
    last_name: String,
    occasion: String,
    dir_id: String,
    description: String,
    graded_questions: Vec<String>,
    text_questions: Vec<String>,
}

impl Template {
    pub fn new(
        assessment_kind: String,
        first_name: String,
        last_name: String,
        occasion: String,
        dir_id: String,
        description: String,
        graded_questions: Vec<String>,
        text_questions: Vec<String>,
    ) -> Self {
        Template {
            assessment_kind,
            first_name,
            last_name,
            occasion,
            dir_id,
            description,
            graded_questions,
            text_questions,
        }
    }

    pub fn code(&self) -> String {
        let concat_graded_qs = self.graded_questions.join("\",\"");
        let concat_text_qs = self.text_questions.join("\",\"");

        format!(
    r###"function createForm() {{
   // configuration
   var cPersonName = "{first_name}";
   var cPersonSurname = "{last_name}";
   var cOccasion = "{occasion}";
   var folderId = "{dir_id}";
  
   // create & name Form  
   var auxDate = new Date();
   var formDate = auxDate.getFullYear().toString() + (auxDate.getMonth() + 1).toString() + auxDate.getDate().toString();

   var item = "{assessment_kind}: ##name## ##surname## - ##occasion##"
      .replace("##name##", cPersonName)
      .replace("##surname##", cPersonSurname)
      .replace("##occasion##", cOccasion);

   var itemDesc = "{description}";
   var form = FormApp.create(item)  
       .setTitle(item)
       .setDescription(itemDesc)
       .setCollectEmail(true)
       .setLimitOneResponsePerUser(true)
       .setShuffleQuestions(true)
       .setShowLinkToRespondAgain(false)
       .setProgressBar(true);

   // section 1 agree or disagree   
   var pageTwo = form.addPageBreakItem()
       .setTitle("Agree or Disagree")
       .setGoToPage(FormApp.PageNavigationType.CONTINUE)
       .setHelpText("Agree or disagree with the provided statements. The scale should be interpreted as follows: 1 - strongly disagree, 5 - neutral, 10 - strongly agree.");

  var qs = ["{concat_graded_qs}"];
      
   qs.forEach(function (v, i) {{
     form.addScaleItem()
       .setTitle(v)  
       .setBounds(1, 10)
       .setLabels("strongly disagree","strongly agree")
       .setRequired(true);
   }});
                
  // section 2 Strenths and Improvements
  var text_qs = ["{concat_text_qs}"];
  
  form.addPageBreakItem()
       .setTitle("Strengths and Improvements")
       .setGoToPage(FormApp.PageNavigationType.CONTINUE)
       .setHelpText("You have 2 boxes to add a text or list about your Strengths and Improvements. The text in this section will be shared directly.");  
  
  text_qs.forEach(function (v, i) {{
      form.addParagraphTextItem()
      .setTitle(v)
      .setRequired(true);
    }});
  
   // move to the right folder
   var file = DriveApp.getFileById(form.getId());
   DriveApp.getFolderById(folderId).addFile(file);
   DriveApp.getRootFolder().removeFile(file); 
}}
"###,
    assessment_kind = self.assessment_kind,
    first_name = self.first_name,
    last_name = self.last_name,
    occasion = self.occasion,
    dir_id = self.dir_id,
    description = self.description,
    concat_graded_qs = concat_graded_qs,
    concat_text_qs = concat_text_qs,
  )
    }
}
