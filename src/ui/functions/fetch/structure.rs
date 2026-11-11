use iced::widget::text_editor::Position;

use crate::database::models::{Graduate, GraduateCourse};

#[derive(Clone, Debug)]
pub struct GraduateInfo {
    pub graduate: Graduate,
    pub courses: Vec<GraduateCourse>,
    pub positions: Vec<Position>,
}
