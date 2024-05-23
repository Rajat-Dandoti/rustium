mod tests;

use std::process::Command;
use serde::Serialize;
use tera::{Tera, Context};
use std::fs;

#[derive(Serialize)]
pub struct TestResult {
    pub test_name: String,
    pub passed: bool,
    pub error: Option<String>,
}

fn main() {
    let test_results = run_tests();
    generate_report(test_results);
}

fn run_tests() -> Vec<TestResult> {
    let mut results = Vec::new();

    results.extend(run_functional_tests());
    results.push(run_performance_tests());
    results.push(run_security_tests());
    results.push(run_accessibility_tests());

    results
}

fn run_functional_tests() -> Vec<TestResult> {
    let mut test_results = Vec::new();
    test_results.push(tests::functional_tests::run_example_functional_test());
    test_results
}

fn run_performance_tests() -> TestResult {
    let output = Command::new("wrk")
        .arg("-t12")
        .arg("-c400")
        .arg("-d30s")
        .arg("http://example.com")
        .output()
        .expect("failed to execute wrk");

    let passed = output.status.success();

    TestResult {
        test_name: "Performance Tests".to_string(),
        passed,
        error: if passed { None } else { Some(String::from_utf8_lossy(&output.stderr).to_string()) },
    }
}

fn run_security_tests() -> TestResult {
    let output = Command::new("nikto")
        .arg("-h")
        .arg("http://example.com")
        .output()
        .expect("failed to execute nikto");

    let passed = output.status.success();

    TestResult {
        test_name: "Security Tests".to_string(),
        passed,
        error: if passed { None } else { Some(String::from_utf8_lossy(&output.stderr).to_string()) },
    }
}

fn run_accessibility_tests() -> TestResult {
    let output = Command::new("axe-cli")
        .arg("http://example.com")
        .output()
        .expect("failed to execute axe-cli");

    let passed = output.status.success();

    TestResult {
        test_name: "Accessibility Tests".to_string(),
        passed,
        error: if passed { None } else { Some(String::from_utf8_lossy(&output.stderr).to_string()) },
    }
}

fn generate_report(results: Vec<TestResult>) {
    let tera = Tera::new("templates/*.html").unwrap();
    let mut context = Context::new();
    context.insert("results", &results);

    let rendered = tera.render("report.html", &context).unwrap();
    fs::write("reports/overall_report.html", rendered).unwrap();
}
