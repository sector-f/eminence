struct WmRoot {
    monitors: Vec<Monitor>,
}

struct Monitor {
    name: String,
    workspaces: Vec<Workspace>
}

struct Workspace {
    name: String,
    windows: Vec<Window>,
}

struct Window {
    wid: u32,
}
