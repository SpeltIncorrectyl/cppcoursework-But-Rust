use std::collections::HashMap;
use std::rc::Rc;
use crate::representation::*;
use crate::rckey::RcKey;

pub struct SolutionError;

pub struct AllocatableProject {
    project: Rc<Project>,
    remaining: u32
}

pub struct AllocatableStaff {
    staff: Rc<Staff>,
    remaining: u32
}

pub fn solve(problem: Problem) -> Result<Solution, SolutionError> {
    let mut allocations = problem.students.iter()
        .map(|x| IncompleteAllocation {student: x.clone(), project: None, supervisor: None})
        .collect::<Vec<_>>();

    let mut allocatable_projects = problem.projects.iter()
        .map(|x| (RcKey::from(x), AllocatableProject {project: x.clone(), remaining: x.multiplicity}))
        .collect::<HashMap<RcKey<Project>, _>>();

    let mut allocatable_staff = problem.staffs.iter()
        .map(|x| (RcKey::from(x), AllocatableStaff {staff: x.clone(), remaining: x.load}))
        .collect::<HashMap<RcKey<Staff>, _>>();

    // Phase 1: allocate student's their choices
    for allocation in allocations.iter_mut() {
        for choice in allocation.student.choices.iter() {
            let allocatable_choice = allocatable_projects.get_mut(&RcKey::from(choice)).ok_or(SolutionError)?;
            if allocatable_choice.remaining >= 1 {
                allocatable_choice.remaining -= 1;
                allocation.project = Some(choice.clone());
                break;
            }
        }
    }

    // Bonus Phase 1: allocate whatever is left
    for allocation in allocations.iter_mut().filter(|x| x.project.is_none()) {
        for (_, choice) in allocatable_projects.iter_mut() {
            if choice.remaining >= 1 {
                choice.remaining -= 1;
                allocation.project = Some(choice.project.clone());
                break;
            }
        }
    }

    // Phase 2.1: allocate staff to projects they proposed
    for allocation in allocations.iter_mut() {
        let Some(proposer) = allocation.project.as_ref().map(|x| x.proposer.clone()) else {continue};
        let allocatable_staff = allocatable_staff.get_mut(&RcKey::from(&proposer)).ok_or(SolutionError)?;
        if allocatable_staff.remaining >= 1 {
            allocatable_staff.remaining -= 1;
            allocation.supervisor = Some(proposer);
            break;
        }
    }

    // Phase 2.2 allocate staff to something in their subject area
    for allocation in allocations.iter_mut().filter(|x| x.supervisor.is_none()) {
        for (_, allocatable_staff) in allocatable_staff.iter_mut() {
            let Some(project) = allocation.project.as_ref().map(|x| x.clone()) else {continue};
            if allocatable_staff.remaining >= 1 && allocatable_staff.staff.subject_areas.contains(&project.subject_area) {
                allocatable_staff.remaining -= 1;
                allocation.supervisor = Some(allocatable_staff.staff.clone());
                break;
            }
        }
    }

    // Phase 2.3 allocate whatever is left
    for allocation in allocations.iter_mut().filter(|x| x.project.is_some() && x.supervisor.is_none()) {
        for (_, allocatable_staff) in allocatable_staff.iter_mut() {
            if allocatable_staff.remaining >= 1 {
                allocatable_staff.remaining -= 1;
                allocation.supervisor = Some(allocatable_staff.staff.clone());
                break;
            }
        }
    }

    return Ok(Solution { allocations: allocations.into_iter().filter_map(|x| x.complete()).collect() });
}