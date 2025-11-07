use std::process::Command;

mod model;
pub use model::DiskDrive;

pub fn diskindex_by_driveletter(drive_letter: char) -> Result<usize, Error> {
    let stdout = Command::new("powershell.exe")
        .arg("-command")
        .arg(format!(
            "(Get-Partition -DriveLetter {drive_letter}).DiskNumber"
        ))
        .output()?
        .stdout;

    Ok(String::from_utf8(stdout)
        .expect("failed to parse output")
        .trim()
        .parse()?)
}

pub fn diskindex_by_win32_path(win32_path: impl AsRef<str>) -> Result<usize, Error> {
    let command = format!(
        "(Get-Partition -volume (Get-Volume -Path \"{}\")).DiskNumber",
        win32_path.as_ref()
    );
    let stdout = Command::new("powershell.exe")
        .arg("-command")
        .arg(command)
        .output()?
        .stdout;

    let output = String::from_utf8(stdout)?;
    if output.trim().is_empty() {
        return Err(Error::EmptyOutput);
    }
    Ok(output.trim().parse()?)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("parse error: {0}")]
    ParseError(#[from] std::num::ParseIntError),
    #[error("utf-8 error: {0}")]
    UTF8Error(#[from] std::string::FromUtf8Error),
    #[error("empty output, maybe permission issue!")]
    EmptyOutput,
}
