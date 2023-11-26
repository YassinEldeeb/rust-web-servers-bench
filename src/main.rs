use regex::Regex;
use std::fs::File;
use std::io::Write;
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::thread;
use std::time::Duration;

// Struct to hold the performance metrics
#[derive(Debug)]
struct PerformanceMetrics {
    name: String,
    total_requests: u32,
    success_rate: f32,
    throughput: f32,
    avg_duration: f32,
    errors: u32,
    peak_response: f32,
    vus: u32,
}

fn main() -> std::io::Result<()> {
    let servers = vec!["actix-web", "hyper", "rocket", "async_uws"];

    let mut metrics: Vec<PerformanceMetrics> = Vec::new();

    for server in servers {
        println!("Starting {} server...", server);
        let mut child = Command::new("cargo")
            .args(["run", "--release"])
            .current_dir(server)
            .stdout(Stdio::null()) // Ignore stdout of the server
            .spawn()
            .expect("Failed to start server");

        // Allow server to fully launch
        thread::sleep(Duration::from_secs(5));

        // Run the K6 benchmark
        println!("Running K6 benchmark for {}...", server);
        let output = Command::new("k6")
            .args([
                "run",
                "-e",
                "BENCH_VUS=50",
                "-e",
                "BENCH_OVER_TIME=5s",
                "-e",
                format!("SERVER_ENDPOINT=http://localhost:3000").as_str(),
                "./bench/constant-vus.k6.js",
            ])
            .output()
            .expect("Failed to run K6");

        // Ensure the server process is killed before parsing to avoid locking
        let _ = child.kill();
        let _ = child.wait();

        // Parse the K6 output and collect metrics
        if !output.status.success() {
            eprintln!("K6 command failed for server: {}", server);
            eprintln!("Status: {:?}", output.status);
            eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
            continue; // Skip this server and move to the next
        }

        // Parse the K6 output and collect metrics
        match parse_k6_output(server, &output.stdout) {
            Ok(parsed_metrics) => metrics.push(parsed_metrics),
            Err(e) => eprintln!("Failed to parse K6 output for server {}: {}", server, e),
        }

        // Cooldown phase to ensure the port is free for the next server
        thread::sleep(Duration::from_secs(10));
    }

    // Sort metrics from faster to slower based on throughput
    metrics.sort_by(|a, b| b.throughput.partial_cmp(&a.throughput).unwrap());

    // Generate README.md
    generate_readme(metrics)?;

    Ok(())
}

fn extract_metric<T: FromStr>(text: &str, re: &Regex) -> Result<T, String> {
    re.captures(text)
        .and_then(|cap| cap[1].parse::<T>().ok())
        .ok_or_else(|| "Failed to parse metric".to_string())
}

fn parse_k6_output(server: &str, output: &[u8]) -> Result<PerformanceMetrics, String> {
    let output_str = String::from_utf8_lossy(output);

    // Define regex patterns for the metrics
    let re_total_requests = Regex::new(r"http_reqs\.*:\s*([\d,]+)").unwrap();
    let re_success_rate = Regex::new(r"checks\.*:\s*([\d.]+)%").unwrap();
    let re_avg_duration = Regex::new(r"http_req_duration\.*:\s*avg=([\d.]+)ms").unwrap();
    let re_errors = Regex::new(r"http_req_failed\.*:\s*([\d.]+)%").unwrap();
    let re_peak_response = Regex::new(r"http_req_duration\.*:\s*max=([\d.]+)ms").unwrap();
    let re_throughput = Regex::new(r"http_reqs\.*:\s*([\d,]+)\s*(\d+\.\d+)/s").unwrap();

    // Extract metrics using the utility function
    let total_requests = extract_metric::<String>(&output_str, &re_total_requests)?
        .replace(",", "")
        .parse::<u32>()
        .map_err(|_| "Failed to parse total requests")?;

    let success_rate = extract_metric::<f32>(&output_str, &re_success_rate)?;
    let avg_duration = extract_metric::<f32>(&output_str, &re_avg_duration)?;

    // Note: We assume that http_req_failed percentage is the error rate, but if it's a count, you should parse it as such.
    let errors_rate = extract_metric::<f32>(&output_str, &re_errors)?;
    let errors = (errors_rate / 100.0 * total_requests as f32).round() as u32;

    let peak_response = extract_metric::<f32>(&output_str, &re_peak_response)?;
    let throughput = extract_metric::<String>(&output_str, &re_throughput)?
        .replace(",", "")
        .parse::<f32>()
        .map_err(|_| "Failed to parse throughput")?;

    // Return the parsed metrics
    Ok(PerformanceMetrics {
        name: server.to_string(),
        total_requests,
        success_rate,
        throughput,
        avg_duration,
        errors,
        peak_response,
        vus: 200, // This should be set based on the `vus` used in the test
    })
}

fn generate_readme(metrics: Vec<PerformanceMetrics>) -> std::io::Result<()> {
    let mut file = File::create("README.md")?;
    writeln!(file, "# HTTP Server Performance Comparison\n")?;
    writeln!(file, "This document provides a performance comparison between the following Rust HTTP servers using K6 benchmarks.\n")?;
    writeln!(file, "| Server    | Total Requests | Success Rate | Throughput / RPS | Avg Request Duration (ms) | Errors | Peak Response Time (ms) | VUs |")?;
    writeln!(file, "|-----------|----------------|--------------|-----------------------|---------------------------|--------|-------------------------|-----|")?;

    for m in &metrics {
        writeln!(
            file,
            "| {} | {} | {}% | ~{} | {} | {} | {} | {} |",
            m.name,
            m.total_requests,
            m.success_rate,
            m.throughput,
            m.avg_duration,
            m.errors,
            m.peak_response,
            m.vus
        )?;
    }

    Ok(())
}
