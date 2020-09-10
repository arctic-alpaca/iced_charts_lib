#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod charts;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use iced::widget::canvas::Frame;
use iced::{
    button,
    canvas::{self, Cache, Canvas, Cursor, Geometry},
    executor, time, window, Application, Button, Color, Column, Command, Container, Element, Font,
    HorizontalAlignment, Length, Point, Rectangle, Row, Settings, Size, Subscription, Text, Vector,
    VerticalAlignment,
};

use crate::charts::*;

use crate::charts::axis_enums::{
    AxisOrientation, BarChartDataAxis, XAxisOrientation, YAxisOrientation,
};
use crate::charts::bar_chart_data::{BarChartData, Dataset};
use crate::charts::bar_chart_iced_struct::BarChartIcedStruct;
use crate::charts::coordinate_system::{CoordinateSystem, CoordinateSystemConfig};
use charts::bar_chart_style_info_iced::BarChartStyleInfoIced;
use iced::widget::pane_grid::Axis;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::Pointer;

/*
Vec<Rectangle, f32> liste mit balken zum durchsuchen für hover, hover und blase per overlay um erneutes zeichnen zu verhindern?
frame speichern und nur overlay neu zeichnen? nur bei veränderten daten frame verändern?
 */

pub fn main() -> iced::Result {
    ChartDrawingTest::run(Settings {
        antialiasing: true,
        window: window::Settings {
            size: (1700, 1300),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

struct ChartDrawingTest {
    x_pos_y_pos_x: ChartDrawer1,
    x_pos_y_neg_x: ChartDrawer1,
    x_neg_y_pos_x: ChartDrawer1,
    x_neg_y_neg_x: ChartDrawer1,
    x_pos_y_pos_y: ChartDrawer1,
    x_neg_y_pos_y: ChartDrawer1,
    x_pos_y_neg_y: ChartDrawer1,
    x_neg_y_neg_y: ChartDrawer1,
    x_complete_y_neg_y: ChartDrawer1,
    x_complete_y_pos_y: ChartDrawer1,
    x_pos_y_complete_y: ChartDrawer1,
    x_neg_y_complete_y: ChartDrawer1,
    add_button: button::State,
    remove_button: button::State,
}

struct ChartDrawer1 {
    chart: BarChartIcedStruct,
}
#[derive(Debug, Clone, Copy)]
pub enum Message {
    NewData,
    AddButton,
    RemoveButton,
}

impl ChartDrawer1 {
    fn new(
        axis_data: BarChartDataAxis,
        x_axis_orientation: XAxisOrientation,
        y_axis_orientation: YAxisOrientation,
    ) -> Self {
        let dataset1 = Dataset::new(0, String::from("test0"), vec![1.0, 0.1, 0.2]);
        let dataset2 = Dataset::new(1, String::from("test1"), vec![-1.2, -2.3, -2.4]);
        let dataset3 = Dataset::new(2, String::from("test2"), vec![-1.5, -1.4, -1.3]);
        let dataset4 = Dataset::new(3, String::from("test3"), vec![1.1, 3.0, 2.3]);
        let dataset5 = Dataset::new(4, String::from("test4"), vec![-0.3, -2.1]);
        let dataset6 = Dataset::new(5, String::from("test5"), vec![1.5, 2.5, 2.0]);
        let dataset7 = Dataset::new(3, String::from("test3"), vec![-3.0, -2.7, -1.8]);
        let dataset8 = Dataset::new(4, String::from("test4"), vec![2.4, 1.4]);
        let dataset9 = Dataset::new(5, String::from("test5"), vec![1.5, 2.2, 3.0]);

        let mut hashmap_object: HashMap<u32, Color> = HashMap::new();
        hashmap_object.insert(0, Color::from_rgb(252.0 / 255.0, 7.0 / 255.0, 3.0 / 255.0));
        hashmap_object.insert(1, Color::from_rgb(252.0 / 255.0, 82. / 255.0, 3.0 / 255.0));
        hashmap_object.insert(
            2,
            Color::from_rgb(252.0 / 255.0, 169.0 / 255.0, 3.0 / 255.0),
        );
        hashmap_object.insert(
            3,
            Color::from_rgb(190.0 / 255.0, 252.0 / 255.0, 3.0 / 255.0),
        );
        hashmap_object.insert(4, Color::from_rgb(73.0 / 255.0, 252.0 / 255.0, 3.0 / 255.0));
        hashmap_object.insert(
            5,
            Color::from_rgb(3.0 / 255.0, 252.0 / 255.0, 132.0 / 255.0),
        );

        let data_second_object = BarChartData::new(vec![
            dataset1, dataset2, dataset3, dataset4, dataset5, dataset6, dataset7, dataset8,
            dataset9,
        ]);

        let chart_object = BarChartIcedStruct::new(
            axis_data,
            x_axis_orientation,
            y_axis_orientation,
            data_second_object,
            vec![
                String::from("test1"),
                String::from("test2"),
                String::from("test3"),
            ],
            BarChartStyleInfoIced::new(),
        )
        .unwrap();

        ChartDrawer1 {
            chart: chart_object,
        }
    }
}

impl Application for ChartDrawingTest {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            ChartDrawingTest {
                x_pos_y_pos_x: ChartDrawer1::new(
                    BarChartDataAxis::XPositive,
                    XAxisOrientation::Positive,
                    YAxisOrientation::Positive,
                ),
                x_pos_y_neg_x: ChartDrawer1::new(
                    BarChartDataAxis::XPositive,
                    XAxisOrientation::Positive,
                    YAxisOrientation::Negative,
                ),
                x_neg_y_pos_x: ChartDrawer1::new(
                    BarChartDataAxis::XNegative,
                    XAxisOrientation::Negative,
                    YAxisOrientation::Positive,
                ),
                x_neg_y_neg_x: ChartDrawer1::new(
                    BarChartDataAxis::XNegative,
                    XAxisOrientation::Negative,
                    YAxisOrientation::Positive,
                ),
                x_pos_y_pos_y: ChartDrawer1::new(
                    BarChartDataAxis::YPositive,
                    XAxisOrientation::Positive,
                    YAxisOrientation::Positive,
                ),
                x_neg_y_pos_y: ChartDrawer1::new(
                    BarChartDataAxis::YPositive,
                    XAxisOrientation::Negative,
                    YAxisOrientation::Positive,
                ),
                x_pos_y_neg_y: ChartDrawer1::new(
                    BarChartDataAxis::YNegative,
                    XAxisOrientation::Positive,
                    YAxisOrientation::Negative,
                ),
                x_neg_y_neg_y: ChartDrawer1::new(
                    BarChartDataAxis::YNegative,
                    XAxisOrientation::Negative,
                    YAxisOrientation::Negative,
                ),
                x_complete_y_neg_y: ChartDrawer1::new(
                    BarChartDataAxis::YNegative,
                    XAxisOrientation::Complete,
                    YAxisOrientation::Negative,
                ),
                x_complete_y_pos_y: ChartDrawer1::new(
                    BarChartDataAxis::YPositive,
                    XAxisOrientation::Complete,
                    YAxisOrientation::Positive,
                ),
                x_pos_y_complete_y: ChartDrawer1::new(
                    BarChartDataAxis::XPositive,
                    XAxisOrientation::Positive,
                    YAxisOrientation::Complete,
                ),
                x_neg_y_complete_y: ChartDrawer1::new(
                    BarChartDataAxis::XNegative,
                    XAxisOrientation::Negative,
                    YAxisOrientation::Complete,
                ),
                add_button: button::State::new(),
                remove_button: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Graph-lib - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::NewData => {
                println!("new data arrived, data gets inserted, cache gets cleared");
            }
            Message::AddButton => {
                let rand_string: String =
                    thread_rng().sample_iter(&Alphanumeric).take(10).collect();
                let mut rng = rand::thread_rng();
                let dataset = Dataset::new(
                    self.x_complete_y_neg_y
                        .chart
                        .data
                        .datasets
                        .last()
                        .unwrap_or(&Dataset::new(0, String::from("..."), vec![]))
                        .id
                        + 1,
                    rand_string,
                    vec![
                        rng.gen_range(-5.0, 5.0),
                        rng.gen_range(-5.0, 5.0),
                        rng.gen_range(-5.0, 5.0),
                    ],
                );

                self.x_pos_y_pos_x.chart.add_dataset(dataset.clone());
                self.x_pos_y_neg_x.chart.add_dataset(dataset.clone());
                self.x_neg_y_pos_x.chart.add_dataset(dataset.clone());
                self.x_neg_y_neg_x.chart.add_dataset(dataset.clone());
                self.x_pos_y_pos_y.chart.add_dataset(dataset.clone());
                self.x_neg_y_pos_y.chart.add_dataset(dataset.clone());
                self.x_pos_y_neg_y.chart.add_dataset(dataset.clone());
                self.x_neg_y_neg_y.chart.add_dataset(dataset.clone());
                self.x_complete_y_neg_y.chart.add_dataset(dataset.clone());
                self.x_complete_y_pos_y.chart.add_dataset(dataset.clone());
                self.x_pos_y_complete_y.chart.add_dataset(dataset.clone());
                self.x_neg_y_complete_y.chart.add_dataset(dataset);
                println!("Add Button pressed");
            }
            Message::RemoveButton => {
                self.x_pos_y_pos_x.chart.remove_last_dataset();
                self.x_pos_y_neg_x.chart.remove_last_dataset();
                self.x_neg_y_pos_x.chart.remove_last_dataset();
                self.x_neg_y_neg_x.chart.remove_last_dataset();
                self.x_pos_y_pos_y.chart.remove_last_dataset();
                self.x_neg_y_pos_y.chart.remove_last_dataset();
                self.x_pos_y_neg_y.chart.remove_last_dataset();
                self.x_neg_y_neg_y.chart.remove_last_dataset();
                self.x_complete_y_neg_y.chart.remove_last_dataset();
                self.x_complete_y_pos_y.chart.remove_last_dataset();
                self.x_pos_y_complete_y.chart.remove_last_dataset();
                self.x_neg_y_complete_y.chart.remove_last_dataset();
                println!("Remove Button pressed");
            }
        }

        Command::none()
    }
    /*
    fn subscription(&self) -> Subscription<Message> {
        //time::every(std::time::Duration::from_millis(50)).map(|_| Message::Event)
    }*/

    fn view(&mut self) -> Element<Message> {
        let width = Length::Units(400);
        let height = Length::Units(400);

        let x_pos_y_pos_x = Container::new(self.x_pos_y_pos_x.chart.view())
            .width(width)
            .height(height);
        let x_pos_y_neg_x = Container::new(self.x_pos_y_neg_x.chart.view())
            .width(width)
            .height(height);
        let x_neg_y_pos_x = Container::new(self.x_neg_y_pos_x.chart.view())
            .width(width)
            .height(height);
        let x_neg_y_neg_x = Container::new(self.x_neg_y_neg_x.chart.view())
            .width(width)
            .height(height);
        let x_pos_y_pos_y = Container::new(self.x_pos_y_pos_y.chart.view())
            .width(width)
            .height(height);
        let x_neg_y_pos_y = Container::new(self.x_neg_y_pos_y.chart.view())
            .width(width)
            .height(height);
        let x_pos_y_neg_y = Container::new(self.x_pos_y_neg_y.chart.view())
            .width(width)
            .height(height);
        let x_neg_y_neg_y = Container::new(self.x_neg_y_neg_y.chart.view())
            .width(width)
            .height(height);

        let x_complete_y_neg_y = Container::new(self.x_complete_y_neg_y.chart.view())
            .width(width)
            .height(height);
        let x_complete_y_pos_y = Container::new(self.x_complete_y_pos_y.chart.view())
            .width(width)
            .height(height);
        let x_pos_y_complete_y = Container::new(self.x_pos_y_complete_y.chart.view())
            .width(width)
            .height(height);
        let x_neg_y_complete_y = Container::new(self.x_neg_y_complete_y.chart.view())
            .width(width)
            .height(height);
        let row1 = Row::new()
            .push(x_pos_y_pos_x)
            .push(x_pos_y_neg_x)
            .push(x_neg_y_pos_x)
            .push(x_neg_y_neg_x);
        let row2 = Row::new()
            .push(x_pos_y_pos_y)
            .push(x_pos_y_neg_y)
            .push(x_neg_y_pos_y)
            .push(x_neg_y_neg_y);

        let row3 = Row::new()
            .push(x_complete_y_neg_y)
            .push(x_complete_y_pos_y)
            .push(x_pos_y_complete_y)
            .push(x_neg_y_complete_y);
        let row4 = Row::new()
            .push(
                Button::new(&mut self.add_button, Text::new("Add Data"))
                    .on_press(Message::AddButton),
            )
            .push(
                Button::new(&mut self.remove_button, Text::new("Remove Data"))
                    .on_press(Message::RemoveButton),
            );

        let column = Column::new().push(row1).push(row2).push(row3).push(row4);

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }
}
