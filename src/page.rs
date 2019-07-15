#![allow(dead_code)]
extern crate handlebars;
extern crate serde_json;

use super::post::Post;
use handlebars::Handlebars;

struct Page {
  template: String,
  html: String,
}

impl Page {
  fn new(template: &str, posts: Vec<Post>) -> Page {
    let post_json = posts.into_iter().map(|post| 
      serde_json::to_string(&post).expect("")
    ).collect()

    let reg = Handlebars::new();
    let injected_template = reg
      .render_template(template.trim(), &json!({ "posts": post_json }))
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
      let page = Page::new("badger", vec![]);

      assert_eq!(page.template, "badger")
    }

    #[test]
    fn should_produce_html_from_the_template() {
      let template = r#"
        <div>Hello world</div>
      "#;

      let page = Page::new(template, vec![]);
      assert_eq!(page.html, "<div>Hello world</div>")
    }

    #[test]
    fn it_should_interpolate_rust_in_the_html() {
      let post = Post {
        markdown: "".to_string(),
        html: "".to_string(),
        title: "Title",
        category: "Badger",
      };

      let template = r#"
        {{#each test}}<div>{{ title }} - {{ category }}</div>{{/each}}
      "#;

      let page = Page::new(template, vec![post]);
      assert_eq!(page.html, "<div>Title - Badger</div>");
    }

    #[test]
    fn it_should_be_able_to_use_the_posts_in_the_template() {}

    #[test]
    fn it_should_be_able_to_render_other_partial_templates() {}
  }
}
