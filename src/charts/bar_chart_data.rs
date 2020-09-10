use crate::charts::error::ChartsLibError;

#[derive(Debug, Clone)]
pub struct Dataset {
    pub id: u32,
    pub name: String,
    pub data: Vec<f32>,
}

#[derive(Debug)]
pub struct BarChartData {
    pub datasets: Vec<Dataset>,
}

impl Dataset {
    pub fn new(id: u32, name: String, data: Vec<f32>) -> Self {
        Dataset { id, name, data }
    }
}

impl BarChartData {
    pub fn new(datasets: Vec<Dataset>) -> Self {
        BarChartData { datasets }
    }

    pub fn get_biggest_amount_of_data_entries_in_one_dataset(&self) -> usize {
        let mut result = 0;
        for dataset in &self.datasets {
            if result < dataset.data.len() {
                result = dataset.data.len();
            }
        }
        result
    }

    pub fn get_biggest_data_entry_abs(&self) -> f32 {
        let mut result = 0.0;
        for dataset in &self.datasets {
            for entry in &dataset.data {
                if result < entry.abs() {
                    result = entry.abs();
                }
            }
        }
        result
    }
}
