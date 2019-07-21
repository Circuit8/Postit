use std::fs::File;
use std::io::prelude::*;

pub trait Doc {
  // Loads the file from path into source_content
  fn load_source(&self) {
    let mut file = File::open(self.source_path).expect("Can't open path");
    let mut markdown = String::new();
    file
      .read_to_string(&mut markdown)
      .expect("Can't read from file");
    let markdown = markdown.trim().to_string();
  }

  fn process(&mut self) {
    // self.process_content()
    // self.
  }

  // Override me
  fn process_content(&mut self) {}

  // Called from process, sets self.html wrapped in the template 
  // Potench pass in the pre wrapped html rather than taking from self.html
  fn wrap_in_template(&mut self) {}

  // Takes a path like "./dist/posts"
  // Outputs self.html there with a filename based off self.path
  fn output_to(&self, path: &str) {}

  // Do we need this here? I think we do cant rely on properties of implemntors
  fn source_Path(&self) -> String {}
}

#[cfg(test)]
mod test {

  mod wrap_in_template {

    #[test]
    fn it_should_set_html_correctly() {
        unimplemented!();
    }
  }

  mod output_to {

    #[test]
    fn it_should_output_html_to_the_path() {
        unimplemented!();
    }
  }
}