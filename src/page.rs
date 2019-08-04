#![allow(dead_code)]
use super::doc::Doc;
use super::partial::Partial;
use super::post::Post;
use handlebars::Handlebars;
use std::collections::HashMap;

pub struct Page<'a> {
  pub source_content: Option<String>,
  pub output_html: Option<String>,
  pub source_path: String,
  pub layout_path: String,
  pub posts: &'a Vec<Post>,
  pub partials: &'a Vec<Partial>,
}

impl Doc for Page<'_> {
  fn process_content(&mut self) -> String {
    let mut partial_hashes = HashMap::new();
    for partial in self.partials {
      partial_hashes.insert(partial.name.clone(), partial.html.clone());
    }

    let json = &json!({ "posts": self.posts, "partials": partial_hashes });

    Handlebars::new()
      .render_template(self.source_content.as_ref().unwrap(), json)
      .expect("Could not parse the handlebars template")
  }

  fn set_source_content(&mut self, content: String) {
    self.source_content = Some(content);
  }

  fn layout_path(&self) -> &str {
    &self.layout_path
  }

  fn source_path(&self) -> &str {
    &self.source_path
  }

  fn set_output_html(&mut self, html: String) {
    self.output_html = Some(html);
  }

  fn output_html(&self) -> &Option<String> {
    &self.output_html
  }
}

impl<'a> Page<'a> {
  pub fn new(
    source_path: &str,
    layout_path: &str,
    posts: &'a Vec<Post>,
    partials: &'a Vec<Partial>,
  ) -> Page<'a> {
    Page {
      output_html: None,
      source_content: None,
      source_path: String::from(source_path),
      layout_path: String::from(layout_path),
      posts,
      partials,
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
      let posts = vec![];
      let partials = vec![];
      let mut page = Page::new(
        "test/assets/pages/simple.handlebars",
        "./test/assets/example_src/layout.handlebars",
        &posts,
        &partials,
      );
      page.process();
      assert_eq!(page.source_content.unwrap(), "badger");
    }

    #[test]
    fn should_produce_html_from_the_template() {
      let posts = vec![];
      let partials = vec![];
      let mut page = Page::new(
        "test/assets/pages/hello_world.handlebars",
        "./test/assets/example_src/layout.handlebars",
        &posts,
        &partials,
      );
      page.process();
      assert_eq!(
        page.output_html.unwrap(),
        "<article><div>Hello world</div></article>"
      );
    }

    #[test]
    fn it_should_interpolate_posts_in_the_html() {
      let post = Post {
        source_content: None,
        output_html: None,
        title: "Title".to_string(),
        category: "Badger".to_string(),
        source_path: "test/path.html".to_string(),
        layout_path: "./test/assets/example_src/layout.handlebars".to_string(),
      };

      let posts = vec![post];
      let partials = vec![];

      let mut page = Page::new(
        "test/assets/pages/posts.handlebars",
        "./test/assets/example_src/layout.handlebars",
        &posts,
        &partials,
      );
      page.process();
      assert_eq!(
        page.output_html.unwrap(),
        "<article><div>Title - Badger</div></article>"
      );
    }

    #[test]
    fn it_should_be_able_to_render_other_partial_templates() {
      let partial = Partial::new(
        "footer",
        "
        <footer>Goodbye cruel world</footer>
      ",
      );

      let posts = vec![];
      let partials = vec![partial];

      let mut page = Page::new(
        "test/assets/pages/partial.handlebars",
        "./test/assets/example_src/layout.handlebars",
        &posts,
        &partials,
      );
      page.process();

      assert_eq!(
        page.output_html.unwrap(),
        "<article>Hello <footer>Goodbye cruel world</footer></article>"
      );
    }
  }
}
