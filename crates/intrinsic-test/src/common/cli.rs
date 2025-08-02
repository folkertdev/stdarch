use itertools::Itertools;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub enum Language {
    Rust,
    C,
}

pub enum FailureReason {
    RunC(String),
    RunRust(String),
    Difference(String, String, String),
}

/// Intrinsic test tool
#[derive(clap::Parser)]
#[command(
    name = "Intrinsic test tool",
    about = "Generates Rust and C programs for intrinsics and compares the output"
)]
pub struct Cli {
    /// The input file containing the intrinsics
    pub input: PathBuf,

    /// The rust toolchain to use for building the rust code
    #[arg(long)]
    pub toolchain: Option<String>,

    /// The C++ compiler to use for compiling the c++ code
    #[arg(long, default_value_t = String::from("clang++"))]
    pub cpp_compiler: String,

    /// Run the programs under emulation with this command
    #[arg(long)]
    pub runner: Option<String>,

    /// Filename for a list of intrinsics to skip (one per line)
    #[arg(long)]
    pub skip: Option<PathBuf>,

    /// Regenerate test programs, but don't build or run them
    #[arg(long)]
    pub generate_only: bool,

    /// Pass a target the test suite
    #[arg(long, default_value_t = String::from("armv7-unknown-linux-gnueabihf"))]
    pub target: String,

    /// Set the linker
    #[arg(long)]
    pub linker: Option<String>,

    /// Set the sysroot for the C++ compiler
    #[arg(long)]
    pub cxx_toolchain_dir: Option<String>,
}

pub enum Mode {
    BuildAndRun {
        cpp_compiler: String,
        toolchain: String,
        runner: String,
        linker: Option<String>,
        cxx_toolchain_dir: Option<String>,
    },
    GenerateOnly,
}

pub struct Config {
    pub filename: PathBuf,
    pub mode: Mode,
    pub target: String,
    pub skip: Vec<String>,
}

impl Config {
    pub fn new(cli_options: Cli) -> Self {
        let filename = cli_options.input;
        let runner = cli_options.runner.unwrap_or_default();
        let target = cli_options.target;
        let linker = cli_options.linker;
        let cxx_toolchain_dir = cli_options.cxx_toolchain_dir;

        let skip = if let Some(filename) = cli_options.skip {
            let data = std::fs::read_to_string(&filename).expect("Failed to open file");
            data.lines()
                .map(str::trim)
                .filter(|s| !s.contains('#'))
                .map(String::from)
                .collect_vec()
        } else {
            Default::default()
        };

        let mode = if cli_options.generate_only {
            Mode::GenerateOnly
        } else {
            Mode::BuildAndRun {
                cpp_compiler: cli_options.cpp_compiler,
                toolchain: match cli_options.toolchain {
                    None => String::new(),
                    Some(toolchain) => format!("+{toolchain}"),
                },
                runner,
                linker,
                cxx_toolchain_dir,
            }
        };

        Self {
            target,
            skip,
            filename,
            mode,
        }
    }
}
