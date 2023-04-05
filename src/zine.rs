use serde::{Deserialize, Serialize};
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub struct Magazine {
    pub hello_message: String,
    pub sections: Vec<Section>,
    #[serde(skip)]
    pub hello: String
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
                let pages = Section::pages_for_directory(&page_directory);
                Section {
                    title: section.title,
                    author: section.author,
                    directory: section.directory,
                    pages,
                }
            })
            .collect();
        
        magazine.hello = fs::read_to_string(directory.join(&magazine.hello_message))
                            .unwrap_or_else(|_| panic!("Failed to read hello file {:?}", magazine.hello_message));
        magazine
    }

    pub fn all_sections(&self) -> Vec<&Section> {
        self.sections.iter().collect()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Section {
    pub title: String,
    pub author: String,
    pub directory: String,
    #[serde(skip)]
    pub pages: Vec<Page>,
}

impl Section {
    pub fn all_pages(&self) -> Vec<&Page> {
        self.pages.iter().collect()
    }

    fn pages_for_directory(directory: &Path) -> Vec<Page> {
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
                    Some(Page::from_file(page_number, &entry.path()))
                } else {
                    None
                }
            })
            .collect();
        pages.sort_by_key(|page| page.page_number);
        pages
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Page {
    pub page_number: u32,
    pub sections: String,
}

impl Page {
    fn from_file(page_number: u32, filename: &Path) -> Self {
        let sections = fs::read_to_string(filename)
            .unwrap_or_else(|_| panic!("Failed to read Page file {:?}", filename));
        Self {
            page_number,
            sections,
        }
    }
}