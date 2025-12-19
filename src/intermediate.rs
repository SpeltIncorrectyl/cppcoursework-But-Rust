use crate::representation::*;
use crate::ProgramError;
use crate::solution::solve;

pub fn compute(staffs_file: String, projects_file: String, students_file: String) -> Result<String, ProgramError> {
    let problem = Problem::from_files(&staffs_file, &projects_file, &students_file)?;
    let solution = solve(problem)?;
    return Ok(format!("{}", solution));
}