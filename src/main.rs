fn main() {
    use draw_lr::*;
    use draw_lr::extension::*;
    
    let mut default_game = Game::new();

    // let polygon_lines = thick_polygon_lines(3, 80, None, None, 1);
    // default_game.add_lines(polygon_lines);
    fn offset_sin(num: f64) -> f64 {
        50_f64 + 100f64 * f64::sin(0.01 * num)
    }
    let sine_lines = function_lines(offset_sin, -1000..100000, None, None);
    default_game.add_lines(sine_lines.iter());

    let polygon = thick_polygon_lines(10, 40, None, None, 1, 1);
    default_game.add_lines(polygon.iter());

    let riders: Vec<Rider> = create_riders(
        // Number of riders
        4,
        // Start pos
        CoordOptions::Rand,
        // Start speed
        CoordOptions::Rand,
        // Remountable
        None,
    );
    default_game.add_riders(riders.iter());

    default_game.write_to_file("track.json").ok();
}
