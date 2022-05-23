# Line Rider SDK

This is a library to be able to build on [Line Rider](https://www.linerider.com/); note that this is to interact with the original game, and not an extension. 

## Usage

You can use the crate by instantiating a game (of type `draw_lr::Game`) and adding pieces on top of it (lines, layers, riders) using the crate. You can then write the game to a track JSON file and play with that on the game by uploading it.

```rust
fn main() {
    use draw_lr::*;
    use draw_lr::extension::*;
    
    let mut game = Game::new();

    let polygon = thick_polygon_lines(10, 40, None, None, 1, 1);
    game.add_lines(polygon.iter());

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
    game.add_riders(riders.iter());

    game.write_to_file("decagon.track.json").ok();
}
```