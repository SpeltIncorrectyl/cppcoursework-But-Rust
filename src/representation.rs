use std::str::FromStr;
use std::rc::Rc;
use std::fmt::{self, Display};

#[allow(non_snake_case)]
pub struct Staff {
    pub staffID: String,
    pub load: u32,
    pub subject_areas: Vec<String>
}

#[allow(non_snake_case)]
pub struct Project {
    pub projectID: u32,
    pub proposer: Rc<Staff>,
    pub subject_area: String,
    pub multiplicity: u32,
}

#[allow(non_snake_case)]
pub struct Student {
    pub studentID: String,
    pub choices: Vec<Rc<Project>>
}

pub struct Problem {
    pub staffs: Vec<Rc<Staff>>,
    pub projects: Vec<Rc<Project>>,
    pub students: Vec<Rc<Student>>
}

#[derive(Debug)]
pub struct ParseError;

impl FromStr for Staff {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Staff, ParseError> {
        let mut iter = s.split(" ");
        Ok(Staff {
            staffID: iter.next()
                .ok_or(ParseError)?
                .to_string(),
            load: iter.next()
                .ok_or(ParseError)?
                .parse()
                .map_err(|_| ParseError)?,
            subject_areas: iter.map(|x| x.to_string()).collect()
        })
    }
}

#[allow(non_snake_case)]
struct RawProject {
    projectID: u32,
    proposer: String,
    multiplicity: u32,
    subject_area: String,
}

impl FromStr for RawProject {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<RawProject, ParseError> {
        let mut iter = s.split(" ");
        Ok(RawProject {
            projectID: iter.next()
                .ok_or(ParseError)?
                .parse()
                .map_err(|_| ParseError)?,
            proposer: iter.next()
                .ok_or(ParseError)?
                .to_string(),
            multiplicity: iter.next()
                .ok_or(ParseError)?
                .parse()
                .map_err(|_| ParseError)?,
            subject_area: iter.next()
                .ok_or(ParseError)?
                .to_string()
        })
    }
}

impl RawProject {
    pub fn process(self, staffs: &Vec<Rc<Staff>>) -> Result<Project, ParseError> {
        Ok(Project {
            projectID: self.projectID,
            proposer: staffs.iter()
                .find(|staff| self.proposer == staff.staffID)
                .ok_or(ParseError)?
                .clone(),
            subject_area: self.subject_area,
            multiplicity: self.multiplicity
        })
    }
}

#[allow(non_snake_case)]
struct RawStudent {
    studentID: String,
    choices: Vec<u32>
}

impl FromStr for RawStudent {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<RawStudent, ParseError> {
        let mut iter = s.split(" ");
        Ok(RawStudent { 
            studentID: iter.next()
                .ok_or(ParseError)?
                .to_string(),
            choices: iter.map(|x| x.parse::<u32>().ok())
                .collect::<Option<Vec<_>>>()
                .ok_or(ParseError)?
        })
    }
}

impl RawStudent {
    pub fn process(self, projects: &Vec<Rc<Project>>) -> Result<Student, ParseError> {
        Ok(Student {
            studentID: self.studentID,
            choices: self.choices.into_iter()
                .map(|x| projects.iter()
                    .find(|y| y.projectID == x)
                    .map(|x| x.clone()))
                .collect::<Option<Vec<_>>>()
                .ok_or(ParseError)?
        })
    }
}

impl Problem {
    pub fn from_files(staffs: &str, projects: &str, students: &str) -> Result<Self, ParseError> {
        let staffs = staffs.lines()
            .map(|x| x.parse::<Staff>().ok())
            .map(|x| x.map(Rc::new))
            .collect::<Option<Vec<_>>>()
            .ok_or(ParseError)?;

        let staffs_borrow = &staffs;
        let projects = projects.lines()
            .map(|x| x.parse::<RawProject>().ok())
            .map(|x: Option<RawProject>|
                x.and_then(|y| y.process(staffs_borrow).ok()))
            .map(|x| x.map(Rc::new))
            .collect::<Option<Vec<_>>>()
            .ok_or(ParseError)?;

        let projects_borrow = &projects;
        let students = students.lines()
            .map(|x| x.parse::<RawStudent>().ok())
            .map(|x|
                x.and_then(|y| y.process(projects_borrow).ok()))
            .map(|x| x.map(Rc::new))
            .collect::<Option<Vec<_>>>()
            .ok_or(ParseError)?;

        Ok(Problem {staffs, students, projects})
    }
}

pub struct IncompleteAllocation {
    pub student: Rc<Student>,
    pub project: Option<Rc<Project>>,
    pub supervisor: Option<Rc<Staff>>
}

pub struct Allocation {
    pub student: Rc<Student>,
    pub project: Rc<Project>,
    pub supervisor: Rc<Staff>
}

pub struct Solution {
    pub allocations: Vec<Allocation>
}

impl IncompleteAllocation {
    pub fn complete(self) -> Option<Allocation> {
        let Some(project) = self.project else {return None};
        let Some(supervisor) = self.supervisor else {return None};
        Some(Allocation { student: self.student, project, supervisor })
    }
}

impl Allocation {
    pub fn score(&self) -> u32 {
        let mut score = 0;

        match &self.student.choices[..] {
            [x, ..] if Rc::ptr_eq(x, &self.project) => score += 4,
            [_, x, ..] if Rc::ptr_eq(x, &self.project) => score += 3,
            [_, _, x, ..] if Rc::ptr_eq(x, &self.project) => score += 2,
            [_, _, _, x, ..] if Rc::ptr_eq(x, &self.project) => score += 1,
            _ => ()
        }

        if Rc::ptr_eq(&self.project.proposer, &self.supervisor) {score += 4}
        else if self.supervisor.subject_areas.contains(&self.project.subject_area) {score += 2}

        return score;
    }
}

impl Solution {
    pub fn score(&self) -> u32 {
        self.allocations.iter()
            .map(|x| x.score())
            .sum()
    }
}

impl Display for Allocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.student.studentID, self.project.projectID, self.supervisor.staffID)
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut x = self.allocations.iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>();
        x.sort();
        let output = x.into_iter()
            .map(|x| format!("{}\n", x))
            .collect::<String>();
        write!(f, "{}{}", output, self.score())
    }
}