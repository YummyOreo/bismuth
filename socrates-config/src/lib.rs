#![allow(dead_code)]
use std::path::Path;

pub struct Config<'a> {
    directory: &'a Path,
}

impl<'a> Config<'a> {
    pub fn new(dir: &'a Path) -> Self {
        Config { directory: dir }
    }
}

pub struct Markdown {
}
