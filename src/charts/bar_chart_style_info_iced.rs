use crate::charts::coordinate_system::CoordinateSystemLegendAlignment;
use iced::Color;
use std::collections::HashMap;

const STANDARD_COLOR_MAP: [Color; 11] = [
    Color::from_rgb(
        0x58 as f32 / 255.0,
        0x99 as f32 / 255.0,
        0xDA as f32 / 255.0,
    ),
    Color::from_rgb(
        0xE8 as f32 / 255.0,
        0x74 as f32 / 255.0,
        0x3B as f32 / 255.0,
    ),
    Color::from_rgb(
        0x19 as f32 / 255.0,
        0xA9 as f32 / 255.0,
        0x79 as f32 / 255.0,
    ),
    Color::from_rgb(
        0xED as f32 / 255.0,
        0x4A as f32 / 255.0,
        0x7B as f32 / 255.0,
    ),
    Color::from_rgb(
        0x94 as f32 / 255.0,
        0x5E as f32 / 255.0,
        0xCF as f32 / 255.0,
    ),
    Color::from_rgb(
        0x13 as f32 / 255.0,
        0xA4 as f32 / 255.0,
        0xB4 as f32 / 255.0,
    ),
    Color::from_rgb(
        0x52 as f32 / 255.0,
        0x5D as f32 / 255.0,
        0xF4 as f32 / 255.0,
    ),
    Color::from_rgb(
        0xBF as f32 / 255.0,
        0x39 as f32 / 255.0,
        0x9E as f32 / 255.0,
    ),
    Color::from_rgb(
        0x6C as f32 / 255.0,
        0x88 as f32 / 255.0,
        0x93 as f32 / 255.0,
    ),
    Color::from_rgb(
        0xEE as f32 / 255.0,
        0x68 as f32 / 255.0,
        0x68 as f32 / 255.0,
    ),
    Color::from_rgb(
        0x2F as f32 / 255.0,
        0x64 as f32 / 255.0,
        0x97 as f32 / 255.0,
    ),
];

#[derive(Debug)]
pub struct BarChartStyleInfoIced {
    pub color_map: HashMap<u32, Color>,
    pub standard_color_map: [Color; 11],
    pub axis_stroke_width: Option<f32>,        //
    pub axis_stroke_color: Option<Color>,      //
    pub separator_stroke_width: Option<f32>,   //
    pub separator_stroke_color: Option<Color>, //
    pub maximum_bar_width: f32,
    pub min_cluster_spacing: f32,
    pub bar_spacing: f32,
    pub x_marking_amount: Option<usize>, //
    pub y_marking_amount: Option<usize>, //
    pub max_value: Option<f32>,          //
    pub markings_color: Option<Color>,   //
    pub cluster_color: Option<Color>,    //
    pub headline: Option<String>,        //
    pub headline_color: Option<Color>,   //
    pub legend_alignment: Option<CoordinateSystemLegendAlignment>, //
    pub text_padding: Option<f32>,       //
    pub cluster_text_size: Option<f32>,  //
    pub headline_size: Option<f32>,      //
    pub markings_text_size: Option<f32>, //
}

impl BarChartStyleInfoIced {
    pub fn new() -> Self {
        //TODO: Create own own color palette. Colors taken from Qualitative Palette: https://experience.sap.com/fiori-design-web/values-and-names/
        BarChartStyleInfoIced {
            color_map: HashMap::new(),
            standard_color_map: STANDARD_COLOR_MAP,
            axis_stroke_width: None,
            axis_stroke_color: None,
            separator_stroke_width: None,
            separator_stroke_color: None,
            maximum_bar_width: 50.0,
            min_cluster_spacing: 5.0,
            bar_spacing: 2.0,
            x_marking_amount: Some(5),
            y_marking_amount: Some(5),
            max_value: None,
            markings_color: None,
            cluster_color: None,
            headline: None,
            headline_color: None,
            legend_alignment: None,
            text_padding: None,
            cluster_text_size: None,
            headline_size: None,
            markings_text_size: None,
        }
    }
}
