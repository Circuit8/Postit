#![allow(dead_code)]

pub mod commands {
  // Cleans the ./dist directory
  // Copies the contents of ./src over to dist, converting and interpolating along the way
  fn build() {
    // Takes a path like "./dist/posts"
    // Outputs self.output_html there with a filename based off self.source_Path
    // fs::create_dir_all(path).expect("Cant create path");
    // fs::write(path, self.output_html().as_ref().unwrap()).expect("Unable to write file");
  }

  // Creates a new Postit project in the given directory
  // Copies sensible project outline to it
  // fails if the directory already exists and is not empty
  fn new() {}
}

#[cfg(test)]
mod test {
  use super::*;

  mod build {
    use super::*;

    #[test]
    fn it_should_fail_without_a_src_directory() {
      unimplemented!();
    }

    #[test]
    fn it_should_create_the_dist_directory_if_it_doesnt_exist() {
      unimplemented!();
    }

    #[test]
    fn it_should_clean_the_dist_directory_if_it_does_exist() {
      unimplemented!();
    }

    #[test]
    fn it_should_copy_across_the_styles() {
      unimplemented!();
    }

    #[test]
    fn it_should_copy_across_the_js() {
      unimplemented!();
    }

    #[test]
    fn it_should_copy_across_the_converted_posts_with_layout_interpolation() {
      unimplemented!();
      // let mut post = Post::new("test/assets/posts/post.markdown");
      // post.process();

      // let path = format!("test/tmp/{}/badger.html", Uuid::new_v4());
      // post.output_to(&path);

      // let mut file = File::open(path).unwrap();
      // let mut output_content = String::new();
      // file.read_to_string(&mut output_content).unwrap();

      // assert_eq!(output_content, post.output_html.unwrap());
    }

    #[test]
    fn it_should_copy_across_the_pages_with_partial_and_layout_interpolation() {
      unimplemented!();
    }
  }

  mod new {
    use super::*;
    #[test]
    fn it_should_fail_without_a_project_name() {
      unimplemented!();
    }

    #[test]
    fn it_should_fail_if_the_project_directory_already_exists_with_contents() {
      unimplemented!();
    }
    #[test]
    fn it_should_create_the_project_directory_if_it_doesnt_exist() {
      unimplemented!();
    }

    #[test]
    fn it_should_use_an_empty_existing_project_directory() {
      unimplemented!();
    }

    #[test]
    fn it_should_copy_across_the_default_template() {
      unimplemented!();
    }
  }
}
