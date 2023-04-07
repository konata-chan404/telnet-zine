use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::{self};
use std::path::{Path};
use std::task::Context;
use std::collections::HashMap;
use handlebars::{Handlebars, TemplateError};

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
    pub fn from_directory(directory: &Path) -> Self {
        let index_file = directory.join("index.json");
        let index_str = fs::read_to_string(&index_file)
            .unwrap_or_else(|_| panic!("Failed to read index file {:?}", index_file));

        let mut magazine: Magazine =
            serde_json::from_str(&index_str).expect("Failed to parse index file");

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
        
        magazine.cover_text = fs::read_to_string(directory.join(&magazine.cover))
                            .unwrap_or_else(|_| panic!("Failed to read cover file {:?}", magazine.cover));
        magazine.front_text = fs::read_to_string(directory.join(&magazine.front))
                            .unwrap_or_else(|_| panic!("Failed to read front file {:?}", magazine.front));
        
        let mut engine = Handlebars::new();
        engine.register_template_string("front_text", magazine.front_text.as_str())
                            .unwrap_or_else(|_| panic!("Failed to register front page"));
        let mut context = json!({
            "title": magazine.title.as_str(),
            "sections": magazine.sections,
            "vars": magazine.vars
        });
        println!("{:?}", context);

        magazine.front_text = engine.render("front_text", &context)
                            .unwrap_or_else(|_| panic!("Failed to register front page"));   
        magazine
    }

    pub fn all_sections(&self) -> Vec<&Section> {
        self.sections.iter().collect()
    }
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
    pub fn all_pages(&self) -> Vec<&Page> {
        self.pages.iter().collect()
    }

    fn pages_for_directory(&self, directory: &Path) -> Vec<Page> {
        let mut pages: Vec<Page> = fs::read_dir(directory)
            .unwrap_or_else(|_| panic!("Failed to read directory {:?}", directory))
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
                                                .unwrap_or_else(|_| panic!("Failed to register page"));
                            let mut context = json!({
                                "title": self.title.as_str(),
                                "author": self.author.as_str(),
                                "vars": self.vars
                            });
                            println!("{:?}", context);
                    
                            page_unwrapped.text = engine.render("page_content", &context)
                                                .unwrap_or_else(|_| panic!("Failed to register front page"));
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
    fn from_file(page_number: u32, filename: &Path) -> Self {
        let sections = fs::read_to_string(filename)
            .unwrap_or_else(|_| panic!("Failed to read Page file {:?}", filename));
        Self {
            page_number,
            text: sections,
        }
    }
}