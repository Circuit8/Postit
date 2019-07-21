#![allow(dead_code)]
use super::partial::Partial;
use super::post::Post;
use handlebars::Handlebars;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

struct Page {
  pub input_html: String,
  pub output_html: String,
  pub path: String,
}

impl Page {
  fn new(path: &str, posts: &Vec<Post>, partials: &Vec<Partial>) -> Page {
    let mut file = File::open(path).expect("Can't open path");
    let mut input_html = String::new();
    file
      .read_to_string(&mut input_html)
      .expect("Can't read from file");
    let input_html = input_html.trim().to_string();

    let mut partial_hashes = HashMap::new();
    for partial in partials {
      partial_hashes.insert(partial.name.clone(), partial.html.clone());
    }

    let json = &json!({ "posts": posts, "partials": partial_hashes });

    let reg = Handlebars::new();
    let injected_html = reg
      .render_template(&input_html, json)
      .expect("Could not parse the handlebars template");

    Page {
      input_html,
      output_html: String::from(injected_html),
      path: String::from(path),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod new {
    use super::*;

    #[test]
    fn should_create_a_page() {
      let page = Page::new("test/assets/pages/simple.handlebars", &vec![], &vec![]);

      assert_eq!(page.input_html, "badger");
    }

    #[test]
    fn should_produce_html_from_the_template() {
      let page = Page::new("test/assets/pages/hello_world.handlebars", &vec![], &vec![]);
      assert_eq!(page.output_html, "<div>Hello world</div>");
    }

    #[test]
    fn it_should_interpolate_rust_in_the_html() {
      let post = Post {
        markdown: "".to_string(),
        output_html: "".to_string(),
        title: "Title".to_string(),
        category: "Badger".to_string(),
        path: "test/path.html".to_string(),
      };

      let page = Page::new("test/assets/pages/posts.handlebars", &vec![post], &vec![]);
      assert_eq!(page.output_html, "<div>Title - Badger</div>");
    }

    #[test]
    fn it_should_be_able_to_render_other_partial_templates() {
      let partial = Partial::new(
        "footer",
        "
        <footer>Goodbye cruel world</footer>
      ",
      );

      let page = Page::new(
        "test/assets/pages/partial.handlebars",
        &vec![],
        &vec![partial],
      );
      assert_eq!(
        page.output_html,
        "Hello <footer>Goodbye cruel world</footer>"
      );
    }
  }
}
