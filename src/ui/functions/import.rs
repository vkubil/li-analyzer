use futures::future;
use log::info;
use regex::{Captures, Regex};
use rfd::{AsyncFileDialog, FileHandle};

const EXPRESSION: &str = "^(?:(?:https:\\/\\/linkedin.com\\/in)?[/])?([A-Za-z0-9\\-]+)(?:[/].*)?$";

pub async fn import_graduate_files() -> anyhow::Result<Vec<String>, anyhow::Error> {
    let files = AsyncFileDialog::new()
        .add_filter("text", &["txt"])
        .add_filter("csv", &["csv"])
        .set_directory("/")
        .pick_files()
        .await;

    if files.is_none() {
        return Ok(vec![]);
    }

    let files = files.unwrap();

    import_graduates(files).await
}

pub async fn import_graduates(files: Vec<FileHandle>) -> anyhow::Result<Vec<String>> {
    let links_from_files = future::try_join_all(files.iter().map(read_links)).await?;
    let links = links_from_files.iter().flatten().cloned();

    info!("Gauti URIs is failu: {:?}", links);

    let regex = Regex::new(EXPRESSION)?;
    let results = links
        .filter_map(|line| get_id(regex.captures(&line)))
        .collect::<Vec<String>>();

    Ok(results)
}

fn get_id(capture: Option<Captures<'_>>) -> Option<String> {
    capture.map(|captures| captures.get(1).unwrap().as_str().to_string())
}

enum FileType {
    Txt,
}

impl FileType {
    pub fn from_extension(extension: &str) -> Self {
        match extension {
            _ => FileType::Txt,
        }
    }
}

async fn read_links(file: &FileHandle) -> Result<Vec<String>, anyhow::Error> {
    let path = file.path();
    let file_type = path
        .extension()
        .map(|ext| ext.to_str())
        .flatten()
        .map_or(FileType::Txt, FileType::from_extension);

    let bytes: Vec<u8> = file.read().await;
    let content = str::from_utf8(&bytes)?.to_string();

    match file_type {
        FileType::Txt => {
            let content_split = content.split('\n').map(|line| line.to_string());
            Ok(content_split.collect::<Vec<String>>())
        }
    }
}
