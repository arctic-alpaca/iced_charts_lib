use crate::charts::axis_enums::{
    AxisOrientation, BarChartDataAxis, BarChartDataPositivity, XAxisOrientation, YAxisOrientation,
};
use crate::charts::bar_chart_data::{BarChartData, Dataset};
use crate::charts::bar_chart_style_info_iced::BarChartStyleInfoIced;
use crate::charts::coordinate_system::{
    AxisAndOrigin, CoordinateSystem, CoordinateSystemConfig, SpaceCalculator,
};
use crate::charts::drawing_helper::BarChartDrawingHelper;
use crate::charts::error::{ChartsLibError, ErrorKind};
use crate::charts::util;
use crate::charts::util::placeholder_get_text_width;
use crate::{charts, Message};
use iced::canvas::path::Builder;
use iced::canvas::{LineCap, LineJoin, Path, Stroke};
use iced::mouse::Interaction;
use iced::widget::canvas::{Cache, Cursor, Event, Frame, Geometry, Text};
use iced::{
    canvas, keyboard, mouse, Canvas, Color, Element, Length, Point, Rectangle, Size, Space,
};
use std::borrow::Borrow;
use std::collections::HashMap;

#[derive(Debug)]
pub struct BarChartIcedStruct {
    coordinate_system: CoordinateSystem,
    data_axis: BarChartDataAxis,
    pub data: BarChartData,
    cluster_names: Vec<String>,
    style: BarChartStyleInfoIced,
    chart_cache: Cache,
    rectangle_list: Vec<(Rectangle, f32)>,
    biggest_data_entry_abs: f32,
}

//TODO: Keyboard stuff
impl<'a> canvas::Program<Message> for BarChartIcedStruct {
    fn update(&mut self, event: Event, bounds: Rectangle<f32>, cursor: Cursor) -> Option<Message> {
        match event {
            Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::CursorMoved { .. } => {
                    if let Some(a) = cursor.position_in(&bounds) {
                        if self.rectangle_list.is_empty() {
                            self.rectangle_list = self.create_rectangle_list_for_hover(
                                &AxisAndOrigin::new(
                                    self.coordinate_system
                                        .calculate_axis_length_and_origin(bounds.size()),
                                ),
                                &mut Frame::new(bounds.size()),
                                &cursor,
                                &bounds,
                            );
                        }
                        for rectangle in self.rectangle_list.iter() {
                            if rectangle.0.contains(a) {
                                println!("{}", rectangle.1);
                                self.chart_cache.clear();
                            }
                        }
                        //return Some(Message::NewData);
                    };
                    None
                }
                _ => None,
            },
            Event::Keyboard(keyboard_event) => match keyboard_event {
                keyboard::Event::KeyPressed {
                    key_code,
                    modifiers,
                } => {
                    println!("{:?} - {:?}", key_code, modifiers);
                    None
                }
                keyboard::Event::KeyReleased {
                    key_code,
                    modifiers,
                } => {
                    println!("{:?} - {:?}", key_code, modifiers);
                    None
                }
                keyboard::Event::CharacterReceived(char) => {
                    println!("{:?}", char);
                    None
                }
                keyboard::Event::ModifiersChanged(modifiers) => {
                    println!("{:?}", modifiers);
                    None
                }
            },
        }
    }
    fn draw(&self, bounds: Rectangle<f32>, cursor: Cursor) -> Vec<Geometry> {
        let chart = self.chart_cache.draw(bounds.size(), |frame| {
            let cluster_amount =
                self.data
                    .get_biggest_amount_of_data_entries_in_one_dataset() as f32;
            let amount_of_bars_in_per_cluster = self.data.datasets.len() as f32;
            let axis_and_origin = AxisAndOrigin::new(
                self.coordinate_system
                    .calculate_axis_length_and_origin(frame.size()),
            );

            self.coordinate_system
                .draw_headline_and_labels(&axis_and_origin, frame);
            self.draw_bars(&axis_and_origin, frame, &cursor, &bounds);
            self.coordinate_system.draw_axis(&axis_and_origin, frame);
            frame.fill_text(format!(
                "X: {:?}\nY:{:?}",
                self.coordinate_system.x_axis_orientation,
                self.coordinate_system.y_axis_orientation
            ));
        });
        vec![chart]
    }
    fn mouse_interaction(&self, bounds: Rectangle<f32>, cursor: Cursor) -> mouse::Interaction {
        if cursor.is_over(&bounds) {
            mouse::Interaction::Crosshair
        } else {
            mouse::Interaction::default()
        }
    }
}

impl BarChartIcedStruct {
    pub fn new(
        data_axis: BarChartDataAxis,
        x_axis_orientation: XAxisOrientation,
        y_axis_orientation: YAxisOrientation,
        data: BarChartData,
        cluster_names: Vec<String>,
        style: BarChartStyleInfoIced,
    ) -> Result<Self, ChartsLibError> {
        let mut is_err = true;
        match data_axis {
            BarChartDataAxis::XPositive => match (x_axis_orientation, y_axis_orientation) {
                (XAxisOrientation::Positive, YAxisOrientation::Positive) => (is_err = false),
                (XAxisOrientation::Positive, YAxisOrientation::Negative) => (is_err = false),
                (XAxisOrientation::Positive, YAxisOrientation::Complete) => (is_err = false),
                _ => (),
            },
            BarChartDataAxis::XNegative => match (x_axis_orientation, y_axis_orientation) {
                (XAxisOrientation::Negative, YAxisOrientation::Positive) => (is_err = false),
                (XAxisOrientation::Negative, YAxisOrientation::Negative) => (is_err = false),
                (XAxisOrientation::Negative, YAxisOrientation::Complete) => (is_err = false),
                _ => (),
            },
            BarChartDataAxis::YPositive => match (x_axis_orientation, y_axis_orientation) {
                (XAxisOrientation::Positive, YAxisOrientation::Positive) => (is_err = false),
                (XAxisOrientation::Negative, YAxisOrientation::Positive) => (is_err = false),
                (XAxisOrientation::Complete, YAxisOrientation::Positive) => (is_err = false),
                _ => (),
            },
            BarChartDataAxis::YNegative => match (x_axis_orientation, y_axis_orientation) {
                (XAxisOrientation::Positive, YAxisOrientation::Negative) => (is_err = false),
                (XAxisOrientation::Negative, YAxisOrientation::Negative) => (is_err = false),
                (XAxisOrientation::Complete, YAxisOrientation::Negative) => (is_err = false),
                _ => (),
            },
        };
        if is_err {
            Err(ChartsLibError::new(
                ErrorKind::IncompatibleOrientationAndDataAxis,
                String::from(
                    "The chosen data axis is not compatible to the chosen coordinate system.",
                ),
            ))
        } else {
            //TODO: create proper coordinate_system_config here
            let biggest_data_entry_abs = calculate_biggest_data_value(&style, &data);

            Ok(BarChartIcedStruct {
                coordinate_system: CoordinateSystem::new(
                    x_axis_orientation,
                    y_axis_orientation,
                    update_config(
                        &data_axis,
                        &x_axis_orientation,
                        &y_axis_orientation,
                        &data,
                        &cluster_names,
                        &style,
                    ),
                ),
                data_axis,
                data,
                cluster_names,
                style,
                chart_cache: Default::default(),
                rectangle_list: vec![],
                biggest_data_entry_abs,
            })
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn add_dataset(&mut self, dataset: Dataset) {
        self.data.datasets.push(dataset);
        self.chart_cache.clear();
        self.rectangle_list.clear();
        self.recalculate_after_data_changes();
    }

    pub fn replace_data(&mut self, data: BarChartData) {
        self.data = data;
        self.chart_cache.clear();
        self.rectangle_list.clear();
        self.recalculate_after_data_changes();
    }

    pub fn remove_last_dataset(&mut self) {
        if self.data.datasets.is_empty() {
            return;
        }
        self.data.datasets.remove(self.data.datasets.len() - 1);
        self.chart_cache.clear();
        self.rectangle_list.clear();
        self.recalculate_after_data_changes();
    }

    fn recalculate_after_data_changes(&mut self) {
        if !self.data.datasets.is_empty() {
            self.coordinate_system.coordinate_system_config = update_config(
                &self.data_axis,
                &self.coordinate_system.x_axis_orientation,
                &self.coordinate_system.y_axis_orientation,
                &self.data,
                &self.cluster_names,
                &self.style,
            );
            self.biggest_data_entry_abs = calculate_biggest_data_value(&self.style, &self.data);
        }
    }

    fn calculate_bar_width_and_cluster_spacing(&self, axis_length: f32) -> (f32, f32) {
        let cluster_amount =
            self.data
                .get_biggest_amount_of_data_entries_in_one_dataset() as f32;
        let amount_of_bars_in_per_cluster = self.data.datasets.len() as f32;
        let mut cluster_spacing = self.style.min_cluster_spacing;

        let space_in_cluster_without_spacing =
            (axis_length - (cluster_amount * 2.0) * cluster_spacing) / cluster_amount;
        let space_inside_cluster_without_bar_spacing = space_in_cluster_without_spacing
            - (amount_of_bars_in_per_cluster - 1.0) * self.style.bar_spacing;

        let mut bar_width =
            space_inside_cluster_without_bar_spacing / amount_of_bars_in_per_cluster;

        let total_bar_amount = amount_of_bars_in_per_cluster * cluster_amount;
        if bar_width > self.style.maximum_bar_width {
            cluster_spacing += ((bar_width - self.style.maximum_bar_width) * total_bar_amount)
                / (cluster_amount * 2.0);
            bar_width = self.style.maximum_bar_width;
        }
        (bar_width, cluster_spacing)
    }

    fn create_drawable_bar_vec(&self) -> Vec<Vec<(u32, f32)>> {
        let cluster_amount = self
            .data
            .get_biggest_amount_of_data_entries_in_one_dataset();

        let mut data_vector: Vec<Vec<(u32, f32)>> =
            vec![vec![(0, 0.0); self.data.datasets.len()]; cluster_amount];

        for (dataset_index, dataset) in self.data.datasets.iter().enumerate() {
            for cluster_index in 0..cluster_amount {
                if let Some(entry) = dataset.data.get(cluster_index) {
                    data_vector.get_mut(cluster_index).unwrap()[dataset_index].1 = *entry;
                }
                data_vector[cluster_index][dataset_index].0 = dataset.id;
            }
        }
        data_vector
    }

    fn draw_bars(
        &self,
        axis_and_origin: &AxisAndOrigin,
        frame: &mut Frame,
        cursor: &Cursor,
        bounds: &Rectangle,
    ) -> Vec<(Rectangle, f32)> {
        let mut rectangle_list_to_return: Vec<(Rectangle, f32)> = vec![];

        let (
            x_axis_positive_length,
            x_axis_negative_length,
            y_axis_positive_length,
            y_axis_negative_length,
            origin,
        ) = axis_and_origin.get_axis_and_origin();

        let data_vector = self.create_drawable_bar_vec();

        let bar_spacing = if self.data.datasets.len() == 1 {
            0.0
        } else {
            self.style.bar_spacing
        };
        let (bar_width, cluster_spacing) = match self.data_axis {
            BarChartDataAxis::XPositive => {
                self.calculate_bar_width_and_cluster_spacing(x_axis_positive_length)
            }
            BarChartDataAxis::XNegative => {
                self.calculate_bar_width_and_cluster_spacing(x_axis_negative_length)
            }
            BarChartDataAxis::YPositive => {
                self.calculate_bar_width_and_cluster_spacing(y_axis_positive_length)
            }
            BarChartDataAxis::YNegative => {
                self.calculate_bar_width_and_cluster_spacing(y_axis_negative_length)
            }
        };

        let factor = 1.0 / self.biggest_data_entry_abs;
        let indicator_axis_length: f32;
        let mut current_position = origin;
        let mut draw_from_top = false;
        let mut draw_from_left = false;
        let mut orientation_factor = 1.0;
        let positivity: BarChartDataPositivity;

        match (
            self.data_axis,
            self.coordinate_system.x_axis_orientation,
            self.coordinate_system.y_axis_orientation,
        ) {
            (
                BarChartDataAxis::XPositive,
                XAxisOrientation::Positive,
                YAxisOrientation::Positive,
            ) => {
                indicator_axis_length = y_axis_positive_length;
                draw_from_top = true;
                positivity = BarChartDataPositivity::Positive;
            }
            (
                BarChartDataAxis::XPositive,
                XAxisOrientation::Positive,
                YAxisOrientation::Negative,
            ) => {
                indicator_axis_length = y_axis_negative_length;
                draw_from_top = true;
                positivity = BarChartDataPositivity::Negative;
            }
            (
                BarChartDataAxis::XPositive,
                XAxisOrientation::Positive,
                YAxisOrientation::Complete,
            ) => {
                indicator_axis_length = y_axis_negative_length;
                draw_from_top = true;
                positivity = BarChartDataPositivity::Both;
            }
            (
                BarChartDataAxis::XNegative,
                XAxisOrientation::Negative,
                YAxisOrientation::Positive,
            ) => {
                orientation_factor = -1.0;
                current_position.x -= bar_width;
                indicator_axis_length = y_axis_positive_length;
                draw_from_top = true;
                positivity = BarChartDataPositivity::Positive;
            }
            (
                BarChartDataAxis::XNegative,
                XAxisOrientation::Negative,
                YAxisOrientation::Negative,
            ) => {
                orientation_factor = -1.0;
                current_position.x -= bar_width;
                indicator_axis_length = y_axis_negative_length;
                draw_from_top = true;
                positivity = BarChartDataPositivity::Negative;
            }
            (
                BarChartDataAxis::XNegative,
                XAxisOrientation::Negative,
                YAxisOrientation::Complete,
            ) => {
                orientation_factor = -1.0;
                current_position.x -= bar_width;
                indicator_axis_length = y_axis_negative_length;
                draw_from_top = true;
                positivity = BarChartDataPositivity::Both;
            }
            (
                BarChartDataAxis::YPositive,
                XAxisOrientation::Positive,
                YAxisOrientation::Positive,
            ) => {
                orientation_factor = -1.0;
                current_position.y -= bar_width;
                indicator_axis_length = x_axis_positive_length;
                positivity = BarChartDataPositivity::Positive;
            }
            (
                BarChartDataAxis::YPositive,
                XAxisOrientation::Negative,
                YAxisOrientation::Positive,
            ) => {
                orientation_factor = -1.0;
                current_position.y -= bar_width;
                indicator_axis_length = x_axis_negative_length;
                positivity = BarChartDataPositivity::Negative;
            }
            (
                BarChartDataAxis::YPositive,
                XAxisOrientation::Complete,
                YAxisOrientation::Positive,
            ) => {
                orientation_factor = -1.0;
                current_position.y -= bar_width;
                indicator_axis_length = x_axis_positive_length;
                positivity = BarChartDataPositivity::Both;
            }
            (
                BarChartDataAxis::YNegative,
                XAxisOrientation::Positive,
                YAxisOrientation::Negative,
            ) => {
                indicator_axis_length = x_axis_positive_length;
                positivity = BarChartDataPositivity::Positive;
            }
            (
                BarChartDataAxis::YNegative,
                XAxisOrientation::Negative,
                YAxisOrientation::Negative,
            ) => {
                indicator_axis_length = x_axis_negative_length;
                positivity = BarChartDataPositivity::Negative;
            }
            (
                BarChartDataAxis::YNegative,
                XAxisOrientation::Complete,
                YAxisOrientation::Negative,
            ) => {
                indicator_axis_length = x_axis_negative_length;
                positivity = BarChartDataPositivity::Both;
            }
            (_, _, _) => panic!("Bar Graph draw bars died"),
        }

        for cluster in &data_vector {
            if self.data_axis == BarChartDataAxis::XPositive
                || self.data_axis == BarChartDataAxis::XNegative
            {
                current_position.x += cluster_spacing * orientation_factor;
            } else {
                current_position.y += cluster_spacing * orientation_factor;
            }

            for entry in cluster {
                let mut bar_length = indicator_axis_length * entry.1 * factor;
                let mut is_entry_negative = false;
                if bar_length < 0.0 {
                    bar_length *= -1.0;
                    is_entry_negative = true;
                    if self.data_axis == BarChartDataAxis::XPositive
                        || self.data_axis == BarChartDataAxis::XNegative
                    {
                        draw_from_top = !draw_from_top;
                    } else {
                        draw_from_left = !draw_from_left;
                    }
                }
                let color = match self.style.color_map.get(&entry.0) {
                    Some(temp) => *temp,
                    None => {
                        self.style.standard_color_map
                            [entry.0 as usize % self.style.standard_color_map.len()]
                    }
                };
                let size = if self.data_axis == BarChartDataAxis::XPositive
                    || self.data_axis == BarChartDataAxis::XNegative
                {
                    Size::new(bar_width, bar_length)
                } else {
                    Size::new(bar_length, bar_width)
                };
                if draw_from_top {
                    current_position.y -= bar_length;
                }
                if draw_from_left {
                    current_position.x -= bar_length;
                }

                if (is_entry_negative, positivity) == (true, BarChartDataPositivity::Negative)
                    || (is_entry_negative, positivity) == (true, BarChartDataPositivity::Both)
                    || (is_entry_negative, positivity) == (false, BarChartDataPositivity::Positive)
                    || (is_entry_negative, positivity) == (false, BarChartDataPositivity::Both)
                    || bar_length == 0.0
                {
                    frame.fill(
                        &Path::rectangle(Point::new(current_position.x, current_position.y), size),
                        color,
                    );
                    rectangle_list_to_return.push((
                        self.create_rectangle_for_list(
                            current_position,
                            indicator_axis_length,
                            origin,
                            bar_width,
                        ),
                        entry.1,
                    ));
                }

                if draw_from_top {
                    current_position.y += bar_length;
                }
                if draw_from_left {
                    current_position.x += bar_length;
                }

                if is_entry_negative {
                    if self.data_axis == BarChartDataAxis::XPositive
                        || self.data_axis == BarChartDataAxis::XNegative
                    {
                        draw_from_top = !draw_from_top;
                    } else {
                        draw_from_left = !draw_from_left;
                    }
                }

                if self.data_axis == BarChartDataAxis::XPositive
                    || self.data_axis == BarChartDataAxis::XNegative
                {
                    current_position.x += (bar_spacing + bar_width) * orientation_factor;
                } else {
                    current_position.y += (bar_spacing + bar_width) * orientation_factor;
                }
            }
            if self.data_axis == BarChartDataAxis::XPositive
                || self.data_axis == BarChartDataAxis::XNegative
            {
                current_position.x += (cluster_spacing - bar_spacing) * orientation_factor;
            } else {
                current_position.y += (cluster_spacing - bar_spacing) * orientation_factor;
            }
        }
        rectangle_list_to_return
    }

    fn create_rectangle_for_list(
        &self,
        current_position: Point,
        indicator_axis_length: f32,
        origin: Point,
        bar_width: f32,
    ) -> Rectangle {
        let mut point_for_rectangle_list = Point::new(current_position.x, current_position.y);

        let size = if self.data_axis == BarChartDataAxis::XPositive
            || self.data_axis == BarChartDataAxis::XNegative
        {
            if self.coordinate_system.y_axis_orientation == YAxisOrientation::Positive {
                point_for_rectangle_list.y = origin.y - indicator_axis_length;
                Size::new(bar_width, indicator_axis_length)
            } else if self.coordinate_system.y_axis_orientation == YAxisOrientation::Complete {
                point_for_rectangle_list.y = origin.y - indicator_axis_length;
                Size::new(bar_width, indicator_axis_length * 2.0)
            } else {
                Size::new(bar_width, indicator_axis_length)
            }
        } else if self.coordinate_system.x_axis_orientation == XAxisOrientation::Negative {
            point_for_rectangle_list.x = origin.x - indicator_axis_length;
            Size::new(indicator_axis_length, bar_width)
        } else if self.coordinate_system.x_axis_orientation == XAxisOrientation::Complete {
            point_for_rectangle_list.x = origin.x - indicator_axis_length;
            Size::new(indicator_axis_length * 2.0, bar_width)
        } else {
            Size::new(indicator_axis_length, bar_width)
        };
        Rectangle::new(point_for_rectangle_list, size)
    }

    fn create_rectangle_list_for_hover(
        &self,
        axis_and_origin: &AxisAndOrigin,
        frame: &mut Frame,
        cursor: &Cursor,
        bounds: &Rectangle,
    ) -> Vec<(Rectangle, f32)> {
        self.draw_bars(&axis_and_origin, frame, cursor, bounds)
    }

    pub fn set_headline(&mut self, headline: String) {
        self.coordinate_system.coordinate_system_config.headline = Some(headline);
    }
}

fn update_config(
    data_axis: &BarChartDataAxis,
    x_axis_orientation: &XAxisOrientation,
    y_axis_orientation: &YAxisOrientation,
    data: &BarChartData,
    cluster_names: &[String],
    style: &BarChartStyleInfoIced,
) -> CoordinateSystemConfig {
    /*
    x_text,
    x_text_color,
    y_text,
    y_text_color,
    x_marking_amount,
    y_marking_amount,
    headline,
    headline_color,
    separator_stroke_width,
    separator_stroke_color,
    axis_stroke_color,
    axis_stroke_width,
    legend_alignment,
    text_padding,
    x_min_value,
    x_max_value,
    y_min_value,
    y_max_value,
    cluster_text_size: TEXT_SIZE,
    headline_size: TEXT_SIZE,
    markings_text_size: TEXT_SIZE,*/

    let biggest_entry_abs = if let Some(max_value) = style.max_value {
        let biggest_entry_abs = data.get_biggest_data_entry_abs();
        if max_value > biggest_entry_abs {
            max_value
        } else {
            biggest_entry_abs
        }
    } else {
        data.get_biggest_data_entry_abs()
    };

    let mut coordinate_system_config = CoordinateSystemConfig::new();
    coordinate_system_config.x_max_value = 0.0;
    coordinate_system_config.x_min_value = 0.0;
    coordinate_system_config.y_max_value = 0.0;
    coordinate_system_config.y_min_value = 0.0;

    match data_axis {
        BarChartDataAxis::XPositive => {
            match (x_axis_orientation, y_axis_orientation) {
                (XAxisOrientation::Positive, YAxisOrientation::Positive) => {
                    coordinate_system_config.y_max_value = biggest_entry_abs;

                    coordinate_system_config.draw_cluster_label_on_x_axis =
                        Some(XAxisOrientation::Positive);
                    coordinate_system_config.draw_marking_label_on_y_axis =
                        Some(*y_axis_orientation);
                }
                (XAxisOrientation::Positive, YAxisOrientation::Negative) => {
                    coordinate_system_config.y_min_value = -biggest_entry_abs;

                    coordinate_system_config.draw_cluster_label_on_x_axis =
                        Some(XAxisOrientation::Positive);
                    coordinate_system_config.draw_marking_label_on_y_axis =
                        Some(*y_axis_orientation);
                }
                (XAxisOrientation::Positive, YAxisOrientation::Complete) => {
                    coordinate_system_config.y_max_value = biggest_entry_abs;
                    coordinate_system_config.y_min_value = -biggest_entry_abs;

                    coordinate_system_config.draw_cluster_label_on_x_axis =
                        Some(XAxisOrientation::Positive);
                    coordinate_system_config.draw_marking_label_on_y_axis =
                        Some(*y_axis_orientation);
                }
                _ => (),
            }
            if let Some(markings_text_size) = style.markings_text_size {
                coordinate_system_config.y_text_size = markings_text_size;
            }
            if let Some(clusters_text_size) = style.cluster_text_size {
                coordinate_system_config.x_text_size = clusters_text_size;
            }
        }
        BarChartDataAxis::XNegative => {
            match (x_axis_orientation, y_axis_orientation) {
                (XAxisOrientation::Negative, YAxisOrientation::Positive) => {
                    coordinate_system_config.y_max_value = biggest_entry_abs;

                    coordinate_system_config.draw_cluster_label_on_x_axis =
                        Some(XAxisOrientation::Negative);
                    coordinate_system_config.draw_marking_label_on_y_axis =
                        Some(*y_axis_orientation);
                }
                (XAxisOrientation::Negative, YAxisOrientation::Negative) => {
                    coordinate_system_config.y_min_value = -biggest_entry_abs;

                    coordinate_system_config.draw_cluster_label_on_x_axis =
                        Some(XAxisOrientation::Negative);

                    coordinate_system_config.draw_marking_label_on_y_axis =
                        Some(*y_axis_orientation);
                }
                (XAxisOrientation::Negative, YAxisOrientation::Complete) => {
                    coordinate_system_config.y_max_value = biggest_entry_abs;
                    coordinate_system_config.y_min_value = -biggest_entry_abs;

                    coordinate_system_config.draw_cluster_label_on_x_axis =
                        Some(XAxisOrientation::Negative);

                    coordinate_system_config.draw_marking_label_on_y_axis =
                        Some(*y_axis_orientation);
                }
                _ => (),
            }
            if let Some(markings_text_size) = style.markings_text_size {
                coordinate_system_config.y_text_size = markings_text_size;
            }
            if let Some(clusters_text_size) = style.cluster_text_size {
                coordinate_system_config.x_text_size = clusters_text_size;
            }
        }
        BarChartDataAxis::YPositive => {
            match (x_axis_orientation, y_axis_orientation) {
                (XAxisOrientation::Positive, YAxisOrientation::Positive) => {
                    coordinate_system_config.x_max_value = biggest_entry_abs;

                    coordinate_system_config.draw_cluster_label_on_y_axis =
                        Some(YAxisOrientation::Positive);
                    coordinate_system_config.draw_marking_label_on_x_axis =
                        Some(*x_axis_orientation);
                }
                (XAxisOrientation::Negative, YAxisOrientation::Positive) => {
                    coordinate_system_config.x_min_value = -biggest_entry_abs;

                    coordinate_system_config.draw_cluster_label_on_y_axis =
                        Some(YAxisOrientation::Positive);
                    coordinate_system_config.draw_marking_label_on_x_axis =
                        Some(*x_axis_orientation);
                }
                (XAxisOrientation::Complete, YAxisOrientation::Positive) => {
                    coordinate_system_config.x_max_value = biggest_entry_abs;
                    coordinate_system_config.x_min_value = -biggest_entry_abs;

                    coordinate_system_config.draw_cluster_label_on_y_axis =
                        Some(YAxisOrientation::Positive);
                    coordinate_system_config.draw_marking_label_on_x_axis =
                        Some(*x_axis_orientation);
                }
                _ => (),
            }
            if let Some(markings_text_size) = style.markings_text_size {
                coordinate_system_config.x_text_size = markings_text_size;
            }
            if let Some(clusters_text_size) = style.cluster_text_size {
                coordinate_system_config.y_text_size = clusters_text_size;
            }
        }
        BarChartDataAxis::YNegative => {
            match (x_axis_orientation, y_axis_orientation) {
                (XAxisOrientation::Positive, YAxisOrientation::Negative) => {
                    coordinate_system_config.x_max_value = biggest_entry_abs;

                    coordinate_system_config.draw_cluster_label_on_y_axis =
                        Some(YAxisOrientation::Negative);
                    coordinate_system_config.draw_marking_label_on_x_axis =
                        Some(*x_axis_orientation);
                }
                (XAxisOrientation::Negative, YAxisOrientation::Negative) => {
                    coordinate_system_config.x_min_value = -biggest_entry_abs;

                    coordinate_system_config.draw_cluster_label_on_y_axis =
                        Some(YAxisOrientation::Negative);
                    coordinate_system_config.draw_marking_label_on_x_axis =
                        Some(*x_axis_orientation);
                }
                (XAxisOrientation::Complete, YAxisOrientation::Negative) => {
                    coordinate_system_config.x_max_value = biggest_entry_abs;
                    coordinate_system_config.x_min_value = -biggest_entry_abs;

                    coordinate_system_config.draw_cluster_label_on_y_axis =
                        Some(YAxisOrientation::Negative);
                    coordinate_system_config.draw_marking_label_on_x_axis =
                        Some(*x_axis_orientation);
                }
                _ => (),
            }
            if let Some(markings_text_size) = style.markings_text_size {
                coordinate_system_config.x_text_size = markings_text_size;
            }
            if let Some(clusters_text_size) = style.cluster_text_size {
                coordinate_system_config.y_text_size = clusters_text_size;
            }
        }
    };

    match data_axis {
        BarChartDataAxis::XPositive => {
            coordinate_system_config.x_text = Some(cluster_names.to_vec());
        }
        BarChartDataAxis::XNegative => {
            coordinate_system_config.x_text = Some(cluster_names.to_vec());
        }
        BarChartDataAxis::YPositive => {
            coordinate_system_config.y_text = Some(cluster_names.to_vec());
        }
        BarChartDataAxis::YNegative => {
            coordinate_system_config.y_text = Some(cluster_names.to_vec());
        }
    }

    coordinate_system_config.x_marking_amount = style.x_marking_amount;
    coordinate_system_config.y_marking_amount = style.y_marking_amount;

    if let Some(axis_stroke_width) = style.axis_stroke_width {
        coordinate_system_config.axis_stroke_width = axis_stroke_width;
    }
    if let Some(axis_stroke_color) = style.axis_stroke_color {
        coordinate_system_config.axis_stroke_color = axis_stroke_color;
    }
    if let Some(separator_stroke_width) = style.separator_stroke_width {
        coordinate_system_config.separator_stroke_width = separator_stroke_width;
    }
    if let Some(separator_stroke_color) = style.separator_stroke_color {
        coordinate_system_config.separator_stroke_color = separator_stroke_color;
    }

    coordinate_system_config.headline = style.headline.clone();

    if let Some(headline_color) = style.headline_color {
        coordinate_system_config.headline_color = headline_color;
    }
    if let Some(headline_size) = style.headline_size {
        coordinate_system_config.headline_size = headline_size;
    }
    coordinate_system_config.legend_alignment = style.legend_alignment;

    if let Some(text_padding) = style.text_padding {
        coordinate_system_config.text_padding = text_padding;
    }

    coordinate_system_config
}
fn calculate_biggest_data_value(style: &BarChartStyleInfoIced, data: &BarChartData) -> f32 {
    if let Some(max_value) = style.max_value {
        let biggest_entry_abs = data.get_biggest_data_entry_abs();
        if max_value.abs() > biggest_entry_abs {
            max_value.abs()
        } else {
            biggest_entry_abs
        }
    } else {
        data.get_biggest_data_entry_abs()
    }
}
