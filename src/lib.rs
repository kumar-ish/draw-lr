#![warn(missing_docs)]
//! Crate to make maps & interact with the [Line Rider](https://linerider.com) game.

use std::fmt::Debug;
use std::fs;

use serde::Serialize;

/// Coordinate system to represent vectors.
#[derive(Default, Serialize, Debug, Copy, Clone)]
pub struct Coordinates {
    x: f64,
    y: f64,
}

/// Riders (characters with snowboards) with some starting position/velocity.
#[derive(Default, Serialize, Debug, Copy, Clone)]
pub struct Rider {
    #[serde(rename = "startPosition")]
    start_position: Coordinates,
    #[serde(rename = "startVelocity")]
    start_velocity: Coordinates,
    remountable: usize,
}

/// Layers on the game.
#[derive(Default, Serialize, Debug)]
pub struct Layer {
    id: usize,
    name: String,
    visible: bool,
    editable: bool,
}

impl Layer {
    /// Default layer for the game.
    pub fn new() -> Self {
        Layer {
            id: 0,
            name: "Base Layer".to_string(),
            visible: true,
            editable: true,
        }
    }
}

/// Single line representation representation that stretches from (`x1`, `y1`) to (`x2`, `y2`)
/// on the 2D coordinate system of the game; and of type `kind`.
#[derive(Default, Serialize, Debug, Copy, Clone)]
pub struct Line {
    /// Game requires (unique) id for every line -- we make this an Option type so that the game handles the
    /// enumeration of lines passed to it
    id: Option<usize>,
    #[serde(rename = "type")] // JSON representation has "type", which is a reserved keyword in Rust
    kind: usize,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    /// Whether the line is flipped
    flipped: bool,
    #[serde(rename = "leftExtended")] // JSON representation requires camelCase; Rust requires snake_case
    left_extended: bool,
    #[serde(rename = "rightExtended")]
    right_extended: bool,
}

/// Version used for game.
#[derive(Serialize, Debug)]
pub struct Version(String);

/// Crate only tested version for default game -- can override.
impl Default for Version {
    fn default() -> Self {
        Self("6.2".into())
    }
}

/// Main representation of a Line Rider game.
#[derive(Default, Serialize, Debug)]
pub struct Game {
    label: String,
    creator: String,
    description: String,
    duration: usize,
    version: Version,
    audio: Option<String>,
    #[serde(rename = "startPosition")] // JSON representation requires camelCase; Rust requires snake_case
    start_position: Coordinates,
    riders: Vec<Rider>,
    layers: Vec<Layer>,
    lines: Vec<Line>,
}

impl Game {
    /// Creates a new version of the game that can be instantiated and have lines added to.
    pub fn new() -> Self {
        Game {
            label: "Track created by lr-rust".to_string(),
            creator: "lr-rust".to_string(),
            duration: 120,
            layers: Vec::from([Layer::new()]),
            ..Game::default()
        }
    }

    /// Add a singular line to the game.
    pub fn add_line(&mut self, line: &Line) {
        let line_with_id = Line {
            id: Some(self.lines.len() + 1),
            ..*line
        };
        self.lines.push(line_with_id);
    }

    /// Add several lines to the game.
    pub fn add_lines<'a, T: Iterator<Item = &'a Line>>(&mut self, lines: T) {
        for line in lines {
            self.add_line(line);
        }
    }

    /// Add a singular rider to the game.
    pub fn add_rider(&mut self, rider: &Rider) {
        self.riders.push(*rider);
    }

    /// Add several riders to the game.
    pub fn add_riders<'a, T: Iterator<Item = &'a Rider>>(&mut self, riders: T) {
        for rider in riders {
            self.add_rider(rider);
        }
    }

    /// Construct JSON representation of game that can be imported.
    pub fn construct_game(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    /// Writes the JSON representation to a given filename.
    pub fn write_to_file(&self, filename: &str) -> std::io::Result<()> {
        fs::write(filename, self.construct_game())?;
        Ok(())
    }
}

/// Extension definitions and functions to create Line Rider maps with.
pub mod extension {
    use std::ops::Range;

    use crate::*;

    /// Options for passing coordinates to functions.
    #[derive(Clone, Copy)]
    pub enum CoordOptions {
        /// Random coordinates in a default range
        Rand,
        /// Random coordinates over range
        RandRange(Coordinates, Coordinates),
        /// Specify exact coordinates (Some) or use default (None)
        Other(Option<Coordinates>),
        /// Evenly space out coordinates over range
        EvenlySpaced(Coordinates, Coordinates),
    }

    /// Adds `n` riders to a given `game`, at some `start_position` and `speed_range`; all with characteristic `remountable`.
    pub fn create_riders(n: usize, start_position: CoordOptions, speed_range: CoordOptions, remountable: Option<usize>) -> Vec<Rider> {
        fn check_min_max(min: f64, max: f64) {
            // Min/max checks might not be strictly necessary, but may prevent bugs.
            if max < min {
                panic!("Max ({:.}) is less than min ({:.})", max, min);
            }
        }
        fn even_spaced_rider(min: Coordinates, max: Coordinates, i: usize, n: usize) -> Coordinates {
            check_min_max(min.x, max.x);
            check_min_max(min.y, max.y);

            Coordinates {
                x: min.x + i as f64 * (max.x - min.x) / (n - 1) as f64,
                y: min.y + i as f64 * (max.y - min.y) / (n - 1) as f64,
            }
        }
        /// Gets a random number between range.
        fn coord_between(min: f64, max: f64) -> f64 {
            check_min_max(min, max);
            min + rand::random::<f64>() * (max - min)
        }

        fn match_coords(coordinates: CoordOptions, i: usize, n: usize) -> Option<Coordinates> {
            match coordinates {
                CoordOptions::Rand => Some(Coordinates {
                    x: coord_between(-10.0, 10.0),
                    y: coord_between(-10.0, 10.0),
                }),

                CoordOptions::RandRange(min, max) => Some(Coordinates {
                    x: coord_between(min.x, max.x),
                    y: coord_between(min.y, max.y),
                }),

                CoordOptions::EvenlySpaced(min, max) => Some(even_spaced_rider(min, max, i, n)),
                CoordOptions::Other(x) => x,
            }
        }

        let mut riders: Vec<Rider> = Vec::new();

        for i in 0..n {
            let start_position: Option<Coordinates> = match_coords(start_position, i, n);
            let start_velocity: Option<Coordinates> = match_coords(speed_range, i, n);

            riders.push(Rider {
                start_position: start_position.unwrap_or_default(),
                start_velocity: start_velocity.unwrap_or_default(),
                remountable: remountable.unwrap_or_default(),
            });
        }

        riders
    }

    /// Creates a polygon with given characteristics.
    /// As sides -> \inf, the function can better approximate a circle.
    pub fn polygon_lines(sides: u16, radius: u16, start_position: Option<Coordinates>, rotation: Option<f64>, kind: usize) -> Vec<Line> {
        let center = start_position.unwrap_or_default();

        let mut polygon_lines: Vec<Line> = Vec::new();

        let vertex_degree: f64 = std::f64::consts::TAU / sides as f64;
        let initial_angle: f64 = vertex_degree / 2_f64 + rotation.unwrap_or_default();

        // We'll use a sliding window from the first point that starts (vertex_angle) / 2 clockwise from 0° until we've gone around 360°.
        let mut first_point: Coordinates = Coordinates {
            x: center.x + radius as f64 * initial_angle.cos(),
            y: center.y + radius as f64 * initial_angle.sin(),
        };

        for i in 1..sides + 1 {
            let second_point: Coordinates = Coordinates {
                x: center.x + radius as f64 * (initial_angle + i as f64 * vertex_degree).cos(),
                y: center.y + radius as f64 * (initial_angle + i as f64 * vertex_degree).sin(),
            };

            polygon_lines.push(Line {
                id: None,
                kind,
                x1: first_point.x,
                y1: first_point.y,
                x2: second_point.x,
                y2: second_point.y,
                flipped: true,
                left_extended: true,
                right_extended: true,
            });

            first_point = second_point;
        }

        polygon_lines
    }

    /// Creates lines to sketch out polygon with a given thickness (see `polygon_lines`).
    pub fn thick_polygon_lines(
        sides: u16,
        radius: u16,
        start_position: Option<Coordinates>,
        rotation: Option<f64>,
        thickness: u16,
        kind: usize,
    ) -> Vec<Line> {
        let mut single_polygon_lines: Vec<Line> = Vec::new();

        for i in 0..thickness {
            single_polygon_lines.extend(polygon_lines(sides, radius + i, start_position, rotation, kind));
        }

        single_polygon_lines
    }

    /// Creates and returns lines to sketch out a function `func` over a given range `range`;
    /// with n `iterations` done over integer steps. All lines created will be of type `kind`.
    pub fn function_lines(func: fn(f64) -> f64, range: Range<i64>, iterations: Option<u8>, kind: Option<usize>) -> Vec<Line> {
        let mut function_lines: Vec<Line> = Vec::new();
        let num_iterations = iterations.unwrap_or(10);

        let mut last_x: f64 = range.start as f64;
        let mut last_y: f64 = func(last_x);

        for i in range {
            for j in (1..num_iterations).rev() {
                // Approximate divisions between integer units
                let x = i as f64 + (j as f64 / num_iterations as f64);
                let y = func(x);
                function_lines.push(Line {
                    kind: kind.unwrap_or(1),
                    x1: last_x,
                    y1: last_y,
                    x2: x,
                    y2: y,
                    ..Line::default()
                });
                last_x = x;
                last_y = y;
            }
        }

        function_lines
    }
}
