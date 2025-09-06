- All:
    - type: String
    - Position: (u32, u32)
    - ObjectId: String
    - enabled: bool
    - mutable: bool

- Structures
    - Stairs:
        - field size: u32
        - field orientation: {-1, 1}
    - Platform:
        - field size: (u32, u32)
- Enemies
    - Bird
    - Pig
    - Snail
    - Beartrap

    none of these have additional fields
- Collectables
    - FlagPole
    - Grain
    - GrowPowerup
    - OneUp
    - RedBull

    none of these have additional fields
- Tiles
    - Ground
    - Grass
    - Mud
    - Ice
    - Wall
    - Powerup:
        - field collectableClass: String