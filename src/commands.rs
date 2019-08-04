#![allow(dead_code)]
extern crate uuid;
use super::doc::Doc;
use super::page::Page;
use super::post::Post;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

// Cleans the ./dist directory
// Copies the contents of ./src over to dist, converting and interpolating along the way
pub fn build(source_dir: &str, dist_dir: &str) {
  reset_dist_dir(dist_dir);
  copy_dir(source_dir, dist_dir, "style");
  copy_dir(source_dir, dist_dir, "js");

  let layout_path = format!("{}/{}", source_dir, "layout.handlebars");
  let posts = copy_posts(source_dir, dist_dir, &layout_path);
  copy_pages(source_dir, dist_dir, &layout_path, posts);
}

fn reset_dist_dir(dist_dir: &str) {
  if Path::new(&dist_dir).exists() {
    fs::remove_dir_all(dist_dir).expect("Cant clean dist dir");
  }
  fs::create_dir_all(dist_dir).expect("Cant create dist dir");
}

fn copy_dir(source_dir: &str, dist_dir: &str, sub_dir: &str) {
  fs::create_dir_all(format!("{}/{}", dist_dir, sub_dir)).expect("Cant create output dir");

  for entry in WalkDir::new(format!("{}/{}", source_dir, sub_dir)) {
    let file = entry.unwrap();

    if file.file_type().is_dir() {
      fs::create_dir_all(file.path()).expect("Cant create sub dir");
    } else {
      let output_path = format!(
        "{}/{}/{}",
        dist_dir,
        sub_dir,
        file.file_name().to_str().unwrap()
      );
      fs::copy(file.path(), output_path).expect("Cant copy file");
    }
  }
}

fn copy_posts(source_dir: &str, dist_dir: &str, layout_path: &str) -> Vec<Post> {
  fs::create_dir_all(format!("{}/{}", dist_dir, "posts")).expect("Cant create posts dir");
  let mut posts: Vec<Post> = vec![];

  for entry in WalkDir::new(format!("{}/{}", source_dir, "posts")) {
    let file = entry.unwrap();
    if file.file_type().is_dir() {
      continue;
    }

    let mut post = Post::new(file.path().to_str().unwrap(), layout_path);
    post.process();

    let file_name = file
      .file_name()
      .to_str()
      .unwrap()
      .split('.')
      .collect::<Vec<&str>>()[0];

    let output_path = format!("{}/posts/{}.html", dist_dir, file_name);
    fs::write(output_path, post.output_html.as_ref().unwrap()).expect("Cant write post");
    posts.push(post);
  }

  posts
}

fn copy_pages(source_dir: &str, dist_dir: &str, layout_path: &str, posts: Vec<Post>) {
  fs::create_dir_all(format!("{}/{}", dist_dir, "pages")).expect("Cant create pages dir");

  for entry in WalkDir::new(format!("{}/{}", source_dir, "pages")) {
    let file = entry.unwrap();
    if file.file_type().is_dir() {
      continue;
    }

    let partials = vec![];
    let mut page = Page::new(
      file.path().to_str().unwrap(),
      &layout_path,
      &posts,
      &partials,
    );
    page.process();

    let file_name = file
      .file_name()
      .to_str()
      .unwrap()
      .split('.')
      .collect::<Vec<&str>>()[0];

    let output_path = format!("{}/pages/{}.html", dist_dir, file_name);
    fs::write(output_path, page.output_html.as_ref().unwrap()).expect("Cant write page");
  }
}

// Creates a new Postit project in the given directory
// Copies sensible project outline to it
// fails if the directory already exists and is not empty
pub fn new() {}

#[cfg(test)]
mod test {
  use super::*;
  use uuid::Uuid;

  mod build {
    use super::*;

    fn test_dist_dir() -> String {
      format!("./test/tmp/{}", Uuid::new_v4())
    }

    #[test]
    fn it_should_create_the_dist_directory_if_it_doesnt_exist() {
      let dist_dir = test_dist_dir();
      build("./test/assets/example_src", &dist_dir);

      assert_eq!(Path::new(&dist_dir).exists(), true);
    }

    #[test]
    fn it_should_clean_the_dist_directory_if_it_does_exist() {
      let dist_dir = test_dist_dir();
      let file = format!("{}/badger.txt", dist_dir);

      fs::create_dir_all(&dist_dir).expect("Cant create dist dir");
      fs::write(&file, "test").expect("Could not write test file");

      build("./test/assets/example_src", &dist_dir);

      assert_eq!(Path::new(&file).exists(), false);
    }

    #[test]
    fn it_should_copy_across_the_styles() {
      let dist_dir = test_dist_dir();
      build("./test/assets/example_src", &dist_dir);

      assert_eq!(
        Path::new(&format!("{}/{}", dist_dir, "style/index.css")).exists(),
        true
      );
    }

    #[test]
    fn it_should_copy_across_the_js() {
      let dist_dir = test_dist_dir();
      build("./test/assets/example_src", &dist_dir);

      assert_eq!(
        Path::new(&format!("{}/{}", dist_dir, "js/library.js")).exists(),
        true
      );
    }

    #[test]
    fn it_should_copy_across_the_converted_posts() {
      let dist_dir = test_dist_dir();
      build("./test/assets/example_src", &dist_dir);

      assert_eq!(
        Path::new(&format!("{}/{}", dist_dir, "posts/test.html")).exists(),
        true
      );
    }

    #[test]
    fn it_should_copy_across_the_pages() {
      let dist_dir = test_dist_dir();
      build("./test/assets/example_src", &dist_dir);

      assert_eq!(
        Path::new(&format!("{}/{}", dist_dir, "pages/home.html")).exists(),
        true
      );
    }
  }

  mod new {
    use super::*;
    #[test]
    fn it_should_fail_without_a_project_name() {}

    #[test]
    fn it_should_fail_if_the_project_directory_already_exists_with_contents() {}
    #[test]
    fn it_should_create_the_project_directory_if_it_doesnt_exist() {}

    #[test]
    fn it_should_use_an_empty_existing_project_directory() {}

    #[test]
    fn it_should_copy_across_the_default_template() {}
  }
}
