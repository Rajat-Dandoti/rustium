mod base;
mod locators;
mod tests;

use async_std::task;
use serde::Serialize;
use tera::{Tera, Context};
use tests::functional_tests::example_test;
use std::fs;

#[derive(Serialize)]
struct TestResult {
    test_name: String,
    passed: bool,
    error: Option<String>,
}

#[tokio::main]
async fn main() {
    let test_results = run_functional_tests();
    generate_report(test_results).await;
}

pub fn run_functional_tests() -> Vec<TestResult> {
    let mut results = Vec::new();
    task::block_on(async {
        
        match example_test() {
            Ok(_) => results.push(TestResult {
                test_name: "Example Test".to_string(),
                passed: true,
                error: None,
            }),
            Err(e) => results.push(TestResult {
                test_name: "Example Test".to_string(),
                passed: false,
                error: Some(e.to_string()),
            }),
            }
        });
        results
    }
        

async fn generate_report(results: Vec<TestResult>) {
    let tera = Tera::new("templates/*.html").unwrap();
    let mut context = Context::new();
    context.insert("results", &results);

    let rendered = tera.render("report.html", &context).unwrap();
    fs::write("reports/overall_report.html", rendered).unwrap();
}
