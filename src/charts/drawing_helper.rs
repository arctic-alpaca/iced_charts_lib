use crate::charts::coordinate_system::SpaceCalculator;
use iced::canvas::Frame;
use iced::Point;

pub trait DrawingHelper {
    fn get_x_axis_positive_length(&self) -> f32;
    fn get_x_axis_negative_length(&self) -> f32;
    fn get_y_axis_positive_length(&self) -> f32;
    fn get_y_axis_negative_length(&self) -> f32;
    fn get_origin(&self) -> Point;
    fn get_space_calculator(&self) -> SpaceCalculator;
    fn get_cluster_amount(&self) -> f32;
}

pub struct BarChartDrawingHelper {
    cluster_amount: f32,
    amount_of_bars_in_per_cluster: f32,
    bar_width: f32,
    cluster_spacing: f32,
    x_axis_positive_length: f32,
    x_axis_negative_length: f32,
    y_axis_positive_length: f32,
    y_axis_negative_length: f32,
    origin: Point,
    space_calculator: SpaceCalculator,
}

impl BarChartDrawingHelper {
    pub fn new(cluster_amount: f32, amount_of_bars_in_per_cluster: f32) -> Self {
        BarChartDrawingHelper {
            cluster_amount,
            amount_of_bars_in_per_cluster,
            bar_width: 0.0,
            cluster_spacing: 0.0,
            x_axis_positive_length: 0.0,
            x_axis_negative_length: 0.0,
            y_axis_positive_length: 0.0,
            y_axis_negative_length: 0.0,
            origin: Point::new(0.0, 0.0),
            space_calculator: SpaceCalculator::new(),
        }
    }
    pub fn populate_axis_and_origin(&mut self, axis_data: (f32, f32, f32, f32, Point)) {
        self.x_axis_positive_length = axis_data.0;
        self.x_axis_negative_length = axis_data.1;
        self.y_axis_positive_length = axis_data.2;
        self.y_axis_negative_length = axis_data.3;
        self.origin = axis_data.4;
    }
    pub fn populate_space_calculator(&mut self, space_calculator: SpaceCalculator) {
        self.space_calculator = space_calculator;
    }
    pub fn populate_bar_width_and_cluster_spacing(
        &mut self,
        bar_width_and_cluster_spacing: (f32, f32),
    ) {
        self.bar_width = bar_width_and_cluster_spacing.0;
        self.cluster_spacing = bar_width_and_cluster_spacing.1;
    }
    pub fn get_amount_of_bars_in_per_cluster(&self) -> f32 {
        self.amount_of_bars_in_per_cluster
    }
}
impl DrawingHelper for BarChartDrawingHelper {
    fn get_x_axis_positive_length(&self) -> f32 {
        self.x_axis_positive_length
    }
    fn get_x_axis_negative_length(&self) -> f32 {
        self.x_axis_negative_length
    }
    fn get_y_axis_positive_length(&self) -> f32 {
        self.y_axis_positive_length
    }
    fn get_y_axis_negative_length(&self) -> f32 {
        self.y_axis_negative_length
    }
    fn get_origin(&self) -> Point {
        self.origin
    }
    fn get_space_calculator(&self) -> SpaceCalculator {
        self.space_calculator
    }
    fn get_cluster_amount(&self) -> f32 {
        self.cluster_amount
    }
}
