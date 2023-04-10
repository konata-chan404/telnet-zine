use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::{self};
use std::path::{Path};
use std::collections::HashMap;
use handlebars::{Handlebars};
use std::fmt;
use chrono::prelude::*;
use crate::helpers::{color_helper, italic_helper, bold_helper, underline_helper, rainbow_helper};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Magazine {
    pub title: String,
    pub cover: String,
    pub front: String,
    pub sections: Vec<Section>,
    #[serde(default)]
    pub vars: HashMap<String, String>,
    #[serde(skip)]
    pub cover_text: String,
    #[serde(skip)]
    pub front_text: String
}

impl Magazine {
    /// Creates a new magazine from a directory path
    pub fn from_directory(directory: &Path) -> Self {
        // Read the index file to get the magazine's metadata
        let index_file = directory.join("index.json");
        let index_str = fs::read_to_string(&index_file)
            .unwrap_or_else(|_| panic!("Failed to read index file {:?}", index_file));

        let mut magazine: Magazine =
            serde_json::from_str(&index_str).expect("Failed to parse index file");

        // Read and parse the pages for each section in the magazine
        magazine.sections = magazine
            .sections
            .into_iter()
            .map(|section| {
                let page_directory = directory.join(&section.directory);
                let pages = section.pages_for_directory(&page_directory);
                Section {
                    title: section.title,
                    author: section.author,
                    directory: section.directory,
                    vars: section.vars,
                    pages,
                }
            })
            .collect();

        // Read and render the cover and front pages of the magazine
        magazine.cover_text = read_and_render_page(&directory, &magazine.cover);
        magazine.front_text = read_and_render_page(&directory, &magazine.front);

        // Render the front page with the magazine metadata and sections
        let mut engine = Handlebars::new();
        engine.register_template_string("front_text", magazine.front_text.as_str())
                            .unwrap_or_else(|_| panic!("Failed to register front page"));
        engine.register_template_string("cover_text", magazine.cover_text.as_str())
                            .unwrap_or_else(|_| panic!("Failed to register cover page"));
        let context = json!({
            "title": magazine.title.as_str(),
            "sections": magazine.sections,
            "vars": magazine.vars
        });
        engine.register_helper("color", Box::new(color_helper));
        engine.register_helper("italic", Box::new(italic_helper));
        engine.register_helper("bold", Box::new(bold_helper));
        engine.register_helper("underline", Box::new(underline_helper));
        engine.register_helper("rainbow", Box::new(rainbow_helper));

        magazine.front_text = engine.render("front_text", &context)
                            .unwrap_or_else(|_| panic!("Failed to render front page"));
        magazine.cover_text = engine.render("cover_text", &context)
                            .unwrap_or_else(|_| panic!("Failed to render cover page"));   
        magazine
    }

    /// Returns a vector containing references to all the sections in the magazine
    pub fn all_sections(&self) -> Vec<&Section> {
        self.sections.iter().collect()
    }

    pub fn get_section(&self, index: usize) -> Option<&Section> {
        self.sections.get(index)
    }
}


/// Reads and renders a page file located in the specified directory
fn read_and_render_page(directory: &Path, page_filename: &str) -> String {
    let page_path = directory.join(page_filename);
    fs::read_to_string(&page_path)
        .unwrap_or_else(|_| panic!("Failed to read page file {:?}", page_path))
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Section {
    pub title: String,
    pub author: String,
    pub directory: String,
    #[serde(default)]
    pub vars: HashMap<String, String>,
    #[serde(skip)]
    pub pages: Vec<Page>,
}

impl Section {
    // Get all pages in the section
    pub fn all_pages(&self) -> Vec<&Page> {
        self.pages.iter().collect()
    }

    pub fn get_page(&self, index: usize) -> Option<&Page> {
        self.pages.get(index)
    }

    // Get all pages for a given directory
    fn pages_for_directory(&self, directory: &Path) -> Vec<Page> {
        let mut pages: Vec<Page> = fs::read_dir(directory)
            .unwrap_or_else(|err| panic!("Failed to read directory {:?}: {}", directory, err))
            .filter_map(|entry| {
                let entry = entry.expect("Failed to read directory entry");
                if entry.path().is_file() {
                    let page_number = entry
                        .file_name()
                        .to_str()
                        .and_then(|s| s.trim_end_matches(".txt").parse().ok())
                        .expect("Failed to parse page number");
                    let page = Some(Page::from_file(page_number, &entry.path()));
                    if page.is_some() {
                            let mut page_unwrapped = page.unwrap();
                            let mut engine = Handlebars::new();
                            engine.register_template_string("page_content", page_unwrapped.text.as_str())
                                                .unwrap_or_else(|err| panic!("Failed to register page: {}", err));
                            engine.register_helper("color", Box::new(color_helper));
                            engine.register_helper("italic", Box::new(italic_helper));
                            engine.register_helper("bold", Box::new(bold_helper));
                            engine.register_helper("underline", Box::new(underline_helper));
                            engine.register_helper("rainbow", Box::new(rainbow_helper));
                            let context = json!({
                                "title": self.title.as_str(),
                                "author": self.author.as_str(),
                                "vars": self.vars
                            });
                            println!("[{}] Rendering page {} with context: {:?}", Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), 
                                                                            page_unwrapped.page_number, context);
                    
                            page_unwrapped.text = engine.render("page_content", &context)
                                                .unwrap_or_else(|err| panic!("Failed to render page: {}", err));
                            return Some(page_unwrapped);
                        }
                    return page;
                } else {
                    None
                }
            })
            .collect();
        pages.sort_by_key(|page| page.page_number);
        pages
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Page {
    pub page_number: u32,
    pub text: String,
}

impl Page {
    // Create a new page from a file
    fn from_file(page_number: u32, filename: &Path) -> Self {
        let sections = fs::read_to_string(filename)
            .unwrap_or_else(|err| panic!("Failed to read Page file {:?}: {}", filename, err));
        Self {
            page_number,
            text: sections,
        }
    }
}

impl fmt::Display for Page {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}