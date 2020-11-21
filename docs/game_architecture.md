# Architecture

An overview of what components (and sub-components) make up Space Station 13
for the purpose of allowing versatile modding without sacrificing literally
all structure and by extension, performance.

## Components

- Users
  - Can be admins, spectators, or normal players
    - Normal players are the majority of a server's population
    - Admins can:
      - manage players (whitelisting and blacklisting)
      - stages/assets
      - server config
    - Spectators can view everything happening but interact with nothing
- Players
  - Have an inventory
    - A set of items that
- Tiles
  - Represents floors and walls, not what is on a floor
    - Floors:
      - are a specific texture or animation w/ an orientation
      - can have a stack of effects: (must have a definite order)
        - water spills that affect movement?
        - blood spills
        - poison that hurts players
    - Walls:
      - Typically represent either doors or normal walls
