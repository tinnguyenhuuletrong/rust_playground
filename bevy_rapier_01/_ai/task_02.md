Target: 
- Trying to make a determinism physic simulation by using bevy + rapier2d
- It should output same simulation state on web and desktop 

Document: 
- https://docs.rs/bevy_rapier2d/latest/bevy_rapier2d/

Task 
- Add new system check all physic entities sleep, world no more update then calculate sha256 for world state
- Print it into gui with format "Physic simulation steps: {number of simulation step so far}. hash {sha256}"
