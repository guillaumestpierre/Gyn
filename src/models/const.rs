use crate::models::body_part::BodyPart;

pub const EXERCISE_LIST:[&'static str; 75] = [
    "Benchpress", "Inclined Benchpress", "Cable Lateral Raise", "Squat", "Deadlift", 
    "Overhead Press", "Barbell Row", "Pull Up", "Wide Grip Pull Up", "Close Grip Pull Up", "Chin Up", "Dip", "Push-Up", 
    "Cable Bicep Curl", "Cable Tricep Extension", "Leg Raise", "Plank", "Crunches", "Preacher Curl", "Hammer Preacher Curl",
    "Lunges", "Leg Press", "Leg Extension", "Standing Calf Raise", "Seated Calf Raise", "Face Pull", "Cable Row", "Lat Pulldown",
    "Tricep Pushdown", "Skullcrusher", "Cable Chest Fly", "Pec Deck", "Inclined Dumbbell Curl",
    "Cable Rear Delt Fly", "Cable Upright Row", "Cable Shrug", "Barbell Shrug", "Dumbbell Shrug",
    "Cable Front Raise", "Front Raise", "Dumbbell Lateral Raise", "Dumbbell Front Raise", "Dumbbell Rear Delt Fly", 
    "Dumbbell Upright Row", "Dumbbell Row", "Dumbbell Benchpress", "Dumbbell Inclined Benchpress", "Dumbbell Shoulder Press", 
    "Dumbbell Bicep Curl", "EZ Bar Biceps Curl", "Dumbbell Hammer Curl", "Dumbbell Tricep Extension",
    "Dumbbell Lunge", "Dumbbell Squat", "Dumbbell Deadlift", "Bar Shrug", "Machine Row", "Machine Chest Press",
    "Machine Shoulder Press", "Machine Bicep Curl", "Machine Tricep Extension", "T Bar Row", "Dumbbell Chest Fly", "Smith Benchpress",
    "Smith Inclined Benchpress", "Smith Overhead Press", "Smith Squat", "Smith Lunge", "Smith Deadlift", "Smith Shrug", 
    "Romanian Deadlift", "Bulgarian Split Squat", "Cable Crunch", "Standing Armstring Curl", "Seated Armstring Curl"
    ];

pub const CHEST_EXERCISES: [&'static str; 12] = [
    "Benchpress", "Inclined Benchpress", "Dumbbell Benchpress", "Dumbbell Inclined Benchpress", "Smith Inclined Benchpress",
    "Smith Benchpress", "Cable Chest Fly", "Machine Chest Press", "Dumbbell Chest Fly", "Dip", "Push-Up", "Pec Deck"
    ];

pub const BACK_EXERCISES: [&'static str; 22] = [
    "Deadlift", "Barbell Row", "Cable Row", "Lat Pulldown", "Pull Up", "Wide Grip Pull Up", "Close Grip Pull Up", "Chin Up",
    "Dumbbell Row", "Machine Row", "T Bar Row", "Face Pull", "Cable Rear Delt Fly", "Cable Shrug", "Dumbbell Rear Delt Fly",
    "Barbell Shrug", "Dumbbell Shrug", "Dumbbell Deadlift", "Bar Shrug", "Smith Deadlift", "Smith Shrug", "Romanian Deadlift"
    ];

pub const LEG_EXERCISES: [&'static str; 13] = [
    "Squat", "Lunges", "Leg Press", "Leg Extension", "Standing Calf Raise", "Seated Calf Raise", "Dumbbell Lunge",
    "Dumbbell Squat", "Smith Squat", "Smith Lunge", "Bulgarian Split Squat", "Standing Armstring Curl", "Seated Armstring Curl"
    ];

pub const ARM_EXERCISES: [&'static str; 13] = [
    "Cable Bicep Curl", "Cable Tricep Extension", "Preacher Curl", "Tricep Pushdown", "Skullcrusher",
    "Dumbbell Bicep Curl", "EZ Bar Biceps Curl", "Dumbbell Hammer Curl", "Dumbbell Tricep Extension", 
    "Hammer Preacher Curl", "Inclined Dumbbell Curl", "Machine Bicep Curl", "Machine Tricep Extension"
    ];

pub const SHOULDER_EXERCISES: [&'static str; 12] = [
    "Overhead Press", "Dumbbell Shoulder Press", "Smith Overhead Press", "Cable Lateral Raise", "Cable Upright Row", "Cable Front Raise",
    "Dumbbell Lateral Raise", "Dumbbell Upright Row", "Front Raise", "Dumbbell Front Raise", "Dumbbell Rear Delt Fly", "Machine Shoulder Press"
    ];

pub const CORE_EXERCISES: [&'static str; 4] = [
    "Leg Raise", "Plank", "Crunches", "Cable Crunch"
    ];


pub const BODY_PARTS: &[BodyPart] = &[
    BodyPart { name: "Chest", exercises: &CHEST_EXERCISES },
    BodyPart { name: "Back", exercises: &BACK_EXERCISES },
    BodyPart { name: "Legs", exercises: &LEG_EXERCISES },
    BodyPart { name: "Arms", exercises: &ARM_EXERCISES },
    BodyPart { name: "Shoulders", exercises: &SHOULDER_EXERCISES },
    BodyPart { name: "Core", exercises: &CORE_EXERCISES },
];

pub const TABLE_EXERCISES: &str = "exercises";

pub const TABLE_REPS: &str = "reps";

pub const TABLE_WEIGHT: &str = "weight";
