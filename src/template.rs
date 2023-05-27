use std::{io, path::Path};

use askama::Template;

#[derive(Template)]
#[template(path = "file_not_found.html")]
pub struct FileNotFound {
    uri: String,
}

#[derive(Template)]
#[template(path = "file_list.html")]
pub struct FileList {
    dir: String,
    items: Vec<String>,
}

impl FileNotFound {
    pub fn new(uri: String) -> Self {
        Self { uri: uri }
    }
}

impl TryFrom<String> for FileList {
    type Error = io::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut items = Path::new(&format!(".{}", value))
            .read_dir()?
            .filter_map(|f| f.ok())
            .map(|f| Some(String::from(f.file_name().to_str()?)))
            .filter_map(|x| x)
            .collect::<Vec<String>>();

        if value != "/" {
            items.insert(0, "..".to_string());
        }

        Ok(Self { dir: value, items })
    }
}
