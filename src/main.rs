use std::collections::VecDeque;

use file_obtainer::ConfigurationBuilder;
use offset_calculator::OffsetPlotter;
use plotter::definitions::PlotterBuilder;

mod file_obtainer;
mod offset_calculator;
mod plotter;

fn main() {

    let mut args = std::env::args().collect::<VecDeque<String>>();
    
    if args.len() < 2 {
        eprintln!("Usage: nester <file_location> <optional: file_endings, .cs .rs>\nExample: nester ./results/out.png .cs .rs .cpp");
    }

    // Discard program name
    _ = args.pop_front();

    let root_dir = args.pop_front().unwrap();
    let mut file_endings = Vec::new();
    while let Some(file_ending) = args.pop_front() {
        file_endings.push(file_ending);
    }

    let config = ConfigurationBuilder::new(root_dir)
        .add_file_endings(file_endings)
        .build();

    let file_paths = file_obtainer::get_files_in_directory(&config);
    let offsets = offset_calculator::get_code_offsets(&file_paths);

    let plotter = PlotterBuilder::new("./out.png".to_string()).build();

    plotter.plot_offsets(offsets);
}