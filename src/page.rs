#![allow(dead_code)]
// use crate::Post;

struct Page {
  template: String,
  html: String,
}

impl Page {
  fn new(template: &str) -> Page {
    Page {
      template: String::from(template),
      html: String::from(template.trim()),
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
      let page = Page::new("badger");

      assert_eq!(page.template, "badger")
    }

    #[test]
    fn should_produce_html_from_the_template() {
      let template = r#"
        <div>Hello world</div>
      "#;

      let page = Page::new(template);
      assert_eq!(page.html, "<div>Hello world</div>")
    }

    #[test]
    fn it_should_interpolate_rust_in_the_html() {}

    #[test]
    fn it_should_be_able_to_use_the_posts_in_the_template() {}

    #[test]
    fn it_should_be_able_to_render_other_partial_templates() {}
  }
}
