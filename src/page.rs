#![allow(dead_code)]
use super::partial::Partial;
use super::post::Post;
use handlebars::Handlebars;
use std::collections::HashMap;

struct Page {
  pub template: String,
  pub html: String,
  // pub path: String,
}

impl Page {
  fn new(template: &str, posts: &Vec<Post>, partials: &Vec<Partial>) -> Page {
    let mut partial_hashes = HashMap::new();
    for partial in partials {
      partial_hashes.insert(partial.name.clone(), partial.html.clone());
    }

    let json = &json!({ "posts": posts, "partials": partial_hashes });

    let reg = Handlebars::new();
    let injected_template = reg
      .render_template(template.trim(), json)
      .expect("Could not parse the handlebars template");

    Page {
      template: String::from(template),
      html: injected_template,
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
      let page = Page::new("badger", &vec![], &vec![]);

      assert_eq!(page.template, "badger");
    }

    #[test]
    fn should_produce_html_from_the_template() {
      let template = r#"
        <div>Hello world</div>
      "#;

      let page = Page::new(template, &vec![], &vec![]);
      assert_eq!(page.html, "<div>Hello world</div>");
    }

    #[test]
    fn it_should_interpolate_rust_in_the_html() {
      let post = Post {
        markdown: "".to_string(),
        html: "".to_string(),
        title: "Title".to_string(),
        category: "Badger".to_string(),
        path: "test/path.html".to_string(),
      };

      let template = r#"
        {{#each posts}}<div>{{ title }} - {{ category }}</div>{{/each}}
      "#;

      let page = Page::new(template, &vec![post], &vec![]);
      assert_eq!(page.html, "<div>Title - Badger</div>");
    }

    #[test]
    fn it_should_be_able_to_render_other_partial_templates() {
      let partial = Partial::new(
        "footer",
        "
        <footer>Goodbye cruel world</footer>
      ",
      );

      let template = "Hello {{{ partials.footer }}}";

      let page = Page::new(template, &vec![], &vec![partial]);
      assert_eq!(page.html, "Hello <footer>Goodbye cruel world</footer>");
    }
  }
}
