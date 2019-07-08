#![allow(dead_code)]
use comrak::{markdown_to_html, ComrakOptions};
use frontmatter::parse_and_find_content;

struct Post {
  markdown: String,
  title: String,
  category: String,
  html: String,
}

impl Post {
  fn new(markdown: &str) -> Post {
    // parse the markdown into html
    // parse the yaml config at the top into the title and category
    match parse_and_find_content(markdown) {
      Some(result) => {
        println(result);
      },

      None(result) => {
        println(result);
      }
    }

    let html = markdown_to_html(markdown, &ComrakOptions::default());
    let title = String::from("Untitled");
    let category = String::from("Uncategorised");

    Post {
      markdown: String::from(markdown),
      html,
      category,
      title,
    }
  }

  // fn from_file() -> Post {

  // }
}

#[cfg(test)]
mod tests {
  mod new {
    use super::super::*;

    const MARKDOWN: &str = r#"
      ---
      title = badger
      category = bodger
      ---

      - This is
      - A list
    "#;

    #[test]
    fn sets_the_defaults() {
      let post = Post::new("Hello `world`!");
      assert_eq!(post.markdown, "Hello `world`!");
      assert_eq!(post.category, "Uncategorised");
      assert_eq!(post.title, "Untitled")
    }

    #[test]
    fn converts_the_html_correctly_with_a_simple_input() {
      let post = Post::new(MARKDOWN);

      assert_eq!(
        post.html,
        "<ul>\n<li>This is</li>\n<li>A list</li>\n</ul>\n"
      )
    }

    #[test]
    fn saves_the_config_title_and_category_correctly() {
      let post = Post::new(MARKDOWN);

      assert_eq!(post.title, "badger");
      assert_eq!(post.category, "bodger");
    }

  }

  mod from_file {}
}
