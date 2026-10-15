use diesel::prelude::*;

#[derive(Queryable, Selectable, Clone, Insertable, Debug)]
#[diesel(table_name = crate::database::schema::imported_graduates)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ImportedGraduate {
    pub id: String,
}

#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug)]
#[diesel(table_name = crate::database::schema::graduates)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Graduate {
    pub id: String,
    pub full_name: String,
}
#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug)]
#[diesel(table_name = crate::database::schema::positions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Position {
    pub id: i32,
    pub graduate_id: String,
    pub company_id: Option<i32>,
    pub company_name: Option<String>,
    pub headline: String,
    pub start_month: i32,
    pub end_month: Option<i32>,
    pub from_course_id: Option<i32>,
}

#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug)]
#[diesel(table_name = crate::database::schema::graduate_courses)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct GraduateCourse {
    pub id: i32,
    pub course_id: i32,
    pub graduate_id: String,
}

#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug)]
#[diesel(table_name = crate::database::schema::courses)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Course {
    pub id: i32,
    pub name: String,
}
#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug)]
#[diesel(table_name = crate::database::schema::course_known_positions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CourseKnownPosition {
    pub id: i32,
    pub course_id: i32,
    pub name: String,
}
