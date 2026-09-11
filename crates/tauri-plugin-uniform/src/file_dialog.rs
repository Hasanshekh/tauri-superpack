use crate::error::{Error, Result};
use rfd::AsyncFileDialog;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenFileFilter {
    pub name: String,
    pub extensions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenDialogOptions {
    pub title: Option<String>,
    pub default_path: Option<String>,
    pub filters: Option<Vec<OpenFileFilter>>,
    pub multiple: Option<bool>,
    pub directory: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveDialogOptions {
    pub title: Option<String>,
    pub default_path: Option<String>,
    pub default_name: Option<String>,
    pub filters: Option<Vec<OpenFileFilter>>,
}

pub async fn open_file(options: Option<OpenDialogOptions>) -> Result<Vec<String>> {
    let opts = options.unwrap_or(OpenDialogOptions {
        title: None,
        default_path: None,
        filters: None,
        multiple: None,
        directory: None,
    });

    let mut dialog = AsyncFileDialog::new();

    if let Some(title) = &opts.title {
        dialog = dialog.set_title(title);
    }
    if let Some(path) = &opts.default_path {
        dialog = dialog.set_directory(path);
    }
    if let Some(filters) = &opts.filters {
        for f in filters {
            let ext_slices: Vec<&str> = f.extensions.iter().map(|s| s.as_str()).collect();
            dialog = dialog.add_filter(&f.name, &ext_slices);
        }
    }

    if opts.directory.unwrap_or(false) {
        if opts.multiple.unwrap_or(false) {
            let folders = dialog.pick_folders().await;
            match folders {
                Some(handles) => Ok(handles
                    .into_iter()
                    .map(|h| h.path().to_string_lossy().to_string())
                    .collect()),
                None => Err(Error::DialogCancelled),
            }
        } else {
            let folder = dialog.pick_folder().await;
            match folder {
                Some(h) => Ok(vec![h.path().to_string_lossy().to_string()]),
                None => Err(Error::DialogCancelled),
            }
        }
    } else if opts.multiple.unwrap_or(false) {
        let files = dialog.pick_files().await;
        match files {
            Some(handles) => Ok(handles
                .into_iter()
                .map(|h| h.path().to_string_lossy().to_string())
                .collect()),
            None => Err(Error::DialogCancelled),
        }
    } else {
        let file = dialog.pick_file().await;
        match file {
            Some(h) => Ok(vec![h.path().to_string_lossy().to_string()]),
            None => Err(Error::DialogCancelled),
        }
    }
}

pub async fn save_file(options: Option<SaveDialogOptions>) -> Result<String> {
    let opts = options.unwrap_or(SaveDialogOptions {
        title: None,
        default_path: None,
        default_name: None,
        filters: None,
    });

    let mut dialog = AsyncFileDialog::new();

    if let Some(title) = &opts.title {
        dialog = dialog.set_title(title);
    }
    if let Some(path) = &opts.default_path {
        dialog = dialog.set_directory(path);
    }
    if let Some(name) = &opts.default_name {
        dialog = dialog.set_file_name(name);
    }
    if let Some(filters) = &opts.filters {
        for f in filters {
            let ext_slices: Vec<&str> = f.extensions.iter().map(|s| s.as_str()).collect();
            dialog = dialog.add_filter(&f.name, &ext_slices);
        }
    }

    let file = dialog.save_file().await;
    match file {
        Some(h) => Ok(h.path().to_string_lossy().to_string()),
        None => Err(Error::DialogCancelled),
    }
}
