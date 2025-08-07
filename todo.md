### TODOs:
- [x] Camera code is weird, owning 2 different cameras
- [x] add undo/redo feature
- [x] implement fill area
- [x] add different modes (brush, select for enemies, sections)
- [x] add metadata to Tiles (e.g. name, enemy speed etc.)
<!-- - ~~[ ] save Tiles as sections~~
    - ~~automatically add tile to a section if neighboring sh~~
    - ~~else create a new section with that tile~~
    - ~~union of section if they are connected~~
    - ~~vertices describing their location and shape~~
    - ~~when trying to select a block in a section, immediately select the whole thing~~
<!-- - ~~[ ] replace ground/grass with "Terrain" and use special rendering rules for terrain~~ -->
- [ ] tiles start as rects not squares
- [ ] add level floor (immutable except for gaps)
    - level floor is a 
- [ ] add gaps, which can be placed and resized, destroying ground tiles
- [ ] add other tile types (ice, mud, walls, enemies)