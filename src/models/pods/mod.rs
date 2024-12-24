use serde::{Deserialize, Serialize};
use std::{error::Error, fmt::Display, fs, path::PathBuf};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PodsCollectionDirectoryItem {
    path: PathBuf,
    is_file: bool,
}

#[derive(Deserialize, Serialize)]
pub struct ResourceRequestQueryInfo {
    pub iri: String,
}

pub struct PodsCollection {
    dir: String,
    contents: Option<Vec<PodsCollectionDirectoryItem>>,
    root: bool,
}

impl PodsCollection {
    pub fn load_dir_contents(&mut self) -> Result<(), Box<dyn Error>> {
        let dir_contents = fs::read_dir(&self.dir)?;
        let mut mapped_contents = vec![];

        for item in dir_contents {
            let dir_ent = item?;
            let dir_ent_info = dir_ent.metadata()?;
            mapped_contents.push(PodsCollectionDirectoryItem {
                path: dir_ent.path(),
                is_file: dir_ent_info.is_file(),
            });
        }

        self.contents = Some(mapped_contents);
        Ok(())
    }
}

pub struct PodsCollectionBuilder;

impl PodsCollectionBuilder {
    pub fn new(path: String) -> PodsCollection {
        PodsCollection {
            dir: path,
            contents: None,
            root: false, // TODO check this
        }
    }
}

const PODS_COLLECTION_LOAD_DIR_CONTENTS_ERROR_CODE: u16 = 509;
const PODS_COLLECTION_LOAD_DIR_CONTENTS_ERROR_MESSAGE: &'static str =
    "[PodsCollection] Load Pods Directory Contents Error";

#[derive(Debug, Clone, Copy)]
pub struct PodsCollectionLoadDirContentsError {}

impl Display for PodsCollectionLoadDirContentsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Error (code: {}): {}",
            PODS_COLLECTION_LOAD_DIR_CONTENTS_ERROR_CODE,
            PODS_COLLECTION_LOAD_DIR_CONTENTS_ERROR_MESSAGE
        )
    }
}

impl Error for PodsCollectionLoadDirContentsError {}
