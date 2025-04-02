use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct Training {
    id: i32,
    date: NaiveDate,
    duration: i32,
    exercices: Vec<Exercise>
}

#[derive(Debug, Clone)]
pub struct Exercise {
    id: i32,
    name: String,
    reps: Vec<(u32, u32)>,
}