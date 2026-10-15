diesel::table! {
    graduates (id) {
        id -> Text,
        full_name -> Text,
    }
}
diesel::table! {
    positions (id) {
        id -> Integer,
        graduate_id -> Text,
        company_id -> Nullable<Integer>,
        company_name -> Nullable<Text>,
        headline -> Text,
        start_month -> Integer,
        end_month -> Nullable<Integer>,
        from_course_id -> Nullable<Integer>,
    }
}
diesel::table! {
    graduate_courses (id) {
        id -> Integer,
        graduate_id -> Text,
        course_id -> Integer,
    }
}
diesel::table! {
    courses (id) {
        id -> Integer,
        name -> Text,
    }
}
diesel::table! {
    course_known_positions (id) {
        id -> Integer,
        course_id -> Integer,
        name -> Text,
    }
}
diesel::table! {
    imported_graduates (id) {
        id -> Text,
    }
}

diesel::joinable!(positions -> graduates (graduate_id));
diesel::joinable!(positions -> courses (from_course_id));

diesel::joinable!(graduate_courses -> graduates (graduate_id));
diesel::joinable!(graduate_courses -> courses (course_id));

diesel::joinable!(course_known_positions -> courses (course_id));

diesel::allow_tables_to_appear_in_same_query!(
    graduates,
    imported_graduates,
    courses,
    positions,
    graduate_courses,
    course_known_positions
);
