use lazy_static::lazy_static;
use serde::Deserialize;
use std::fmt;
use std::{fs, path::PathBuf};

#[derive(Deserialize, Debug, Clone)]
pub struct OllamaConfig {
    pub url: String,
    pub model: String,
}

impl Default for OllamaConfig {
    fn default() -> Self {
        OllamaConfig {
            url: "https://u447140-b619-b81b9121.bjb1.seetacloud.com:8443/ollama/generate"
                .to_string(),
            model: "impactframes/llama3_ifai_sd_prompt_mkr_q4km".to_string(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ServiceConfig {
    pub api_key: String,
    pub endpoint: String,
    pub port: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PsqlConfig {
    pub host: String,
    pub port: u32,
    pub user: String,
    pub password: String,
    pub dbname: String,
}

impl fmt::Display for PsqlConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "host={} user={} password={} port={} dbname={}",
            self.host, self.user, self.password, self.port, self.dbname
        )
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct SDXLConfig {
    pub normal: String,
    pub cartoon: String,
    pub cyberpunk: String,
    pub film: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FluxConfig {
    pub normal: String,
    pub cartoon: String,
    pub cyberpunk: String,
    pub film: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RefresherConfig {
    pub print_log: bool,
    pub sdxl_interval: u64,
    pub flux_interval: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CheckpointsConfig {
    pub max_length_prompt: usize,
    pub max_length_negative_prompt: usize,
    pub max_length_description: usize,
    pub max_length_img_link: usize,
    pub max_image_width: u16,
    pub min_image_width: u16,
    pub max_image_height: u16,
    pub min_image_height: u16,
}

#[derive(Deserialize, Debug, Clone)]
pub struct WorkerConfig {
    pub max_buffer_size: usize,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub ollama: OllamaConfig,
    pub service: ServiceConfig,
    pub postgres: PsqlConfig,
    pub sdxl: SDXLConfig,
    pub flux: FluxConfig,
    pub refresher: RefresherConfig,
    pub checkpoints: CheckpointsConfig,
    pub worker: WorkerConfig,
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
