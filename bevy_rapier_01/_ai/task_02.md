##Target: 
- Trying to make a determinism physic simulation by using bevy + rapier2d
- It should output same simulation state on web and desktop 

##Document: 
- https://docs.rs/bevy_rapier2d/latest/bevy_rapier2d/

##Task 
- Add new system check all physic entities sleep, world no more update then calculate sha256 for world state
- Print it into gui with format "Physic simulation steps: {number of simulation step so far}. sha256: {sha256}"


##Iter 01
###Plan
1. Add a SHA-256 Crate: I'll add the sha2 crate to the project's dependencies in Cargo.toml to handle the hashing.
2. Create a State Resource: I'll introduce a new Bevy resource to store the simulation step count and the resulting SHA-256 hash.
3. Implement a Verification System: I will create a new system that runs every frame to:
    * Check if all physics bodies in the simulation are sleeping using the RapierContext.
    * If they are, it will construct a string containing the transform data (position and rotation) of every dynamic body.
    * It will then compute a SHA-256 hash of this string.
    * Finally, it will update the state resource with the new hash and the current step count.
4. Update the UI: I will add a new text element to the UI and a system to keep it updated with the information from the state resource.

###Problem
- The text Physic simulation steps should move to bottom right. align right and smaller
- physic_state.step should stop counting when all world sleeped

##Iter 02
###Plan
1.  **Adjust UI Text**: I will modify the `Style` component for the physics state text in the `ui_setup` function to move it to the bottom-right corner of the screen and reduce its font size.
2.  **Correct Step Counting**: I will update the `physic_step_hash_check` system to ensure that the `physic_state.step` counter only increments *before* the physics world has come to rest (i.e., before the hash is calculated).

###Result
- The UI text is now correctly positioned and sized.
- The step counter now stops incrementing once the physics world is asleep.

###Problem
- When restart with R. the  Physic simulation steps disappear need to fix it

##Iter 03
###Plan
1.  **Analyze the problem**: The UI text disappears on restart because the `PhysicsState` resource is reset, causing the `update_physic_state_text` system to render an empty string.
2.  **Propose a solution**: I will modify the `update_physic_state_text` system. Instead of showing an empty string when the hash is not available, it will display the current simulation step count. This will provide continuous feedback to the user.
3.  **Implement the fix**: I will apply this logic change to the `update_physic_state_text` function in `src/lib.rs`.

###Result
- The physics state text now correctly displays the step count during the simulation and the final hash when the simulation is complete.