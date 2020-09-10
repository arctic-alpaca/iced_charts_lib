use iced::canvas::Cursor;
use iced::widget::canvas::Text;
use iced::{Color, Point, Rectangle};

pub fn obsolete_cursor_is_over_inside_canvas(
    cursor: &Cursor,
    bounds: &Rectangle,
    bounds_to_search: &Rectangle,
) -> bool {
    let pos = cursor.position_in(&bounds);
    let y: Point;
    match pos {
        Some(x) => y = x,
        None => return false,
    }
    match cursor {
        Cursor::Available(position) => bounds_to_search.contains(y),
        Cursor::Unavailable => false,
    }
}

pub fn placeholder_get_text_width(text: &Text) -> f32 {
    30.0
}

pub fn placeholder_get_text_height(text: &Text) -> f32 {
    20.0
}

pub fn placeholder_get_max_text_width(strings: &[String]) -> f32 {
    let mut max_text_width = 0.0;
    for text in strings {
        let text_width = placeholder_get_text_width(&Text::from(&*text.as_str()));
        if text_width > max_text_width {
            max_text_width = text_width;
        }
    }
    max_text_width
}

pub fn placeholder_get_max_text_height(strings: &[String]) -> f32 {
    let mut max_text_height = 0.0;
    for text in strings {
        let text_height = placeholder_get_text_height(&Text::from(&*text.as_str()));
        if text_height > max_text_height {
            max_text_height = text_height;
        }
    }
    max_text_height
}
