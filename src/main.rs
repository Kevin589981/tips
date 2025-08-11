#![windows_subsystem = "windows"]

use chrono::Local;
use directories::UserDirs;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};
use tokio::process::Command;

const CONFIG_FILE: &str = ".tips.toml";
const TEMP_DIR: &str = "tips_temp";

#[derive(Debug, Default, Serialize, Deserialize)]
struct Config {
    file_type: String,
    editor: String,
}

fn home_dir() -> PathBuf {
    UserDirs::new()
        .expect("无法获取用户目录")
        .home_dir()
        .to_path_buf()
}

fn load_config() -> Config {
    let path = home_dir().join(CONFIG_FILE);
    if !path.exists() {
        let default = Config::default();
        fs::write(&path, toml::to_string_pretty(&default).unwrap()).ok();
        return default;
    }
    toml::from_str(&fs::read_to_string(&path).unwrap_or_default()).unwrap_or_default()
}

fn temp_file_path(home: &Path, ext: &str) -> PathBuf {
    let dir = home.join(TEMP_DIR);
    fs::create_dir_all(&dir).ok();
    let stamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
    let file = dir.join(format!("tips_{}.{}", stamp, ext));
    File::create(&file).ok();
    file
}



#[tokio::main]
async fn main() {
    let home = home_dir();
    let cfg = load_config();
    let file = temp_file_path(&home, &cfg.file_type);

    // 启动编辑器并等待其退出
    let status = Command::new(&cfg.editor)
        .arg("--wait")
        .arg(&file)
        .status()
        .await
        .expect("启动编辑器失败");

    if status.success() {
        fs::remove_file(&file).ok();
    }
}

