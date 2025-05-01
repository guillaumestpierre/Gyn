use chrono::{Local, NaiveDate};
use crate::models::r#const::EXERCISE_LIST;

#[derive(Debug, Clone, PartialEq)]
pub struct Exercise {
    pub exid: u32,
    pub name: String,
    pub reps: Vec<(u32, f32)>,
    pub date: NaiveDate,
    pub starter: bool,
}

impl Exercise {
    pub fn new() -> Exercise{
        Exercise{exid: 0, name: String::new(), reps: vec![(0,0.0)], date: Local::now().naive_local().into(), starter: false}
    }
    pub fn build(exid: u32, name: String, reps: Vec<(u32, f32)>, date: NaiveDate, starter: bool) -> Result<Exercise, ()>{
        if EXERCISE_LIST.contains(&name.as_str()) {
            Ok(Exercise {exid, name, reps, date, starter})
        }
        else {
            Err(())
        }
    }
}