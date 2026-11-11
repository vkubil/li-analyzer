use diesel::{RunQueryDsl, SqliteConnection};

use crate::database::{
    models::{Course, CourseKnownPosition},
    schema::{course_known_positions, courses},
};

pub fn add_courses(
    conn: &mut SqliteConnection,
    names: Vec<String>,
) -> Result<Vec<Course>, diesel::result::Error> {
    let values = names
        .iter()
        .map(|name| Course {
            id: 0,
            name: name.clone(),
        })
        .collect::<Vec<Course>>();

    diesel::insert_or_ignore_into(courses::table)
        .values(values)
        .returning((courses::id, courses::name))
        .load::<Course>(conn)
}

pub fn add_course_known_names(
    conn: &mut SqliteConnection,
    known_positions: Vec<(Course, Vec<String>)>,
) -> Result<usize, diesel::result::Error> {
    let values = known_positions
        .iter()
        .flat_map(|(course, known_positions)| {
            known_positions.iter().map(|position| CourseKnownPosition {
                id: 0,
                course_id: course.id,
                name: position.clone(),
            })
        })
        .collect::<Vec<CourseKnownPosition>>();

    diesel::insert_or_ignore_into(course_known_positions::table)
        .values(values)
        .execute(conn)
}
