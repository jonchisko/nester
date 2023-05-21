use std::{path::PathBuf, fs::File, io::{BufReader, BufRead}};

pub trait OffsetPlotter {
    fn plot_offsets(&self, offsets: Vec<LineOffsets>);
}

pub type LineOffsets = Vec<u16>;

pub fn get_code_offsets(file_paths: &Vec<PathBuf>) -> Vec<LineOffsets> {
    let mut files_line_offsets: Vec<LineOffsets> = Vec::new();

    for file_path in file_paths {
        files_line_offsets.push(get_offset(&file_path))
    }

    files_line_offsets
}

fn get_offset(file_path: &PathBuf) -> LineOffsets {
    let file = File::open(file_path).unwrap();
    let mut reader = BufReader::new(file);

    let mut line_offsets = LineOffsets::new();
    let mut line = String::new();

    while let Ok(number_of_bytes) = reader.read_line(&mut line) {
        if number_of_bytes == 0 {
            break;
        }

        let mut offset = 0u16;
        for (i, ch) in line.chars().enumerate() {
            if ch != ' ' {
                offset = i as u16;
                break;
            }
        }

        line_offsets.push(offset);

        line.clear();
    }

    line_offsets
}