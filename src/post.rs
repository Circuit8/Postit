#![allow(dead_code)]
use comrak::{markdown_to_html, ComrakOptions};
use frontmatter::parse_and_find_content;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
  pub markdown: String,
  pub title: String,
  pub category: String,
  pub output_html: String,
  pub path: String,
}

impl Post {
  pub fn new(path: &str) -> Post {
    let mut title = String::from("Untitled");
    let mut category = String::from("Uncategorised");
    let mut output_html;

    let mut file = File::open(path).expect("Can't open path");
    let mut markdown = String::new();
    file
      .read_to_string(&mut markdown)
      .expect("Can't read from file");
    let markdown = markdown.trim().to_string();

    match parse_and_find_content(&markdown) {
      Ok((matter, markdown)) => {
        if let Some(yaml) = matter {
          title = yaml["title"].as_str().unwrap().to_string();
          category = yaml["category"].as_str().unwrap().to_string();
        }
        output_html = markdown_to_html(markdown, &ComrakOptions::default());
      }

      Err(e) => {
        println!("{}", e);
        output_html = markdown_to_html(&markdown, &ComrakOptions::default());
      }
    }

    Post {
      markdown,
      output_html,
      category,
      title,
      path: String::from(path),
    }
  }
}

#[cfg(test)]
mod tests {
  mod new {
    use super::super::*;

    #[test]
    fn sets_the_defaults() {
      let post = Post::new("./test/assets/posts/simple.markdown");
      assert_eq!(post.markdown, "Hello `world`!");
      assert_eq!(post.category, "Uncategorised");
      assert_eq!(post.title, "Untitled");
    }

    #[test]
    fn converts_the_html_correctly_with_a_simple_input() {
      let post = Post::new("./test/assets/posts/post.markdown");

      assert_eq!(
        post.output_html,
        "<ul>\n<li>This is</li>\n<li>A list</li>\n</ul>\n"
      )
    }

    #[test]
    fn saves_the_config_title_and_category_correctly() {
      let post = Post::new("./test/assets/posts/post.markdown");

      assert_eq!(post.title, "badger");
      assert_eq!(post.category, "bodger");
    }
  }

  mod from_file {}
}
