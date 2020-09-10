#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub enum AxisOrientation {
    XPositiveYPositive,
    XPositiveYNegative,
    XPositiveYComplete,
    XNegativeYPositive,
    XNegativeYNegative,
    XNegativeYComplete,
    XCompleteYPositive,
    XCompleteYNegative,
    XCompleteYComplete,
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub enum XAxisOrientation {
    Positive,
    Negative,
    Complete,
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub enum YAxisOrientation {
    Positive,
    Negative,
    Complete,
}
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub enum BarChartDataAxis {
    XPositive,
    XNegative,
    YPositive,
    YNegative,
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub enum BarChartDataPositivity {
    Positive,
    Negative,
    Both,
}
