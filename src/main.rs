use std::fs::File;
use std::fmt::{self, Display};
use std::io::{Read, Write};
use intermediate::compute;
use std::env;

use representation::ParseError;
use solution::SolutionError;

mod rckey;
mod intermediate;
mod representation;
mod solution;

#[derive(Debug)]
pub enum FileMode {
    Read,
    Write
}

#[derive(Debug)]
pub struct FileHandlingError {
    name: String,
    mode: FileMode
}

impl FileHandlingError {
    pub fn read_error(name: impl Into<String>) -> Self {
        FileHandlingError { name: name.into(), mode: FileMode::Read }
    }

    pub fn write_error(name: impl Into<String>) -> Self {
        FileHandlingError { name: name.into(), mode: FileMode::Write }
    }
}

impl Display for FileHandlingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.mode {
            FileMode::Read => write!(f, "Failed to read the {} file.", self.name),
            FileMode::Write => write!(f, "Failed to write the {} file.", self.name)
        }
    }
}

#[derive(Debug)]
pub enum ProgramError {
    FileHandlingError(FileHandlingError),
    WrongNumberOfArguments,
    ParseError,
    SolutionError
}

impl From<FileHandlingError> for ProgramError {
    fn from(value: FileHandlingError) -> Self {
        ProgramError::FileHandlingError(value)
    }
}

impl From<ParseError> for ProgramError {
    fn from(_: ParseError) -> Self {
        ProgramError::ParseError
    }
}

impl From<SolutionError> for ProgramError {
    fn from(_: SolutionError) -> Self {
        ProgramError::SolutionError
    }
}

impl Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProgramError::FileHandlingError(err) => write!(f, "{}", err),
            ProgramError::WrongNumberOfArguments => write!(f, "Must enter exactly 4 arguments."),
            ProgramError::ParseError => write!(f, "Failed to parse file."),
            ProgramError::SolutionError => write!(f, "The solution failed due to programmer error.")
        }
    }
}

fn read_file(name: impl Into<String>, path: impl Into<String>) -> Result<String, FileHandlingError> {
    let name = name.into();
    let mut file = File::open(path.into()).map_err(|_| FileHandlingError::read_error(&name))?;
    let mut result = String::new();
    file.read_to_string(&mut result).map_err(|_| FileHandlingError::read_error(&name))?;
    return Ok(result);
}

fn write_file(name: impl Into<String>, path: impl Into<String>, value: String) -> Result<(), FileHandlingError> {
    let name = name.into();
    let mut file = File::create(path.into()).map_err(|_| FileHandlingError::write_error(&name))?;
    write!(&mut file, "{}", value).map_err(|_| FileHandlingError::write_error(&name))?;
    Ok(())
}

fn handle() -> Result<(), ProgramError> {
    let args = env::args().collect::<Vec<_>>();
    let [staff_path, project_path, student_path, output_path] = &args[1..] else {return Err(ProgramError::WrongNumberOfArguments)};

    let staffs = read_file("staff", staff_path)?;
    let projects = read_file("projects", project_path)?;
    let students = read_file("students", student_path)?;
    let result = compute(staffs, projects, students)?;
    write_file("output", output_path, result)?;

    Ok(())
}

fn main() {
    let result = handle();
    if let Err(err) = result {
        println!("ERROR: {}", err);
    }
}
