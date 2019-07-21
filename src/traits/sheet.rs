trait Sheet {
  // Called from process, sets self.html wrapped in the template 
  // Potench pass in the pre wrapped html rather than taking from self.html
  fn wrap_in_template(&mut self) {}

  // Takes a path like "./dist/posts"
  // Outputs self.html there with a filename based off self.path
  fn output_to(&self, path: &str) {}
}

#[cfg(test)]
mod test {
  use super::*;

  mod wrap_in_template {
    use super::*;

    #[test]
    fn it_should_set_html_correctly() {
        unimplemented!();
    }
  }

  mod output_to {
    use super::*;

    #[test]
    fn it_should_output_html_to_the_path() {
        unimplemented!();
    }
  }
}