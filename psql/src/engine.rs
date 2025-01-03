use config::CONFIG;
use primitives::{job_status::JobResult, Job, ModelType};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::OnceCell;
use tokio_postgres::{Client, Error, NoTls};

pub struct Engine {
    pub client: Client,
}

pub static ENGINE: OnceCell<Arc<Mutex<Engine>>> = OnceCell::const_new();
pub async fn get_global_engine() -> Arc<Mutex<Engine>> {
    ENGINE
        .get_or_init(|| async {
            let engine = Engine::new().await;
            Arc::new(Mutex::new(engine))
        })
        .await
        .clone() // Clone the Arc to return it
}

impl Engine {
    pub async fn new() -> Self {
        let (client, connection) = tokio_postgres::connect(&CONFIG.postgres.to_string(), NoTls)
            .await
            .expect("Postgres server is down!");

        // Spawn the connection on a separate task to handle it asynchronously
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        Self { client }
    }

    pub async fn fetch_pending_or_processing_job_ids(
        &mut self,
        model_type: ModelType,
    ) -> Result<Vec<String>, Error> {
        let client = &mut self.client;
        // Define your query
        let query = "SELECT * FROM hjobs WHERE job_status = $1 AND job_model = $2";

        let mut results = Vec::new();

        if let Ok(rows) = client
            .query(query, &[&"pending", &model_type.to_string()])
            .await
        {
            if rows.len() > 0 {
                for row in rows {
                    let job_id: String = row.get(2);
                    results.push(job_id);
                }

                // println!(">> Pending IDS: {:#?}", results);
            }
        }

        Ok(results)
    }

    // Save job to hjobs table
    pub async fn save_job(&mut self, job: Job) -> Result<(), tokio_postgres::Error> {
        self.insert_hjob(job.into()).await
    }
}

impl Engine {
    pub async fn update_job_status(&mut self, job_results: &Vec<JobResult>) -> Result<(), Error> {
        for job_result in job_results {
            let query = "UPDATE hjobs SET job_status = $1 WHERE job_id = $2";

            // 执行更新操作
            let _ = self
                .client
                .execute(query, &[&job_result.status, &job_result.job_id])
                .await?;
        }

        Ok(())
    }

    pub async fn update_output_image_url(
        &mut self,
        job_results: &Vec<JobResult>,
    ) -> Result<(), Error> {
        for job_result in job_results {
            if job_result.status == "success" {
                let query = "UPDATE hjobs SET output_image_url = $1 WHERE job_id = $2";

                let output_image_urls = &job_result.file_urls;
                if output_image_urls.len() > 0 {
                    // 执行更新操作
                    let _ = self
                        .client
                        .execute(query, &[&output_image_urls[0], &job_result.job_id])
                        .await?;
                }
            }
        }

        Ok(())
    }
}
