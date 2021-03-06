use crate::chara::CharaConfig;
use anyhow::Context;
use std::path::{Path, PathBuf};

/// read file to toml
fn read_to_toml<T: AsRef<Path>>(toml_path: T) -> anyhow::Result<CharaConfig> {
    let chara_data: CharaConfig = {
        let file_content = std::fs::read_to_string(toml_path).map_err(|e| anyhow::anyhow!(e))?;

        toml::from_str(file_content.as_str())
    }
    .context("不正なTOMLです")?;

    Ok(chara_data)
}

/// directory files push vec
pub async fn dir_files<T: AsRef<Path>>(
    toml_dir_path: T,
) -> Result<Vec<CharaConfig>, anyhow::Error> {
    if toml_dir_path.as_ref().is_dir() {
        let mut vec: Vec<PathBuf> = Vec::new();
        let mut entries = tokio::fs::read_dir(toml_dir_path)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        while let Some(entry) = entries.next_entry().await? {
            let dir_entrey = entry.path();

            vec.push(dir_entrey);
        }

        vec.iter().map(read_to_toml).collect()
    } else {
        Err(anyhow::anyhow!("not directory"))
    }
}

/// directory files push vec no async
pub fn dir_files_noasync<T: AsRef<Path>>(
    toml_dir_path: T,
) -> Result<Vec<CharaConfig>, anyhow::Error> {
    if toml_dir_path.as_ref().is_dir() {
        let mut vec: Vec<PathBuf> = Vec::new();
        let entries = std::fs::read_dir(toml_dir_path).context("ディレクトリが読み込めません")?;
        for entry in entries {
            let dir_entrey = entry?.path();

            vec.push(dir_entrey);
        }

        vec.iter().map(read_to_toml).collect()
    } else {
        Err(anyhow::anyhow!("not directory"))
    }
}
