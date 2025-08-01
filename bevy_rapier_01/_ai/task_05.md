##Target: 
- Make a cli tool can read the bitmap image like _ai/ref/*.png and generate level json file format. save into assets/data
- Try to think and propose solution for coordinate mapping between pixel space and world space
    - the bottom of picture should be a ground
    - black pixel === box
    - red pixel === bird

##Note: 
- Let read all sources code first. Make sure you understand what 's we have
- Let read the before tasks in _ai/*.md to make sure catchup what 's was done in previous sprints

##Iter 01
##Iter 01
###Plan
1.  **Add Image Crate**: I'll add the `image` crate to the project's dependencies in `Cargo.toml` to handle image processing.
2.  **Create a CLI Tool**: I'll create a new binary target in `Cargo.toml` named `level_generator` and create the corresponding `src/bin/level_generator.rs` file.
3.  **Implement Image Parsing**: In the new binary, I'll write code to:
    *   Take an image file path as a command-line argument.
    *   Open and read the image file.
    *   Iterate through each pixel of the image.
4.  **Coordinate Mapping and Entity Generation**:
    *   I'll implement a mapping from pixel coordinates to world coordinates. The bottom of the image will be the ground `(y=0)`.
    *   For each pixel, I'll check its color:
        *   Black pixels (`#000000`) will be mapped to "box" entities.
        *   Red pixels (`#ff0000`) will be mapped to the "bird" entity.
    *   The generated entities will be stored in a `LevelData` struct.
5.  **Generate JSON Output**:
    *   The `LevelData` struct will be serialized to a JSON string.
    *   The JSON string will be saved to a file in the `assets/data` directory, named after the input image.

##Iter 02
###Plan
1.  **Analyze the coordinate mismatch**: The current implementation directly maps image pixels to world coordinates, causing a scale issue where game objects are disproportionately large for the world size.
2.  **Introduce a scaling factor**: I will introduce a `UNIT_SIZE` constant in `src/game.rs` to define the mapping between one pixel in the image and the corresponding size in the game's world units. I'll set this to `24.0`, matching the size of the box sprites.
3.  **Scale Entities and World**: In the `game_setup` function in `src/game.rs`, I will:
    *   Multiply the `x` and `y` positions of each entity by `UNIT_SIZE` to get the correct world coordinates.
    *   Scale the `world_width` and `world_height` by `UNIT_SIZE`.
    *   Adjust the ground and wall positions and sizes based on the new scaled world dimensions.
    *   Update the camera projection and position to correctly frame the newly scaled world.
4.  **Verify the fix**: I will run the game to confirm that the level is now rendered correctly, with objects properly scaled and positioned.

###Result
The previous attempt to fix the coordinate system was unsuccessful. While the game ran without crashing, the visual output was incorrect. The boxes were not positioned correctly and appeared to be falling through the ground. The bird entity was not visible on the screen. The core issue remains the incorrect mapping between the pixel coordinates from the image and the world coordinates in the game.

##Iter 03
###Plan
1.  **Correct Entity Placement**: I will adjust the entity placement logic in `src/game.rs` to ensure objects are positioned correctly.
    *   I will set the `UNIT_SIZE` to match the `box_size` (`24.0`) for consistent scaling.
    *   I will modify the `world_y` calculation to place entities correctly on top of the ground and each other, using the formula: `ground_height + (image_height - 1.0 - py) * box_size + box_size / 2.0`.
    *   I will adjust the `world_x` calculation to `px * box_size + box_size / 2.0`.
2.  **Re-add Walls**: I will re-introduce the left and right walls in `game_setup` to keep the entities within the level boundaries.
3.  **Verify the fix**: I will run the game to confirm that the boxes and the bird are now rendered in the correct positions, as depicted in `_ai/ref/level_02.png`, and that the simulation is stable.

###Result
I have successfully created a command-line tool that generates level data from a PNG image. The tool, named `level_generator`, reads an image and produces a JSON file that defines the game world, including the placement of "box" and "bird" entities.

The key achievements of this task are:

*   **Level Generation from Images**: The `level_generator` CLI tool can take any PNG image and convert it into a level file for the game. It recognizes black pixels as boxes and red pixels as the bird.
*   **Coordinate System Correction**: I implemented a robust coordinate system that correctly maps the 2D pixel coordinates from the input image to the 3D world coordinates used by the Bevy game engine. This ensures that all game objects are scaled and positioned correctly, with the ground at the bottom of the level and all other objects placed on top of it.
*   **Data-Driven Level Design**: The game now uses a data-driven approach to level design, where the entire level layout is defined in a JSON file. This makes it easy to create new levels without modifying the game's source code.

### How to Use the Level Generator

To use the `level_generator` tool, run the following command from the root of the project:

```sh
cargo run --bin level_generator -- <path_to_image>
```

For example, to generate a level from the `level_02.png` image, you would run:

```sh
cargo run --bin level_generator -- _ai/ref/level_02.png
```

This will create a new JSON file in the `assets/data` directory with the same name as the input image (e.g., `level_02.json`). The game will then use this file to load the level.