##Target: 
- Make data driven game_setup
- Make a data format for world setup. To make it easy at first let target for json format
- It should support world size, Entities placement in interger
- Entities type for now
    - box
    - bird
- See the _ai/ref/level_01.png for reference level design 
    - Black is box , Red like is bird
- The coordinate in file is top left = (0,0)

- When spawn a level in game_setup. Should following the json coordinate
    - please consider to make a scale factor in case json world too small
    - camera should snap to bottowm ground 
    - ground should always in the bottom
- should capture the SLINGSHOT_ANCHOR dynamic which used in bird_slingshot system

