// File: harness.rs - This file is part of the DPL Toolchain
// Copyright (c) 2026 Dust LLC, and Contributors
// Description:
//   Test suite for dusttest harness functionality.
//   Tests include:
//     - seeds_generation: Verify seed sequence generation
//     - run_tests: Test execution under deterministic seeds
//     - Configuration loading and parsing

use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use tempfile::tempdir;

use dusttest::{generate_seeds, run_tests, TestSpec};

// Test that generate_seeds produces the expected number of seeds and base seed.
#[test]
fn seeds_generation() {
    let (base, seeds) = generate_seeds(Some(100), 5);
    assert_eq!(base, 100);
    assert_eq!(seeds, vec![100, 101, 102, 103, 104]);
}

// Test running commands with deterministic seeds.
#[test]
fn run_tests_with_dummy_script() {
    // create a temporary directory and dummy script
    let dir = tempdir().expect("create temp dir");
    let script_path = dir.path().join("dummy.sh");
    {
        let mut f = File::create(&script_path).expect("create dummy script");
        // The script exits 0 if seed is 1, otherwise non-zero
        writeln!(f, "#!/bin/sh").unwrap();
        writeln!(
            f,
            "if [ \"$DUST_SEED\" -eq 1 ]; then exit 0; else exit 1; fi"
        )
        .unwrap();
    }
    // Build test specification
    let spec = TestSpec {
        name: "dummy".to_string(),
        cmd: format!("sh {}", script_path.display()),
    };
    let seeds = vec![1, 2, 1];
    let results = run_tests(&[spec], &seeds, false);
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.successes, 2); // seeds 1 and 1 pass, seed 2 fails
    assert_eq!(result.total_runs, 3);
    assert_eq!(result.failing_seeds, vec![2]);
}
