# ggez-game-of-life
Conway's Game of Life in RUST with ggez

### demo [recorded at lower FPS, actul performance will be much much higher]
![game-of-life-rust_2](https://user-images.githubusercontent.com/3184210/82114357-03932280-977a-11ea-8787-c08ad77685ed.gif)

### setup
```
cargo check
```

## Build
```
cargo build --release; .\target\release\ggez-game-of-life.exe
```

By default the alive percentage is set to 15 `cells: Self::generate_cells(15, cols, rows),`

### Shortcuts
 - Click = adds a single cell
 - CTRL + CLICK = Adds a Glider
 - SHIFT + CLICK = Adds a Exploder
 - ALT + CLICK = Adds a Juggler
 
