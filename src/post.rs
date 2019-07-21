#![allow(dead_code)]
use comrak::{markdown_to_html, ComrakOptions};
use frontmatter::parse_and_find_content;
use serde::{Deserialize, Serialize};
use super::doc::Doc;

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
  pub source_content: Option<String>,
  pub output_html: Option<String>,
  pub title: String,
  pub category: String,
  pub source_path: String,
}

impl Doc for Post {}

impl Post {
  pub fn new(source_path: &str) -> Post {
    Post {
      source_content: None,
      output_html: None,
      category: "Uncategorized".to_string(),
      title: "Untitled".to_string(),
      source_path: String::from(source_path),
    }
  }

  pub fn process_content(&mut self) -> String {
    let source_content = self.source_content.unwrap();

    let mut title = String::from("Untitled");
    let mut category = String::from("Uncategorised");

    match parse_and_find_content(&source_content) {
      Ok((matter, markdown)) => {
        if let Some(yaml) = matter {
          self.title = yaml["title"].as_str().unwrap().to_string();
          self.category = yaml["category"].as_str().unwrap().to_string();
        }
        return markdown_to_html(&source_content, &ComrakOptions::default());
      }

      Err(e) => {
        println!("{}", e);
        return markdown_to_html(&source_content, &ComrakOptions::default());
      }
    }
  }
}

#[cfg(test)]
mod tests {
  mod new {
    use super::super::*;

    fn sets_the_defaults() {
      let path = "./test/assets/posts/simple.markdown";
      let post = Post::new(path);

      assert_eq!(post.category, "Uncategorised");
      assert_eq!(post.title, "Untitled");
      assert_eq!(post.source_path, path);
    }
  }

  mod process_content {
    use super::super::*;
    use std::fs::File;
    use std::io::prelude::*;

    fn load_file(path: &str) -> String {
      let mut file = File::open(path).unwrap();
      let mut contents = String::new();
      file
        .read_to_string(&mut contents)
        .expect("Can't read from file");

      contents.trim().to_string()
    }

    #[test]
    fn converts_the_html_correctly_with_a_simple_input() {
      let path = "test/assets/posts/post.markdown";
      let post = Post {
        source_content: Some(load_file(path)),
        output_html: None,
        category: "Uncategorized".to_string(),
        title: "Untitled".to_string(),
        source_path: String::from(path),
      };

      assert_eq!(
        post.process_content(),
        "<ul>\n<li>This is</li>\n<li>A list</li>\n</ul>\n"
      )
    }

    #[test]
    fn saves_the_config_title_and_category_correctly() {
      let path = "test/assets/posts/post.markdown";
      let post = Post {
        source_content: Some(load_file(path)),
        output_html: None,
        category: "Uncategorized".to_string(),
        title: "Untitled".to_string(),
        source_path: String::from(path),
      };

      post.process_content();

      assert_eq!(post.title, "badger");
      assert_eq!(post.category, "bodger");
    }
  }

  mod from_file {}
}
