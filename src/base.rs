use thirtyfour::prelude::*;
use chromedriver_manager::{manager::Handler, loglevel::LogLevel};
use std::sync::Arc;
use std::process::Child;
//use tokio::process::Child;

pub struct TestBase {
    pub driver: Arc<WebDriver>,
    chromedriver: Option<Child>,
}

impl TestBase {
    pub async fn new() -> Self {
        // Create Chrome capabilities
        let mut caps = DesiredCapabilities::chrome();

        // Launch chromedriver on port 9515
        let chromedriver = Handler::new()
            .launch_chromedriver(&mut caps, "9515", LogLevel::Off)
            .await
            .expect("Failed to launch chromedriver");

        // Connect to chrome on the same port
        let driver = WebDriver::new("http://localhost:9515", caps)
            .await
            .expect("Failed to connect to WebDriver");

        TestBase {
            driver: Arc::new(driver),
            chromedriver: Some(chromedriver),
        }
    }

    pub async fn close(mut self) {
        <thirtyfour::WebDriver as Clone>::clone(&self.driver).quit().await.expect("Failed to quit WebDriver");

        if let Some(mut chromedriver) = self.chromedriver.take() {
            chromedriver
                .kill()
                .expect("Failed to kill chromedriver process");
        }
    }
}
