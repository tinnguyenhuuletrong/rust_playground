##Target: 
- Make data driven game_setup
- Make a data format for world setup. To make it easy at first let target for json format
- It should support world size, Entities placement in interger
- Entities type for now
    - box
    - bird
- See the _ai/ref/level_01.png for reference level design 
    - Black is box , Red like is bird
- The word coordinate should following the image coordinate. 

- When spawn a level in game_setup. Should following the image coordinate
    - please consider to make a scale factor since the image is small
    - camera should snap to bottowm ground 
    - ground should always in the bottom
- should capture the SLINGSHOT_ANCHOR dynamic which used in bird_slingshot system


##Iter 01
###Plan
- Define a JSON structure for world data (`world.json`). It will contain:
    - World dimensions (width, height).
    - A list of entities, each with:
        - `type`: "box" or "bird".
        - `position`: [x, y].
- Create a sample `assets/data/world.json` file.
- Add `serde` and `serde_json` dependencies to `Cargo.toml` for deserialization.
- Create Rust structs in `src/resources.rs` that correspond to the JSON structure, deriving `serde::Deserialize`.
- Refactor the `game_setup` function in `src/game.rs` to:
    - Load the `world.json` file.
    - Parse the JSON data into the newly defined Rust structs.
    - Set up the game world (e.g., camera, physics) based on the loaded data.
    - Spawn entities (boxes, birds) in the world according to the data from the JSON file.