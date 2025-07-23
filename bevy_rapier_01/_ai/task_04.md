##Target: 
- Make data driven game_setup
- Make a data format for world setup. To make it easy at first let target for json format
- It should support world size, Entities placement in interger, Coordinate within the file is top left = (0,0)
- Entities type for now
    - box
    - bird

- See the _ai/ref/level_01.png for reference level design 
    - Black is box , Red like is bird

##Iter 01
###Plan
- Read whole source first. make sure you understand what 's exists
- Create data structures for level data (LevelData, EntityData) in a new file `src/level.rs`.
- Add `serde` and `serde_json` to `Cargo.toml` for JSON deserialization.
- Create a `level_01.json` file in `assets/data` to define the level layout.
- Modify `game_setup` in `src/game.rs` to load the JSON file and spawn entities based on its data.
- Create a `BirdStart` resource in `src/resources.rs` to store the bird's starting position from the level data.
- Initialize the `BirdStart` resource in `src/lib.rs`.
- Update `bird_slingshot` and `game_state_control` in `src/systems.rs` to use the `BirdStart` resource.

###Result
- Successfully implemented a data-driven level setup using a JSON file.
- The game now loads the level layout, including world size and entity placements, from `assets/data/level_01.json`.
- The `game_setup` function dynamically spawns boxes and the bird based on the loaded data.
- The bird's starting position is now managed by a `BirdStart` resource, making it configurable through the level data.