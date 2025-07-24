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


##Iter 02
###Target 
 - Read whole source first. make sure you understand what 's exists
 - rework on json file coordinate system describle the world. it should easy to mapping from pixel coordinate. Later we are going to create a tool generate it from image like _ai/ref/level_01.png
 - reworking on the game_setup addapting to new coordinate system
   - ground bottom, left, right calculated base on the json 's world
   - camera should auto adjust make sure the bottom ground at the bottom of the window screen
 - try to visualize coordinate as grid using gizmo

###Plan
- Update `level.rs` to change the `LevelData` struct. The new structure uses `world_bound` (an array representing left, top, right, bottom) and `ground_y` to define the level's geometry, which is more suitable for a top-left origin coordinate system.
- Modify `assets/data/level_01.json` to conform to the new `LevelData` structure. This includes defining the world boundaries, the ground's y-position, and updating all entity coordinates to be relative to the top-left corner (0,0).
- Rework the `game_setup` function in `src/game.rs`:
    - Adapt the logic to use the new `world_bound` and `ground_y` from the JSON file to dynamically calculate the positions of the ground and walls, removing hardcoded values.
    - Implement automatic camera adjustment. The camera's projection is set to `ScalingMode::FixedVertical` and its position is calculated to ensure the entire level is visible, with the ground positioned correctly at the bottom of the screen.
    - Translate entity positions from the top-left coordinate system in the JSON file to Bevy's center-origin world coordinate system.
    - Add a grid visualization using `gizmos` to help debug and understand the new coordinate system.
- Update the `game_state_control` system in `src/systems.rs` to correctly pass the `Gizmos` resource when resetting the game.
- Correct a calculation error in `game.rs` for the ground's y-position to fix a visual bug where objects were falling through the world.

###Result
- Successfully refactored the level data format to use a top-left origin coordinate system, making it more intuitive for future level design tools.
- The game now correctly loads and interprets the new level format from `assets/data/level_01.json`.
- `game_setup` dynamically constructs the level, including walls and ground, based entirely on the loaded level data.
- The camera now automatically adjusts its view to frame the entire level, with the ground properly aligned to the bottom of the viewport.
- A visual grid, drawn with gizmos, has been added to aid in debugging and visualizing the coordinate system.
- Fixed a critical bug in the ground placement calculation, ensuring a solid and correctly positioned play area.
