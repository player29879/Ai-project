use std::{fs, time::Duration};

use regex::Regex;
use tokio::{sync::mpsc::Sender};

use crate::local_shinkai_node::shinkai_node_options::ShinkaiNodeOptions;

use super::{
    logger::LogEntry,
    process_handler::{ProcessHandler, ProcessHandlerEvent},
    process_utils::options_to_env,
};

pub struct ShinkaiNodeProcessHandler {
    default_node_storage_path: String,
    process_handler: ProcessHandler,
    options: ShinkaiNodeOptions,
}

impl ShinkaiNodeProcessHandler {
    const HEALTH_REQUEST_TIMEOUT_MS: u64 = 250;
    const HEALTH_TIMEOUT_MS: u64 = 5000;
    const PROCESS_NAME: &'static str = "shinkai-node";
    const READY_MATCHER: &'static str = "listening on ";

    pub fn new(
        event_sender: Sender<ProcessHandlerEvent>,
        default_node_storage_path: String,
    ) -> Self {
        let ready_matcher = Regex::new(Self::READY_MATCHER).unwrap();
        let options = ShinkaiNodeOptions::with_storage_path(default_node_storage_path.clone());
        let process_handler =
            ProcessHandler::new(Self::PROCESS_NAME.to_string(), event_sender, ready_matcher);
        ShinkaiNodeProcessHandler {
            default_node_storage_path: default_node_storage_path.clone(),
            process_handler,
            options,
        }
    }

    fn get_base_url(&self) -> String {
        let ip = self.options.clone().node_api_ip.unwrap();
        let port = self.options.clone().node_api_port.unwrap();
        let base_url = format!("http://{}:{}", ip, port);
        base_url
    }

    async fn health(base_url: &str, timeout_ms: u64) -> Result<bool, ()> {
        let url = format!("{}/v1/shinkai_health", base_url);
        let client = reqwest::Client::new();
        if let Ok(response) = client
            .get(&url)
            .timeout(std::time::Duration::from_millis(timeout_ms))
            .send()
            .await
        {
            Ok(response.status() == reqwest::StatusCode::OK)
        } else {
            Ok(false)
        }
    }

    async fn wait_shinkai_node_server(&self) -> Result<(), String> {
        let timeout = Duration::from_millis(Self::HEALTH_TIMEOUT_MS);
        let start_time = std::time::Instant::now();
        let base_url = self.get_base_url();
        tokio::select! {
            _ = tokio::time::sleep(timeout) => {
                let elapsed = start_time.elapsed();
                Err(format!("wait shinkai-node server timeout after {}ms", elapsed.as_millis()))
            }
            _ = tokio::spawn(async move {
                loop {
                    match Self::health(base_url.as_str(), Self::HEALTH_REQUEST_TIMEOUT_MS).await {
                        Ok(true) => break,
                        Ok(false) | Err(_) => tokio::time::sleep(Duration::from_millis(50)).await
                    }
                }
            }) => {
                Ok(())
            }
        }
    }

    pub fn set_options(&mut self, options: ShinkaiNodeOptions) -> ShinkaiNodeOptions {
        self.options = ShinkaiNodeOptions::from_merge(self.options.clone(), options);
        self.options.clone()
    }

    pub async fn remove_storage(&self, preserve_keys: bool) -> Result<(), String> {
        if self.process_handler.is_running().await {
            return Err("can't remove node storage while it's running".to_string());
        }
        let options = self.options.clone();
        let storage_path = options.node_storage_path.unwrap();
        for entry in fs::read_dir(storage_path).map_err(|e| format!("Failed to read storage directory: {}", e))? {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();
            if path.is_dir() {
                fs::remove_dir_all(path).map_err(|e| format!("Failed to remove directory: {}", e))?;
            } else {
                if preserve_keys && path.ends_with(".secret") {
                    continue;
                }
                fs::remove_file(path).map_err(|e| format!("Failed to remove file: {}", e))?;
            }
        }
        Ok(())
    }

    pub async fn spawn(&self) -> Result<(), String> {
        let env = options_to_env(&self.options);
        self.process_handler.spawn(env, [].to_vec()).await?;
        if let Err(e) = self.wait_shinkai_node_server().await {
            self.process_handler.kill().await;
            return Err(e);
        }
        Ok(())
    }

    pub async fn get_last_n_logs(&self, n: usize) -> Vec<LogEntry> {
        self.process_handler.get_last_n_logs(n).await
    }

    pub fn set_default_options(&mut self) -> ShinkaiNodeOptions {
        self.options =
            ShinkaiNodeOptions::with_storage_path(self.default_node_storage_path.clone());
        self.options.clone()
    }

    pub fn get_options(&self) -> ShinkaiNodeOptions {
        self.options.clone()
    }

    pub async fn is_running(&self) -> bool {
        self.process_handler.is_running().await
    }

    pub async fn kill(&self) {
        self.process_handler.kill().await;
    }
}
