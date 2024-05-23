use std::process::Command;
use serde::Serialize;

#[derive(Serialize)]
struct TestResult {
    test_name: String,
    passed: bool,
    error: Option<String>,
}

#[tokio::main]
async fn main() {
    let test_results = vec![
        run_performance_test().await,
    ];
    
    generate_report(test_results);
}

async fn run_performance_test() -> TestResult {
    let output = Command::new("wrk")
        .arg("-t12")
        .arg("-c400")
        .arg("-d30s")
        .arg("http://example.com")
        .output()
        .expect("failed to execute wrk");

    let passed = output.status.success();

    TestResult {
        test_name: "performance_test".to_string(),
        passed,
        error: if passed { None } else { Some(String::from_utf8_lossy(&output.stderr).to_string()) },
    }
}

fn generate_report(results: Vec<TestResult>) {
    let tera = Tera::new("templates/*.html").unwrap();
    let mut context = Context::new();
    context.insert("results", &results);

    let rendered = tera.render("report.html", &context).unwrap();
    std::fs::write("reports/performance_report.html", rendered).unwrap();
}
