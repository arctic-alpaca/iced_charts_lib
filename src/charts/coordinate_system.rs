use crate::charts::axis_enums::{AxisOrientation, XAxisOrientation, YAxisOrientation};
use crate::charts::bar_chart_style_info_iced::BarChartStyleInfoIced;
use crate::charts::error::{ChartsLibError, ErrorKind};
use crate::charts::util::{
    placeholder_get_max_text_height, placeholder_get_max_text_width, placeholder_get_text_height,
    placeholder_get_text_width,
};
use iced::canvas::{Frame, LineCap, LineJoin, Path, Stroke, Text};
use iced::{Color, Point, Size, Space};
use std::borrow::Borrow;

//TODO: make presets nicer
const TEXT_PADDING: f32 = 5.0;
const HEADLINE_COLOR: Color = Color::BLACK;
const SEPARATOR_STROKE_WIDTH: f32 = 2.0;
const SEPARATOR_STROKE_COLOR: Color = Color::from_rgb(175.0 / 255.0, 181.0 / 255.0, 189.0 / 255.0);
const AXIS_STROKE_COLOR: Color = Color::BLACK;
const AXIS_STROKE_WIDTH: f32 = 2.0;
const X_TEXT_COLOR: Color = Color::BLACK;
const Y_TEXT_COLOR: Color = Color::BLACK;
const TEXT_SIZE: f32 = 16.0;

//TODO: make internal only, config through chart specific configs
#[derive(Debug, Clone)]
pub struct CoordinateSystemConfig {
    pub draw_cluster_label_on_x_axis: Option<XAxisOrientation>,
    pub draw_cluster_label_on_y_axis: Option<YAxisOrientation>,
    pub draw_marking_label_on_x_axis: Option<XAxisOrientation>,
    pub draw_marking_label_on_y_axis: Option<YAxisOrientation>,
    pub x_text: Option<Vec<String>>,
    pub x_text_color: Color,
    pub x_text_size: f32,
    pub y_text: Option<Vec<String>>,
    pub y_text_color: Color,
    pub y_text_size: f32,
    pub x_marking_amount: Option<usize>,
    pub y_marking_amount: Option<usize>,
    pub headline: Option<String>,
    pub headline_color: Color,
    pub separator_stroke_width: f32,
    pub separator_stroke_color: Color,
    pub axis_stroke_color: Color,
    pub axis_stroke_width: f32,
    pub legend_alignment: Option<CoordinateSystemLegendAlignment>,
    pub text_padding: f32,
    pub x_min_value: f32,
    pub x_max_value: f32,
    pub y_min_value: f32,
    pub y_max_value: f32,
    pub headline_size: f32,
}

impl CoordinateSystemConfig {
    //TODO: add complete config

    pub fn new() -> Self {
        CoordinateSystemConfig {
            draw_cluster_label_on_x_axis: None,
            draw_cluster_label_on_y_axis: None,
            draw_marking_label_on_x_axis: None,
            draw_marking_label_on_y_axis: None,
            x_text: None,
            x_text_color: X_TEXT_COLOR,
            x_text_size: TEXT_SIZE,
            y_text: None,
            y_text_color: Y_TEXT_COLOR,
            y_text_size: TEXT_SIZE,
            x_marking_amount: None,
            y_marking_amount: None,
            headline: None,
            headline_color: HEADLINE_COLOR,
            separator_stroke_width: SEPARATOR_STROKE_WIDTH,
            separator_stroke_color: SEPARATOR_STROKE_COLOR,
            axis_stroke_color: AXIS_STROKE_COLOR,
            axis_stroke_width: AXIS_STROKE_WIDTH,
            legend_alignment: None,
            text_padding: TEXT_PADDING,
            x_min_value: 0.0,
            x_max_value: 0.0,
            y_min_value: 0.0,
            y_max_value: 0.0,
            headline_size: TEXT_SIZE,
        }
    }
}
#[derive(Debug)]
pub struct AxisAndOrigin {
    pub x_axis_positive_length: f32,
    pub x_axis_negative_length: f32,
    pub y_axis_positive_length: f32,
    pub y_axis_negative_length: f32,
    pub origin: Point,
}

impl AxisAndOrigin {
    pub fn new(input: (f32, f32, f32, f32, Point)) -> Self {
        AxisAndOrigin {
            x_axis_positive_length: input.0,
            x_axis_negative_length: input.1,
            y_axis_positive_length: input.2,
            y_axis_negative_length: input.3,
            origin: input.4,
        }
    }
    pub fn get_axis_and_origin(&self) -> (f32, f32, f32, f32, Point) {
        (
            self.x_axis_positive_length,
            self.x_axis_negative_length,
            self.y_axis_positive_length,
            self.y_axis_negative_length,
            self.origin,
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CoordinateSystemLegendAlignment {
    Top,
    Left,
    Right,
    Bottom,
}

#[derive(Debug, Clone, Copy)]
pub struct SpaceCalculator {
    pub space_top: f32,
    pub space_bottom: f32,
    pub space_left: f32,
    pub space_right: f32,
}

impl SpaceCalculator {
    pub fn new() -> Self {
        SpaceCalculator {
            space_top: 0.0,
            space_bottom: 0.0,
            space_left: 0.0,
            space_right: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DrawingAxis {
    X,
    Y,
}

#[derive(Debug, Clone)]
pub struct CoordinateSystem {
    pub x_axis_orientation: XAxisOrientation,
    pub y_axis_orientation: YAxisOrientation,
    pub coordinate_system_config: CoordinateSystemConfig,
}

impl CoordinateSystem {
    pub fn new(
        x_axis_orientation: XAxisOrientation,
        y_axis_orientation: YAxisOrientation,
        coordinate_system_config: CoordinateSystemConfig,
    ) -> CoordinateSystem {
        CoordinateSystem {
            x_axis_orientation,
            y_axis_orientation,
            coordinate_system_config,
        }
    }

    /// returns (x_axis_positive_length, x_axis_negative_length, y_axis_positive_length, y_axis_negative_length, origin)
    pub fn calculate_axis_length_and_origin(&self, size: Size) -> (f32, f32, f32, f32, Point) {
        let space_calculator = self.calculate_spacing(size);
        let padding_top = space_calculator.space_top;
        let padding_bottom = space_calculator.space_bottom;
        let padding_left = space_calculator.space_left;
        let padding_right = space_calculator.space_right;
        let width_canvas = size.width;
        let height_canvas = size.height;
        let coordinate_box_width = width_canvas - padding_left - padding_right;
        let coordinate_box_height = height_canvas - padding_bottom - padding_top;
        let top_left = Point::new(padding_left, padding_top);
        let top_right = Point::new(width_canvas - padding_right, padding_top);
        let bot_left = Point::new(padding_left, height_canvas - padding_bottom);
        let bot_right = Point::new(width_canvas - padding_right, height_canvas - padding_bottom);
        let middle_left = Point::new(padding_left, coordinate_box_height / 2.0 + padding_top);
        let middle_right = Point::new(
            width_canvas - padding_right,
            coordinate_box_height / 2.0 + padding_top,
        );
        let middle_top = Point::new(coordinate_box_width / 2.0 + padding_left, padding_top);
        let middle_bottom = Point::new(
            coordinate_box_width / 2.0 + padding_left,
            height_canvas - padding_bottom,
        );
        let center = Point::new(
            coordinate_box_width / 2.0 + padding_left,
            coordinate_box_height / 2.0 + padding_top,
        );

        let mut x_axis_positive_length = 0.0;
        let mut x_axis_negative_length = 0.0;
        let mut y_axis_positive_length = 0.0;
        let mut y_axis_negative_length = 0.0;
        let origin: Point;

        match (self.x_axis_orientation, self.y_axis_orientation) {
            (XAxisOrientation::Positive, YAxisOrientation::Positive) => {
                x_axis_positive_length = coordinate_box_width;
                y_axis_positive_length = coordinate_box_height;
                origin = bot_left;
            }
            (XAxisOrientation::Positive, YAxisOrientation::Negative) => {
                x_axis_positive_length = coordinate_box_width;
                y_axis_negative_length = coordinate_box_height;
                origin = top_left;
            }
            (XAxisOrientation::Positive, YAxisOrientation::Complete) => {
                x_axis_positive_length = coordinate_box_width;
                y_axis_positive_length = coordinate_box_height / 2.0;
                y_axis_negative_length = coordinate_box_height / 2.0;
                origin = middle_left;
            }
            (XAxisOrientation::Negative, YAxisOrientation::Positive) => {
                x_axis_negative_length = coordinate_box_width;
                y_axis_positive_length = coordinate_box_height;
                origin = bot_right;
            }
            (XAxisOrientation::Negative, YAxisOrientation::Negative) => {
                x_axis_negative_length = coordinate_box_width;
                y_axis_negative_length = coordinate_box_height;
                origin = top_right;
            }
            (XAxisOrientation::Negative, YAxisOrientation::Complete) => {
                x_axis_negative_length = coordinate_box_width;
                y_axis_positive_length = coordinate_box_height / 2.0;
                y_axis_negative_length = coordinate_box_height / 2.0;
                origin = middle_right;
            }

            (XAxisOrientation::Complete, YAxisOrientation::Positive) => {
                x_axis_positive_length = coordinate_box_width / 2.0;
                x_axis_negative_length = coordinate_box_width / 2.0;
                y_axis_positive_length = coordinate_box_height;
                origin = middle_bottom;
            }
            (XAxisOrientation::Complete, YAxisOrientation::Negative) => {
                x_axis_positive_length = coordinate_box_width / 2.0;
                x_axis_negative_length = coordinate_box_width / 2.0;
                y_axis_negative_length = coordinate_box_height;
                origin = middle_top;
            }
            (XAxisOrientation::Complete, YAxisOrientation::Complete) => {
                x_axis_positive_length = coordinate_box_width / 2.0;
                x_axis_negative_length = coordinate_box_width / 2.0;
                y_axis_positive_length = coordinate_box_height / 2.0;
                y_axis_negative_length = coordinate_box_height / 2.0;
                origin = center;
            }
        };
        (
            x_axis_positive_length,
            x_axis_negative_length,
            y_axis_positive_length,
            y_axis_negative_length,
            origin,
        )
    }

    fn calculate_axis_labels_space(&self, space_calculator: &mut SpaceCalculator) {
        let mut x_max_text_width = 0.0;
        let mut x_max_text_height = 0.0;
        let mut y_max_text_width = 0.0;
        let mut y_max_text_height = 0.0;
        if let Some(x_marking_orientation) =
            self.coordinate_system_config.draw_marking_label_on_x_axis
        {
            if let Some(x_markings_text) = self.coordinate_system_config.x_text.as_ref() {
                x_max_text_height = placeholder_get_max_text_height(x_markings_text);
                x_max_text_width = placeholder_get_max_text_width(x_markings_text);
            } else {
                let x_max_value = self.coordinate_system_config.x_max_value;
                let x_min_value = self.coordinate_system_config.x_min_value;
                let x_max_value_text_height =
                    placeholder_get_text_height(&Text::from(x_max_value.to_string().as_str()));
                let x_min_value_text_height =
                    placeholder_get_text_height(&Text::from(x_min_value.to_string().as_str()));
                let x_max_value_text_width =
                    placeholder_get_text_width(&Text::from(x_max_value.to_string().as_str()));
                let x_min_value_text_width =
                    placeholder_get_text_width(&Text::from(x_min_value.to_string().as_str()));
                x_max_text_height = if x_max_value_text_height > x_min_value_text_height {
                    x_max_text_height
                } else {
                    x_min_value_text_height
                };
                x_max_text_width = if x_max_value_text_width > x_min_value_text_width {
                    x_max_text_width
                } else {
                    x_min_value_text_width
                };
            }
        }
        if let Some(y_marking_orientation) =
            self.coordinate_system_config.draw_marking_label_on_y_axis
        {
            if let Some(y_markings_text) = self.coordinate_system_config.y_text.as_ref() {
                y_max_text_height = placeholder_get_max_text_height(y_markings_text);
                y_max_text_width = placeholder_get_max_text_width(y_markings_text);
            } else {
                let y_max_value = self.coordinate_system_config.y_max_value;
                let y_min_value = self.coordinate_system_config.y_min_value;
                let y_max_value_text_height =
                    placeholder_get_text_height(&Text::from(y_max_value.to_string().as_str()));
                let y_min_value_text_height =
                    placeholder_get_text_height(&Text::from(y_min_value.to_string().as_str()));
                let y_max_value_text_width =
                    placeholder_get_text_width(&Text::from(y_max_value.to_string().as_str()));
                let y_min_value_text_width =
                    placeholder_get_text_width(&Text::from(y_min_value.to_string().as_str()));
                y_max_text_height = if y_max_value_text_height > y_min_value_text_height {
                    y_max_text_height
                } else {
                    y_min_value_text_height
                };
                y_max_text_width = if y_max_value_text_width > y_min_value_text_width {
                    y_max_text_width
                } else {
                    y_min_value_text_width
                };
            }
        }
        match self.y_axis_orientation {
            YAxisOrientation::Positive => {
                space_calculator.space_bottom += x_max_text_height;
                space_calculator.space_left += x_max_text_width / 2.0;
                space_calculator.space_right += x_max_text_width / 2.0;
            }
            YAxisOrientation::Negative => {
                space_calculator.space_top += x_max_text_height;
                space_calculator.space_left += x_max_text_width / 2.0;
                space_calculator.space_right += x_max_text_width / 2.0;
            }
            YAxisOrientation::Complete => {
                space_calculator.space_bottom += x_max_text_height;
                space_calculator.space_left += x_max_text_width / 2.0;
                space_calculator.space_right += x_max_text_width / 2.0;
            }
        }
        match self.x_axis_orientation {
            XAxisOrientation::Positive => {
                space_calculator.space_left += y_max_text_width;
                space_calculator.space_top += y_max_text_height / 2.0;
                space_calculator.space_bottom += y_max_text_height / 2.0;
            }
            XAxisOrientation::Negative => {
                space_calculator.space_right += y_max_text_width;
                space_calculator.space_top += y_max_text_height / 2.0;
                space_calculator.space_bottom += y_max_text_height / 2.0;
            }
            XAxisOrientation::Complete => {
                space_calculator.space_left += y_max_text_width;
                space_calculator.space_top += y_max_text_height / 2.0;
                space_calculator.space_bottom += y_max_text_height / 2.0;
            }
        }
    }

    fn calculate_cluster_labels_space(&self, space_calculator: &mut SpaceCalculator) {
        //let x_max_text_width = 0.0;
        let mut x_max_text_height = 0.0;
        let mut y_max_text_width = 0.0;
        //let y_max_text_height = 0.0;
        if let Some(x_cluster_orientation) =
            self.coordinate_system_config.draw_cluster_label_on_x_axis
        {
            let x_clusters_text = self.coordinate_system_config.x_text.as_ref().unwrap();
            //x_max_text_width = self.get_max_text_width(x_clusters_text);
            x_max_text_height = placeholder_get_max_text_height(x_clusters_text);
        }
        if let Some(y_cluster_orientation) =
            self.coordinate_system_config.draw_cluster_label_on_y_axis
        {
            let y_clusters_text = self.coordinate_system_config.y_text.as_ref().unwrap();
            y_max_text_width = placeholder_get_max_text_width(y_clusters_text);
            //y_max_text_height = self.get_max_text_height(y_clusters_text);
        }
        match self.y_axis_orientation {
            YAxisOrientation::Positive => space_calculator.space_bottom += x_max_text_height,
            YAxisOrientation::Negative => space_calculator.space_top += x_max_text_height,
            YAxisOrientation::Complete => space_calculator.space_bottom += x_max_text_height,
        }
        match self.x_axis_orientation {
            XAxisOrientation::Positive => space_calculator.space_left += y_max_text_width,
            XAxisOrientation::Negative => space_calculator.space_right += y_max_text_width,
            XAxisOrientation::Complete => space_calculator.space_left += y_max_text_width,
        }
    }

    fn calculate_headline_space(&self, space_calculator: &mut SpaceCalculator) {
        if let Some(headline_text) = &self.coordinate_system_config.headline {
            space_calculator.space_top +=
                placeholder_get_text_height(&Text::from(&*headline_text.as_str()));
        }
    }
    // TODO create legends stuff
    fn calculate_legend_space(&self, space_calculator: &mut SpaceCalculator) {}

    fn calculate_spacing(&self, size: Size) -> SpaceCalculator {
        let mut space_calculator = SpaceCalculator::new();
        self.calculate_axis_labels_space(&mut space_calculator);
        self.calculate_cluster_labels_space(&mut space_calculator);
        self.calculate_headline_space(&mut space_calculator);
        self.calculate_legend_space(&mut space_calculator);
        space_calculator
    }

    // Helper functions:
    fn calculate_separator_line_parallel_to_y(
        &self,
        current_position: &Point,
        axis_and_origin: &AxisAndOrigin,
    ) -> (Point, Point) {
        let (
            x_axis_positive_length,
            x_axis_negative_length,
            y_axis_positive_length,
            y_axis_negative_length,
            origin,
        ) = axis_and_origin.get_axis_and_origin();
        match self.y_axis_orientation {
            YAxisOrientation::Positive => (
                Point::new(current_position.x, current_position.y),
                Point::new(
                    current_position.x,
                    current_position.y - y_axis_positive_length,
                ),
            ),
            YAxisOrientation::Negative => (
                Point::new(current_position.x, current_position.y),
                Point::new(
                    current_position.x,
                    current_position.y + y_axis_negative_length,
                ),
            ),
            YAxisOrientation::Complete => (
                Point::new(
                    current_position.x,
                    current_position.y - y_axis_positive_length,
                ),
                Point::new(
                    current_position.x,
                    current_position.y + y_axis_positive_length,
                ),
            ),
        }
    }

    fn calculate_separator_line_parallel_to_x(
        &self,
        current_position: Point,
        axis_and_origin: &AxisAndOrigin,
    ) -> (Point, Point) {
        let (
            x_axis_positive_length,
            x_axis_negative_length,
            y_axis_positive_length,
            y_axis_negative_length,
            origin,
        ) = axis_and_origin.get_axis_and_origin();
        match self.x_axis_orientation {
            XAxisOrientation::Positive => (
                Point::new(current_position.x, current_position.y),
                Point::new(
                    current_position.x + x_axis_positive_length,
                    current_position.y,
                ),
            ),
            XAxisOrientation::Negative => (
                Point::new(current_position.x, current_position.y),
                Point::new(
                    current_position.x - x_axis_negative_length,
                    current_position.y,
                ),
            ),
            XAxisOrientation::Complete => (
                Point::new(
                    current_position.x - x_axis_negative_length,
                    current_position.y,
                ),
                Point::new(
                    current_position.x + x_axis_positive_length,
                    current_position.y,
                ),
            ),
        }
    }

    fn create_axis_markings_texts_vector(
        &self,
        start: f32,
        end: f32,
        markings_amount: usize,
        negative: bool,
    ) -> Vec<String> {
        if markings_amount < 1 {
            return vec![];
        }
        //calculate the step size and the text for it:
        let mut markings_text_vec: Vec<String> = Vec::with_capacity(markings_amount);
        let space_between_start_end = (start - end).abs();
        let step = space_between_start_end / (markings_amount as f32 - 1.0);
        let mut current = start;
        while current <= end.abs() + step / 2.0 {
            let mut tmp = current * 100.0;
            tmp = tmp.round();
            tmp /= 100.0;
            if negative {
                let mut inserter = tmp.to_string();
                inserter.insert(0, '-');
                markings_text_vec.push(inserter);
            } else {
                markings_text_vec.push(tmp.to_string());
            }
            current += step;
        }
        markings_text_vec
    }

    // Drawing functions:
    pub fn draw_headline_and_labels(&self, axis_and_origin: &AxisAndOrigin, frame: &mut Frame) {
        self.draw_headline(frame);
        self.draw_axis_makings_labels(axis_and_origin, frame);
        self.draw_cluster_labels(axis_and_origin, frame);
    }

    pub fn draw_axis(&self, axis_and_origin: &AxisAndOrigin, frame: &mut Frame) {
        let (
            x_axis_positive_length,
            x_axis_negative_length,
            y_axis_positive_length,
            y_axis_negative_length,
            origin,
        ) = axis_and_origin.get_axis_and_origin();

        let x_positive_end = Point::new(origin.x + x_axis_positive_length, origin.y);
        let x_negative_end = Point::new(origin.x - x_axis_negative_length, origin.y);
        let y_positive_end = Point::new(origin.x, origin.y - y_axis_positive_length);
        let y_negative_end = Point::new(origin.x, origin.y + y_axis_negative_length);

        let stroke_width = self.coordinate_system_config.axis_stroke_width;

        self.draw_axis_line(origin, x_positive_end, frame);
        self.draw_axis_line(origin, x_negative_end, frame);
        self.draw_axis_line(origin, y_positive_end, frame);
        self.draw_axis_line(origin, y_negative_end, frame);
        let origin_square_top_left =
            Point::new(origin.x - stroke_width / 2.0, origin.y - stroke_width / 2.0);
        frame.fill_rectangle(
            origin_square_top_left,
            Size::new(stroke_width, stroke_width),
            self.coordinate_system_config.axis_stroke_color,
        );
    }

    fn draw_cluster_labels(&self, axis_and_origin: &AxisAndOrigin, frame: &mut Frame) {
        self.draw_x_cluster_labels(&axis_and_origin, frame);
        self.draw_y_cluster_labels(&axis_and_origin, frame);
    }

    fn draw_axis_makings_labels(&self, axis_and_origin: &AxisAndOrigin, frame: &mut Frame) {
        if self
            .coordinate_system_config
            .draw_marking_label_on_x_axis
            .is_some()
        {
            self.draw_x_axis_markings_labels(axis_and_origin, frame);
        }
        if self
            .coordinate_system_config
            .draw_marking_label_on_y_axis
            .is_some()
        {
            self.draw_y_axis_markings_labels(axis_and_origin, frame);
        }
    }

    fn draw_x_cluster_labels(&self, axis_and_origin: &AxisAndOrigin, frame: &mut Frame) {
        let (
            x_axis_positive_length,
            x_axis_negative_length,
            y_axis_positive_length,
            y_axis_negative_length,
            origin,
        ) = axis_and_origin.get_axis_and_origin();

        if let Some(x_cluster_orientation) =
            self.coordinate_system_config.draw_cluster_label_on_x_axis
        {
            if let Some(x_clusters_texts) = self.coordinate_system_config.x_text.clone() {
                let amount = x_clusters_texts.len() as f32;
                let (mut current_position, step) = match self.x_axis_orientation {
                    XAxisOrientation::Positive => (origin, x_axis_positive_length / amount),
                    XAxisOrientation::Negative => (
                        Point::new(origin.x - x_axis_negative_length, origin.y),
                        x_axis_negative_length / amount,
                    ),
                    XAxisOrientation::Complete => (
                        Point::new(origin.x - x_axis_negative_length, origin.y),
                        (x_axis_positive_length + x_axis_negative_length) / amount,
                    ),
                };
                let (mut separator_line_start, mut separator_line_end) =
                    self.calculate_separator_line_parallel_to_y(&current_position, axis_and_origin);
                if self.y_axis_orientation == YAxisOrientation::Negative {
                    current_position.y -= placeholder_get_max_text_height(&*x_clusters_texts);
                }
                if self.y_axis_orientation == YAxisOrientation::Complete {
                    current_position.y += y_axis_negative_length;
                }
                for text in x_clusters_texts {
                    let text = Text::from(&*text.as_str());
                    let text_width = placeholder_get_text_width(&text);

                    self.draw_text_at_point(
                        text,
                        current_position.x + step / 2.0 - text_width / 2.0,
                        current_position.y,
                        self.coordinate_system_config.x_text_color,
                        self.coordinate_system_config.x_text_size,
                        frame,
                    );

                    self.draw_separator_line(separator_line_start, separator_line_end, frame);

                    separator_line_start.x += step;
                    separator_line_end.x += step;

                    current_position.x += step;
                }
                self.draw_separator_line(separator_line_start, separator_line_end, frame);
            }
        }
    }

    fn draw_y_cluster_labels(&self, axis_and_origin: &AxisAndOrigin, frame: &mut Frame) {
        let (
            x_axis_positive_length,
            x_axis_negative_length,
            y_axis_positive_length,
            y_axis_negative_length,
            origin,
        ) = axis_and_origin.get_axis_and_origin();
        if let Some(y_cluster_orientation) =
            self.coordinate_system_config.draw_cluster_label_on_y_axis
        {
            if let Some(y_clusters_texts) = self.coordinate_system_config.y_text.clone() {
                let amount = y_clusters_texts.len() as f32;
                let (mut current_position, step) = match self.y_axis_orientation {
                    YAxisOrientation::Positive => (origin, y_axis_positive_length / amount),
                    YAxisOrientation::Negative => (
                        Point::new(origin.x, origin.y + y_axis_negative_length),
                        y_axis_negative_length / amount,
                    ),
                    YAxisOrientation::Complete => (
                        Point::new(origin.x, origin.y + y_axis_negative_length),
                        (y_axis_positive_length + y_axis_negative_length) / amount,
                    ),
                };
                let (mut separator_line_start, mut separator_line_end) =
                    self.calculate_separator_line_parallel_to_x(current_position, axis_and_origin);
                if self.x_axis_orientation == XAxisOrientation::Positive {
                    current_position.x -= placeholder_get_max_text_width(&*y_clusters_texts);
                }
                if self.x_axis_orientation == XAxisOrientation::Complete {
                    current_position.x -= placeholder_get_max_text_width(&*y_clusters_texts);
                    current_position.x -= x_axis_negative_length;
                }
                for text in y_clusters_texts {
                    let text = Text::from(&*text.as_str());
                    let text_height = placeholder_get_text_height(&text);

                    self.draw_text_at_point(
                        text,
                        current_position.x,
                        current_position.y - step / 2.0 - text_height / 2.0,
                        self.coordinate_system_config.y_text_color,
                        self.coordinate_system_config.y_text_size,
                        frame,
                    );

                    self.draw_separator_line(separator_line_start, separator_line_end, frame);

                    separator_line_start.y -= step;
                    separator_line_end.y -= step;

                    current_position.y -= step;
                }
                self.draw_separator_line(separator_line_start, separator_line_end, frame);
            }
        }
    }

    fn draw_headline(&self, frame: &mut Frame) {
        if let Some(headline_text) = &self.coordinate_system_config.headline {
            let text = Text::from(&*headline_text.as_str());
            let headline_length = placeholder_get_text_width(&text);
            let headline_offset = headline_length / 2.0;

            self.draw_text_at_point(
                text,
                frame.width() / 2.0 - headline_offset,
                0.0,
                self.coordinate_system_config.headline_color,
                self.coordinate_system_config.headline_size,
                frame,
            );
        }
    }

    fn draw_x_axis_markings_labels(&self, axis_and_origin: &AxisAndOrigin, frame: &mut Frame) {
        if let Some(markings_texts) = self.coordinate_system_config.x_text.as_ref() {
            match self.x_axis_orientation {
                XAxisOrientation::Positive => {
                    self.draw_x_axis_positive_marking_labels(
                        markings_texts.clone(),
                        axis_and_origin,
                        frame,
                    );
                }
                XAxisOrientation::Negative => {
                    self.draw_x_axis_negative_marking_labels(
                        markings_texts.clone(),
                        axis_and_origin,
                        frame,
                    );
                }
                XAxisOrientation::Complete => {
                    self.draw_x_axis_positive_marking_labels(
                        markings_texts.clone(),
                        axis_and_origin,
                        frame,
                    );
                    self.draw_x_axis_negative_marking_labels(
                        markings_texts.clone(),
                        axis_and_origin,
                        frame,
                    );
                }
            };
        } else if let Some(markings_amount) = self.coordinate_system_config.x_marking_amount {
            match self.x_axis_orientation {
                XAxisOrientation::Positive => {
                    self.draw_x_axis_positive_marking_labels(
                        self.create_axis_markings_texts_vector(
                            0.0,
                            self.coordinate_system_config.x_max_value,
                            markings_amount,
                            false,
                        ),
                        axis_and_origin,
                        frame,
                    );
                }
                XAxisOrientation::Negative => {
                    self.draw_x_axis_negative_marking_labels(
                        self.create_axis_markings_texts_vector(
                            0.0,
                            self.coordinate_system_config.x_min_value,
                            markings_amount,
                            true,
                        ),
                        axis_and_origin,
                        frame,
                    );
                }
                XAxisOrientation::Complete => {
                    self.draw_x_axis_positive_marking_labels(
                        self.create_axis_markings_texts_vector(
                            0.0,
                            self.coordinate_system_config.x_max_value,
                            markings_amount,
                            false,
                        ),
                        axis_and_origin,
                        frame,
                    );
                    self.draw_x_axis_negative_marking_labels(
                        self.create_axis_markings_texts_vector(
                            0.0,
                            self.coordinate_system_config.x_min_value,
                            markings_amount,
                            true,
                        ),
                        axis_and_origin,
                        frame,
                    );
                }
            };
        } else {
            panic!(
                "coordinate_system_config.y_text or coordinate_system_config.y_marking_amount set"
            );
        }
    }

    fn draw_y_axis_markings_labels(&self, axis_and_origin: &AxisAndOrigin, frame: &mut Frame) {
        if let Some(markings_texts) = self.coordinate_system_config.y_text.as_ref() {
            match self.y_axis_orientation {
                YAxisOrientation::Positive => {
                    self.draw_y_axis_positive_marking_labels(
                        markings_texts.clone(),
                        axis_and_origin,
                        frame,
                    );
                }
                YAxisOrientation::Negative => {
                    self.draw_y_axis_negative_marking_labels(
                        markings_texts.clone(),
                        axis_and_origin,
                        frame,
                    );
                }
                YAxisOrientation::Complete => {
                    self.draw_y_axis_positive_marking_labels(
                        markings_texts.clone(),
                        axis_and_origin,
                        frame,
                    );
                    self.draw_y_axis_negative_marking_labels(
                        markings_texts.clone(),
                        axis_and_origin,
                        frame,
                    );
                }
            };
        } else if let Some(markings_amount) = self.coordinate_system_config.y_marking_amount {
            match self.y_axis_orientation {
                YAxisOrientation::Positive => {
                    self.draw_y_axis_positive_marking_labels(
                        self.create_axis_markings_texts_vector(
                            0.0,
                            self.coordinate_system_config.y_max_value,
                            markings_amount,
                            false,
                        ),
                        axis_and_origin,
                        frame,
                    );
                }
                YAxisOrientation::Negative => {
                    self.draw_y_axis_negative_marking_labels(
                        self.create_axis_markings_texts_vector(
                            0.0,
                            self.coordinate_system_config.y_min_value,
                            markings_amount,
                            true,
                        ),
                        axis_and_origin,
                        frame,
                    );
                }
                YAxisOrientation::Complete => {
                    self.draw_y_axis_positive_marking_labels(
                        self.create_axis_markings_texts_vector(
                            0.0,
                            self.coordinate_system_config.y_max_value,
                            markings_amount,
                            false,
                        ),
                        axis_and_origin,
                        frame,
                    );
                    self.draw_y_axis_negative_marking_labels(
                        self.create_axis_markings_texts_vector(
                            0.0,
                            self.coordinate_system_config.y_min_value,
                            markings_amount,
                            true,
                        ),
                        axis_and_origin,
                        frame,
                    );
                }
            };
        } else {
            panic!(
                "coordinate_system_config.y_text or coordinate_system_config.y_marking_amount set"
            );
        }
    }

    fn draw_x_axis_positive_marking_labels(
        &self,
        markings_texts: Vec<String>,
        axis_and_origin: &AxisAndOrigin,
        frame: &mut Frame,
    ) {
        let (
            x_axis_positive_length,
            x_axis_negative_length,
            y_axis_positive_length,
            y_axis_negative_length,
            origin,
        ) = axis_and_origin.get_axis_and_origin();

        let markings_amount = if markings_texts.len() <= 1 {
            return;
        } else {
            markings_texts.len() as f32 - 1.0
        };

        let mut current_position = origin;
        let step = x_axis_positive_length / markings_amount;

        let (mut separator_line_start, mut separator_line_end) =
            self.calculate_separator_line_parallel_to_y(&current_position, &axis_and_origin);
        if self.y_axis_orientation == YAxisOrientation::Negative {
            current_position.y -= placeholder_get_max_text_height(&*markings_texts);
        }
        for text in markings_texts {
            let text = Text::from(&*text.as_str());
            let text_width = placeholder_get_text_width(&text);

            self.draw_text_at_point(
                text,
                current_position.x - text_width / 2.0,
                current_position.y,
                self.coordinate_system_config.x_text_color,
                self.coordinate_system_config.x_text_size,
                frame,
            );

            self.draw_separator_line(separator_line_start, separator_line_end, frame);

            separator_line_start.x += step;
            separator_line_end.x += step;

            current_position.x += step;
        }
    }

    fn draw_x_axis_negative_marking_labels(
        &self,
        markings_texts: Vec<String>,
        axis_and_origin: &AxisAndOrigin,
        frame: &mut Frame,
    ) {
        let (
            x_axis_positive_length,
            x_axis_negative_length,
            y_axis_positive_length,
            y_axis_negative_length,
            origin,
        ) = axis_and_origin.get_axis_and_origin();

        let markings_amount = if markings_texts.len() <= 1 {
            return;
        } else {
            markings_texts.len() as f32 - 1.0
        };

        let mut current_position = origin;
        let step = x_axis_negative_length / markings_amount;

        let (mut separator_line_start, mut separator_line_end) =
            self.calculate_separator_line_parallel_to_y(&current_position, &axis_and_origin);
        if self.y_axis_orientation == YAxisOrientation::Negative {
            current_position.y -= placeholder_get_max_text_height(&*markings_texts);
        }
        for text in markings_texts {
            let text = Text::from(&*text.as_str());
            let text_width = placeholder_get_text_width(&text);

            self.draw_text_at_point(
                text,
                current_position.x - text_width / 2.0,
                current_position.y,
                self.coordinate_system_config.x_text_color,
                self.coordinate_system_config.x_text_size,
                frame,
            );

            self.draw_separator_line(separator_line_start, separator_line_end, frame);

            separator_line_start.x -= step;
            separator_line_end.x -= step;

            current_position.x -= step;
        }
    }

    fn draw_y_axis_negative_marking_labels(
        &self,
        markings_texts: Vec<String>,
        axis_and_origin: &AxisAndOrigin,
        frame: &mut Frame,
    ) {
        let (
            x_axis_positive_length,
            x_axis_negative_length,
            y_axis_positive_length,
            y_axis_negative_length,
            origin,
        ) = axis_and_origin.get_axis_and_origin();

        let amount = if markings_texts.len() <= 1 {
            return;
        } else {
            markings_texts.len() as f32 - 1.0
        };

        let mut current_position = origin;

        let step = y_axis_negative_length / amount;

        let (mut separator_line_start, mut separator_line_end) =
            self.calculate_separator_line_parallel_to_x(current_position, axis_and_origin);

        if self.x_axis_orientation == XAxisOrientation::Positive {
            current_position.x -= placeholder_get_max_text_width(&markings_texts);
        }

        for text in markings_texts {
            let text = Text::from(&*text.as_str());
            let text_height = placeholder_get_text_height(&text);

            self.draw_text_at_point(
                text,
                current_position.x,
                current_position.y - text_height / 2.0,
                self.coordinate_system_config.y_text_color,
                self.coordinate_system_config.y_text_size,
                frame,
            );
            self.draw_separator_line(separator_line_start, separator_line_end, frame);
            current_position.y += step;
            separator_line_start.y += step;
            separator_line_end.y += step;
        }
    }

    fn draw_y_axis_positive_marking_labels(
        &self,
        markings_texts: Vec<String>,
        axis_and_origin: &AxisAndOrigin,
        frame: &mut Frame,
    ) {
        let (
            x_axis_positive_length,
            x_axis_negative_length,
            y_axis_positive_length,
            y_axis_negative_length,
            origin,
        ) = axis_and_origin.get_axis_and_origin();

        let amount = if markings_texts.len() <= 1 {
            return;
        } else {
            markings_texts.len() as f32 - 1.0
        };

        let mut current_position = origin;

        let step = y_axis_positive_length / amount;
        let (mut separator_line_start, mut separator_line_end) =
            self.calculate_separator_line_parallel_to_x(current_position, axis_and_origin);

        if self.x_axis_orientation == XAxisOrientation::Positive {
            current_position.x -= placeholder_get_max_text_width(&markings_texts);
        }

        for text in markings_texts {
            let text = Text::from(&*text.as_str());
            let text_height = placeholder_get_text_height(&text);

            self.draw_text_at_point(
                text,
                current_position.x,
                current_position.y - text_height / 2.0,
                self.coordinate_system_config.y_text_color,
                self.coordinate_system_config.y_text_size,
                frame,
            );
            self.draw_separator_line(separator_line_start, separator_line_end, frame);
            current_position.y -= step;
            separator_line_start.y -= step;
            separator_line_end.y -= step;
        }
    }

    fn draw_text_at_point(
        &self,
        mut text: Text,
        top_left_x: f32,
        top_left_y: f32,
        color: Color,
        size: f32,
        frame: &mut Frame,
    ) {
        text.position = Point::new(top_left_x, top_left_y);
        text.color = color;
        text.size = size;
        frame.fill_text(text);
    }

    fn draw_separator_line(&self, start: Point, end: Point, frame: &mut Frame) {
        frame.stroke(
            &Path::line(start, end),
            Stroke {
                color: self.coordinate_system_config.separator_stroke_color,
                width: self.coordinate_system_config.separator_stroke_width,
                line_cap: LineCap::Butt,
                line_join: LineJoin::Round,
            },
        );
    }

    fn draw_axis_line(&self, start: Point, end: Point, frame: &mut Frame) {
        frame.stroke(
            &Path::line(start, end),
            Stroke {
                color: self.coordinate_system_config.axis_stroke_color,
                width: self.coordinate_system_config.axis_stroke_width,
                line_cap: LineCap::Butt,
                line_join: LineJoin::Round,
            },
        );
    }

    //TODO: implement
    fn draw_zero_at_origin() {
        unimplemented!()
    }
}
