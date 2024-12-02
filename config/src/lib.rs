use lazy_static::lazy_static;
use serde::Deserialize;
use std::{fs, path::PathBuf};

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub ollama: OllamaConfig,
    pub hoopics: HoopicsConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OllamaConfig {
    pub model: String,
    pub url: String,
}

impl Default for OllamaConfig {
    fn default() -> Self {
        OllamaConfig {
            url: "https://u447140-b619-b81b9121.bjb1.seetacloud.com:8443/ollama/generate".to_string(), // 自定义默认URL
            model: "impactframes/llama3_ifai_sd_prompt_mkr_q4km".to_string(),   // 自定义默认模型
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct HoopicsConfig {
    pub api_key: String,
    pub endpoint: String,
}

// 全局静态配置变量
lazy_static! {
    pub static ref CONFIG: Config = load_config("config.toml").expect("Failed to load config");
}

// 从文件加载配置
fn load_config(filename: &str) -> Result<Config, Box<dyn std::error::Error>> {
    // 获取项目根目录
    let mut path = PathBuf::from(std::env::current_dir()?);
    
    // 将配置文件名添加到路径中
    path.push(filename);
    
    let contents = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}

// 根据 section name 获取配置
pub fn get(section: &str) -> Option<&'static dyn std::fmt::Debug> {
    match section {
        "ollama" => Some(&CONFIG.ollama),
        "hoopics" => Some(&CONFIG.hoopics),
        _ => None,
    }
}