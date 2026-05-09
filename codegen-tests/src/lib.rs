use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{Context, Result};

use dbc_codegen2::utils::Language;
use dbc_codegen2::{app::CodegenPipeline, codegen::config::CodegenConfig};

struct GeneratedFileGuard {
    path: PathBuf,
    original: Option<Vec<u8>>,
}

impl GeneratedFileGuard {
    fn new(path: PathBuf) -> std::io::Result<Self> {
        let original = if path.exists() {
            Some(std::fs::read(&path)?)
        } else {
            None
        };

        Ok(Self { path, original })
    }
}

impl Drop for GeneratedFileGuard {
    fn drop(&mut self) {
        match &self.original {
            Some(content) => {
                let _ = std::fs::write(&self.path, content);
            }
            None => {
                let _ = std::fs::remove_file(&self.path);
            }
        }
    }
}

const DBC_DIR: &str = "./test-files";
const GENERATED_FILE: &str = "../data/generated.rs";
const VALIDATOR_CRATE: &str = "data";
const DEBUG_ENV_VAR: &str = "CODEGEN_TEST_DEBUG";

fn _dbc_files() -> Vec<PathBuf> {
    let base = Path::new(DBC_DIR);
    let mut files = Vec::new();

    fn visit(dir: &Path, files: &mut Vec<PathBuf>) {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_dir() {
                visit(&path, files);
            } else if path
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e.eq_ignore_ascii_case("dbc"))
                .unwrap_or(false)
            {
                files.push(path);
            }
        }
    }

    visit(base, &mut files);
    files.sort();
    files
}

fn _should_pass(file: &Path) -> bool {
    file.starts_with(Path::new(DBC_DIR).join("currently-work"))
}

fn _debug_output_enabled() -> bool {
    env::var(DEBUG_ENV_VAR)
        .map(|value| matches!(value.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"))
        .unwrap_or(false)
}

fn _status_line(message: impl AsRef<str>) {
    let _ = writeln!(io::stderr().lock(), "{}", message.as_ref());
}

fn _run_codegen(input: &Path) -> Result<()> {
    let input_str = input
        .to_str()
        .context("Invalid UTF-8 in input path")?
        .to_string();

    let config = CodegenConfig {
        inputs: vec![input_str],
        output: GENERATED_FILE.into(),
        lang: Language::Rust,
        no_enum_other: true,
        no_enum_dedup: false,
        zero_zero_range_allows_all: false,
        rust_code_injections: HashMap::new(),
        cpp_code_injections: HashMap::new(),
        generate_tests: true,
    };

    CodegenPipeline::run(config).context("codegen failed")?;
    Ok(())
}

fn _cargo_check_data_crate() -> Result<()> {
    let mut command = Command::new("cargo");
    command.args(["check", "-p", VALIDATOR_CRATE]);

    if !_debug_output_enabled() {
        command.stdout(Stdio::null()).stderr(Stdio::null());
    }

    let status = command.status().context("failed to run cargo check")?;

    if !status.success() {
        anyhow::bail!("cargo check failed");
    }

    Ok(())
}

fn _cargo_test_data_crate() -> Result<()> {
    let mut command = Command::new("cargo");
    command.args(["test", "-p", VALIDATOR_CRATE]);

    if !_debug_output_enabled() {
        command.stdout(Stdio::null()).stderr(Stdio::null());
    }

    let status = command.status().context("failed to run cargo test")?;

    if !status.success() {
        anyhow::bail!("cargo test failed");
    }

    Ok(())
}

fn _run_generation_test(file: &Path) -> Result<()> {
    _run_codegen(file).with_context(|| format!("Codegen failed for {:?}", file))?;

    _cargo_check_data_crate().with_context(|| format!("Compilation failed for {:?}", file))?;

    _cargo_test_data_crate().with_context(|| format!("Test failed for {:?}", file))?;

    Ok(())
}

#[test]
fn test_all_dbc_files() -> Result<()> {
    let _guard =
        GeneratedFileGuard::new(GENERATED_FILE.into()).context("Failed to create file guard")?;

    let files = _dbc_files();
    _status_line(format!(
        "Running {} DBC generation fixtures. Set {}=1 to show expected failure details and underlying data crate cargo output.",
        files.len(),
        DEBUG_ENV_VAR
    ));

    let mut failures = Vec::new();
    let debug_output_enabled = _debug_output_enabled();

    for file in files {
        let should_pass = _should_pass(&file);
        let result = _run_generation_test(&file).map_err(|e| format!("{:#}", e));

        match (result, should_pass) {
            (Ok(()), true) => {
                _status_line(format!("{} passed", file.display()));
            }
            (Err(e), false) => {
                if debug_output_enabled {
                    _status_line(format!("{} expectedly failed:\n{}", file.display(), e));
                } else {
                    _status_line(format!("{} expectedly failed", file.display()));
                }
            }
            (Ok(()), false) => {
                _status_line(format!("{} unexpectedly passed", file.display()));
                failures.push((
                    file,
                    "Expected generation pipeline to fail, but it passed".to_string(),
                ));
            }
            (Err(e), true) => {
                _status_line(format!("{} unexpectedly failed", file.display()));
                failures.push((file, e));
            }
        }
    }

    if !failures.is_empty() {
        eprintln!("\n=== FAILURES ===\n");

        for (file, err) in &failures {
            eprintln!("File: {:?}", file);
            eprintln!("{}\n", err);
        }

        anyhow::bail!("{} test(s) failed", failures.len());
    }

    Ok(())
}
