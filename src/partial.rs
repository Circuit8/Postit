#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Partial {
  pub html: String,
  pub name: String,
}

impl Partial {
  pub fn new(name: &str, html: &str) -> Partial {
    Partial {
      name: name.to_string(),
      html: html.trim().to_string(),
    }
  }
}
