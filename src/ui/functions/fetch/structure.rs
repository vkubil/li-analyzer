use iced::widget::text_editor::Position;

use crate::database::models::{Graduate, GraduateCourse};

#[derive(Clone, Debug)]
pub struct GraduateInfo {
    #[allow(dead_code)]
    pub graduate: Graduate,

    #[allow(dead_code)]
    pub courses: Vec<GraduateCourse>,

    #[allow(dead_code)]
    pub positions: Vec<Position>,
}
