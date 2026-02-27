// File: lib.rs - This file is part of the DPL Toolchain
// Copyright (c) 2026 Dust LLC, and Contributors
// Description:
//   Core library for the `dusttest` test harness.
//   Provides:
//     - Test configuration file parsing
//     - Deterministic seed sequence generation
//     - Command execution under controlled seeds
//     - Language-agnostic test execution

//! Core library for the `dusttest` test harness.
//!
//! This module defines data structures and functions for parsing test
//! configuration files, generating deterministic seed sequences, and
//! executing commands under those seeds.  The harness is designed to
//! be language agnostic: it simply runs shell commands with an
//! environment variable set to control randomness.

use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::process::{Command, Stdio};

/// Representation of a test specification in the configuration file.
#[derive(Debug, Deserialize, Clone)]
pub struct TestSpec {
    /// Name of the test.
    pub name: String,
    /// Shell command to run for this test.
    pub cmd: String,
}

/// Top‑level configuration structure mapping to the TOML format.
#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "test")]
    pub tests: Vec<TestSpec>,
}

/// Result of running a single test across multiple seeds.
#[derive(Debug)]
pub struct TestResult {
    pub name: String,
    pub successes: usize,
    pub total_runs: usize,
    pub failing_seeds: Vec<u64>,
    pub outputs: Vec<(u64, String)>,
}

/// Load a configuration file from the given path.
pub fn load_config(path: &str) -> io::Result<Vec<TestSpec>> {
    let content = fs::read_to_string(path)?;
    let config: Config =
        toml::from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(config.tests)
}

/// Generate a deterministic sequence of seeds for an ensemble run.
///
/// If `count` is 0, an empty vector is returned.  If `base_seed` is
/// provided, seeds are `base_seed + i` for `i` in `[0, count)`.  Otherwise a
/// random base seed is generated from a `StdRng` seeded from the OS.
pub fn generate_seeds(base_seed: Option<u64>, count: usize) -> (u64, Vec<u64>) {
    if count == 0 {
        return (base_seed.unwrap_or(0), Vec::new());
    }
    let base = match base_seed {
        Some(s) => s,
        None => {
            // generate a random seed using StdRng seeded from entropy
            let mut rng = StdRng::from_entropy();
            rng.gen::<u64>()
        }
    };
    let seeds: Vec<u64> = (0..count).map(|i| base.wrapping_add(i as u64)).collect();
    (base, seeds)
}

/// Run a suite of tests for a given set of seeds.
///
/// * `tests` – list of test specifications to run.
/// * `seeds` – seeds to use for each ensemble run.
/// * `verbose` – whether to print per‑run output.
///
/// Returns a vector of results, one per test.
pub fn run_tests(tests: &[TestSpec], seeds: &[u64], verbose: bool) -> Vec<TestResult> {
    let mut results = Vec::new();
    for test in tests {
        let mut successes = 0;
        let mut failing_seeds = Vec::new();
        let mut outputs = Vec::new();
        for &seed in seeds {
            // Build command: we run through `sh -c` to allow complex commands
            let mut cmd = Command::new("sh");
            cmd.arg("-c").arg(&test.cmd);
            cmd.env("DUST_SEED", seed.to_string());
            // Capture stdout and stderr
            cmd.stdout(Stdio::piped());
            cmd.stderr(Stdio::piped());
            match cmd.output() {
                Ok(output) => {
                    let status = output.status;
                    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                    let combined = if !stderr.is_empty() {
                        format!("{}\n{}", stdout, stderr)
                    } else {
                        stdout
                    };
                    if status.success() {
                        successes += 1;
                    } else {
                        failing_seeds.push(seed);
                    }
                    if verbose {
                        outputs.push((seed, combined));
                    }
                }
                Err(e) => {
                    // treat spawn error as failure
                    failing_seeds.push(seed);
                    if verbose {
                        outputs.push((seed, format!("error invoking command: {}", e)));
                    }
                }
            }
        }
        let result = TestResult {
            name: test.name.clone(),
            successes,
            total_runs: seeds.len(),
            failing_seeds,
            outputs,
        };
        results.push(result);
    }
    results
}

/// Format the summary of test results into a human readable string.
pub fn format_summary(results: &[TestResult], base_seed: u64) -> String {
    let mut summary = String::new();
    summary.push_str(&format!(
        "Running {} tests with base seed {}\n",
        results.len(),
        base_seed
    ));
    let mut all_pass = true;
    for result in results {
        if result.total_runs == 0 {
            summary.push_str(&format!("Test {}: SKIPPED (no seeds)\n", result.name));
            continue;
        }
        if result.failing_seeds.is_empty() {
            summary.push_str(&format!(
                "Test {}: pass on {}/{} seeds\n",
                result.name, result.successes, result.total_runs
            ));
        } else {
            all_pass = false;
            summary.push_str(&format!(
                "Test {}: pass on {}/{} seeds (failures on seeds: {:?})\n",
                result.name, result.successes, result.total_runs, result.failing_seeds
            ));
        }
    }
    if all_pass {
        summary.push_str("All tests passed!\n");
    } else {
        summary.push_str("Some tests failed.\n");
    }
    summary
}
