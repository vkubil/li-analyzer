use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, column, row, scrollable, stack, table, text};
use iced::{Element, Length};
use iced_aw::badge;
use multimap::MultiMap;

use crate::database::connection::establish_connection;
use crate::database::models::{Course, CourseKnownPosition};
use crate::database::schema::{course_known_positions, courses};
use crate::ui::state::Message;
use crate::ui::style;

fn name_view<'a>(value: (Course, Vec<CourseKnownPosition>)) -> Element<'a, Message> {
    text(value.0.name.clone()).into()
}
fn pos_view<'a>(value: (Course, Vec<CourseKnownPosition>)) -> Element<'a, Message> {
    let position_badges = value
        .1
        .iter()
        .map(|pos| badge(text(pos.name.clone())).into())
        .collect::<Vec<_>>();

    stack(position_badges).into()
}

const ESTIMATED_COURSE_COUNT: usize = 32;

pub fn view<'a>() -> Result<Element<'a, Message>, anyhow::Error> {
    let conn = &mut establish_connection();
    let course_and_known_pos = courses::table
        .left_join(
            course_known_positions::table.on(course_known_positions::course_id.eq(courses::id)),
        )
        .load::<(Course, Option<CourseKnownPosition>)>(conn)?;

    // Mazdaug nuspeti, kad bus apie 32 kursu. Jei bus daugiau kursu, tai bus realokaciju, bet zymiai maziau ju
    let mut courses_merged =
        MultiMap::<Course, CourseKnownPosition>::with_capacity(ESTIMATED_COURSE_COUNT);

    for (course, known_pos) in course_and_known_pos {
        if let Some(pos) = known_pos {
            courses_merged.insert(course, pos);
        }
    }

    let courses_merged = courses_merged
        .iter_all()
        .map(|(course, known_positions)| (course.clone(), known_positions.clone()))
        .collect::<Vec<_>>();

    let view = column![
        row![
            button("Importuoti")
                .style(style::button::neutral::left)
                .on_press(Message::Import),
            button("Ištrinti visus")
                .style(style::button::danger::right)
                .on_press(Message::ClearImports),
        ]
        .align_y(Vertical::Center)
        .spacing(4)
        .width(Length::Fill),
        scrollable(
            table::table(
                vec![
                    table::column(text("Kursas"), name_view),
                    table::column(text("Žinoma pozicija"), pos_view)
                ],
                courses_merged
            )
            .width(Length::Fill)
        )
        .width(Length::Fill)
        .height(Length::Fill),
    ]
    .padding(16)
    .align_x(Horizontal::Center)
    .spacing(2)
    .width(Length::Fill)
    .height(Length::Fill)
    .into();

    Ok(view)
}
