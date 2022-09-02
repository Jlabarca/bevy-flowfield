#[derive(Component)]
struct Tile {
    pub position: Vec2,
    pub visible: bool,
}
#[derive(Component)]
struct Tile {
    pub position: Vec2,
    pub visible: bool,
}

#[derive(Component)]
struct FlowField {
    values: Vec<u32>,
    flow: Vec<IVec2>,
    width: usize,
    height: usize,
}

