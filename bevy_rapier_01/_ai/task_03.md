##Target: 
- Review and organize source
- This project should support desktop bin and webasm build target. which need different entry file

##Problem:
- the lib.rs is quite big right now

##Task 
- Analyze the source + refactor it

##Iter 01
###Plan
1.  **Analyze Project Structure**: Review `Cargo.toml`, `src/main.rs`, and `src/lib.rs` to understand the existing project setup, dependencies, and build configurations for both desktop and WASM targets.
2.  **Create New Modules**: Create a set of new files to logically separate the codebase:
    *   `src/components.rs`: For struct definitions of Bevy components.
    *   `src/resources.rs`: For struct definitions of Bevy resources.
    *   `src/systems.rs`: For general gameplay systems.
    *   `src/ui.rs`: For UI-related setup and systems.
    *   `src/physics.rs`: For physics-related systems.
    *   `src/game.rs`: For the main game setup logic.
3.  **Relocate Code**: Move the existing code from the monolithic `src/lib.rs` file into the appropriate new modules.
4.  **Update `lib.rs`**: Modify `src/lib.rs` to declare the new modules and import the necessary functions and structs, serving as the main library file that ties everything together.
5.  **Verify Changes**: Run `cargo check` and `cargo run` to ensure that the refactored code compiles and runs without errors.

###Result
- The source code is now successfully modularized, with components, resources, systems, and setup logic separated into their own files.
- `src/lib.rs` is significantly cleaner and primarily serves to wire up the different modules and plugins.
- The project structure is more organized and scalable for future development.
- The desktop application compiles and runs as expected after the refactoring.
