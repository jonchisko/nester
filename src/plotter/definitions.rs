use plotly::{Plot, Scatter, color::Rgba, common::{Fill, Line, LineShape}, layout::Axis, Layout, ImageFormat};

use crate::offset_calculator::{OffsetPlotter, LineOffsets};

pub const START_OPACITY: f64 = 0.2f64;

pub const RESULT_DIMENSION: ResultDimension = ResultDimension(800, 600);
pub const SMOOTHNESS: f64 = 1.0;

#[derive(Clone, Copy)]
pub struct ResultDimension(pub usize, pub usize);

pub struct Plotter {
    result_location: String,
    dimension: ResultDimension,
    smoothing: f64,
}

impl Plotter {
    fn new(result_location: String, dimension: ResultDimension, smoothing: f64) -> Plotter {
        Plotter {result_location, dimension, smoothing}
    }

    fn get_trace(&self, offset: &LineOffsets, opacity_computer: &mut OpacityComputerIterator) -> Box<plotly::Scatter<usize, u16>> {
        let opacity = opacity_computer.next().expect("Should be Some, but is None. This means, the element is above max number of elements. Peculiar.");
        let color = Rgba::new(50, 50, 200, opacity / 1.0f64);
    
        let line = Line::new()
            .shape(LineShape::Spline)
            .color(color)
            .smoothing(self.smoothing);
    
        let trace = Scatter::new((0..offset.len()).collect::<Vec<usize>>(), offset.clone())
            .fill_color(color)
            .fill(Fill::ToZeroY)
            .show_legend(false)
            .line(line);
    
        trace
    }
}

pub struct PlotterBuilder {
    result_location: String,
    dimension: Option<ResultDimension>,
    smoothing: Option<f64>,
}

impl PlotterBuilder {
    pub fn new(result_location: String) -> PlotterBuilder {
        PlotterBuilder { result_location, dimension: None, smoothing: None }
    }

    pub fn with_dimension(mut self, dimension: ResultDimension) -> PlotterBuilder {
        self.dimension = Some(dimension);
        
        self
    }

    pub fn with_smoothing(mut self, smoothing: f64) -> PlotterBuilder {
        self.smoothing = Some(smoothing);
        
        self
    }

    pub fn build(self) -> Plotter {
        Plotter::new(
            self.result_location, 
            self.dimension.unwrap_or(RESULT_DIMENSION),
            self.smoothing.unwrap_or(SMOOTHNESS)
        )
    }
}

impl OffsetPlotter for Plotter {
    fn plot_offsets(&self, offsets: Vec<LineOffsets>) {
        let mut plot = Plot::new();

        let mut opacity_computer = OpacityComputer::new(START_OPACITY, offsets.len()).into_iter();
        
        for offset in offsets.iter() {
            let trace = self.get_trace(offset, &mut opacity_computer);
    
            plot.add_trace(trace);
        }
        set_plot_layout(&mut plot);

        plot.write_image(
            &self.result_location, 
            ImageFormat::PNG, 
            self.dimension.0, 
            self.dimension.1, 
            1.0
        );
    }
}

fn set_plot_layout(plot: &mut Plot) {
    let layout = Layout::new()
        .x_axis(Axis::new().visible(false))
        .y_axis(Axis::new().visible(false));

    plot.set_layout(layout);
}

struct OpacityComputer {
    start_opacity: f64,
    opacity_step: f64,
    elements: usize,
}

impl OpacityComputer {
    pub fn new(start_opacity: f64, elements: usize) -> OpacityComputer {
        assert!(start_opacity <= 1.0, "start_opacity is above 1.0, but should not be.");

        let to_divide = 1.0 - start_opacity;
        let opacity_step = to_divide / elements as f64;

        OpacityComputer {start_opacity: start_opacity, opacity_step: opacity_step, elements}
    }
}

impl IntoIterator for OpacityComputer {
    type Item = f64;
    type IntoIter = OpacityComputerIterator;

    fn into_iter(self) -> Self::IntoIter {
        OpacityComputerIterator { opacity_computer: self, index: 0 }
    }
}

struct OpacityComputerIterator {
    opacity_computer: OpacityComputer,
    index: usize,
}

impl Iterator for OpacityComputerIterator {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.opacity_computer.elements {
            Some(self.opacity_computer.start_opacity + self.index as f64 + self.opacity_computer.opacity_step)
        } else {
            None
        }
    }
}