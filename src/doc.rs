use std::fs;

pub trait Doc {
  fn process(&mut self) {
    self.load_source_content();
    let output_html = self.process_content();
    self.set_output_html(output_html);
  }

  // Loads the file from path into source_content
  fn load_source_content(&mut self) {
    let content = fs::read_to_string(self.source_path()).expect("Can't load source content");
    self.set_source_content(content.trim().to_string());
  }

  fn source_path(&self) -> &str;
  fn set_source_content(&mut self, content: String);
  fn set_output_html(&mut self, content: String);
  fn process_content(&mut self) -> String;
  fn output_html(&self) -> &Option<String>;
}
