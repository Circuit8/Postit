#![allow(dead_code)]
use super::doc::Doc;
use comrak::{markdown_to_html, ComrakOptions};
use frontmatter::parse_and_find_content;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
  pub source_content: Option<String>,
  pub output_html: Option<String>,
  pub title: String,
  pub category: String,
  pub source_path: String,
  pub layout_path: String,
  pub file_name: String,
}

impl Doc for Post {
  fn process_content(&mut self) -> String {
    let source_content = self.source_content.as_ref().unwrap();

    match parse_and_find_content(&source_content) {
      Ok((matter, markdown)) => {
        if let Some(yaml) = matter {

          let title = yaml["title"].as_str();
          if let Some(title) = title {
            self.title = title.to_string()
          }

          let category = yaml["category"].as_str();
          if let Some(category) = category {
            self.category = category.to_string()
          }
        }
        return markdown_to_html(&markdown, &ComrakOptions::default());
      }

      Err(e) => {
        println!("{}", e);
        return markdown_to_html(&source_content, &ComrakOptions::default());
      }
    }
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

impl Post {
  pub fn new(source_path: &str, layout_path: &str) -> Post {
    Post {
      file_name: source_path
        .split('/')
        .last()
        .unwrap()
        .split('.')
        .nth(0)
        .unwrap()
        .to_string(),
      source_content: None,
      output_html: None,
      category: "Uncategorized".to_string(),
      title: "Untitled".to_string(),
      source_path: String::from(source_path),
      layout_path: String::from(layout_path),
    }
  }
}

#[cfg(test)]
mod tests {
  mod new {
    use super::super::*;

    fn sets_the_defaults() {
      let path = "./test/assets/posts/simple.markdown";
      let layout_path = "./test/assets/example_src/layout.handlebars";
      let post = Post::new(path, layout_path);

      assert_eq!(post.category, "Uncategorised");
      assert_eq!(post.title, "Untitled");
      assert_eq!(post.source_path, path);
      assert_eq!(post.layout_path, layout_path);
    }
  }

  mod process_content {
    use super::super::*;

    #[test]
    fn converts_the_html_correctly_with_a_simple_input() {
      let mut post = Post::new(
        "test/assets/posts/post.markdown",
        "./test/assets/example_src/layout.handlebars",
      );
      post.process();

      assert_eq!(
        post.output_html.unwrap(),
        "<article><ul>\n<li>This is</li>\n<li>A list</li>\n</ul>\n</article>"
      )
    }

    #[test]
    fn saves_the_config_title_and_category_correctly() {
      let mut post = Post::new(
        "test/assets/posts/post.markdown",
        "./test/assets/example_src/layout.handlebars",
      );
      post.process();

      assert_eq!(post.title, "badger");
      assert_eq!(post.category, "bodger");
    }
  }
}
