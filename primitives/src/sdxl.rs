use serde::{Deserialize, Serialize};

use crate::{Job, JobStyle, ModelType};

#[derive(Debug, Serialize, Deserialize)]
pub struct SDXLJobRequest {
    pub prompt: String,
    pub job_id: String,
    pub style: JobStyle,
    pub model_type: ModelType,
    pub width: u16,
    pub height: u16,
}

impl From<Job> for SDXLJobRequest {
    fn from(item: Job) -> Self {
        SDXLJobRequest {
            prompt: item.prompt,
            job_id: item.id,
            style: item.job_style,
            model_type: item.model,
            width: item.width,
            height: item.height,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Img2ImgRequest {
    prompt: String,
    job_id: String,
    img_url: String,
    style: JobStyle,
    model_type: ModelType,
}

impl From<Job> for Img2ImgRequest {
    fn from(item: Job) -> Self {
        Img2ImgRequest {
            prompt: item.prompt,
            job_id: item.id,
            style: item.job_style,
            model_type: item.model,
            img_url: item.img_link,
        }
    }
}
