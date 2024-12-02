# Slate Surface

### How to Run

```sh
cargo run -p slate-surface --features dev
```

## Features:

### Tools:

#### 1. Selection:

-   Single-click on a sprite to single-select its entity.
-   Shift + Single-click on a sprite to add it to the active selection.
-   Drag selection marquee to multi-select entities within its bounds.

#### 2. Sprite Painter:

-   Click and drag to "paint" sprites anywhere on the artboard. The resulting entity is placed in the scene and is selectable, movable, etc.
-   Click and drag directly on an existing painted sprite to modify it.
-   Modes: "Add" and "Subtract" for "shaping" painted sprites.
-   Output is affected by Reticle shape/size.

#### 3. Shape Composer:

-   Click and drag to spawn a 2d shape anywhere on the artboard. The resulting entity is placed in the scene and is selectable, movable, etc.
