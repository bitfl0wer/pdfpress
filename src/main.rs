use std::ffi::OsStr;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::process::Command;

use clap::{command, Parser, ValueEnum};

lazy_static::lazy_static! {
    static ref ARGS: CliArgs = CliArgs::parse();
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct CliArgs {
    /// File path to the .pdf file that you want to compress.
    input: PathBuf,
    /// Output file path for resulting, compressed .pdf file. Defaults to input filepath and -name, where the name has "_pressed" appended.
    output: Option<PathBuf>,
    #[arg(short = 'm', long = "mode", default_value_t = Mode::Ebook)]
    /// Compression mode for resulting .pdf file. Determines resulting quality and file size.
    mode: Mode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Default)]
enum Mode {
    /// 75ppi, lowest file size.
    Screen,
    /// 150ppi, medium file size.
    #[default]
    Ebook,
    /// 300ppi, large file size.
    Prepress,
    /// 600ppi, largest file size.
    Print,
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Screen => write!(f, "screen"),
            Self::Ebook => write!(f, "ebook"),
            Self::Prepress => write!(f, "prepress"),
            Self::Print => write!(f, "print"),
        }
    }
}

fn main() -> anyhow::Result<()> {
    CliArgs::parse();

    let output_file_name = {
        if let Some(output) = ARGS.output.clone() {
            output
        } else {
            let input_file_name = ARGS.input.file_stem().unwrap_or(OsStr::new("pdfpress"));
            ARGS.input
                .with_file_name(format!("{}_pressed.pdf", input_file_name.to_string_lossy()))
        }
    };
    let exec_result = Command::new("gs")
        .arg("-SDEVICE=pdfwrite")
        .arg("-dCompatibilityLevel=1.4")
        .arg("-dNOPAUSE")
        .arg("-dQUIET")
        .arg("-dBATCH")
        .arg(format!(
            "-sOutputFile={}",
            output_file_name.to_string_lossy()
        ))
        .arg(format!(
            "-dPDFSETTINGS=/{}",
            ARGS.mode.to_string().to_lowercase()
        ))
        .arg(ARGS.input.to_string_lossy().to_string())
        .spawn()?
        .wait();

    exec_result.map(|_| ()).map_err(anyhow::Error::from)
}
