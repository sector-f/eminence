use named_vec::{NamedVec, Named};

struct WmRoot {
    monitors: NamedVec<Monitor>,
}

struct Monitor {
    name: String,
    workspaces: NamedVec<Workspace>,
}

struct Workspace {
    name: String,
    windows: Vec<Window>,
}

struct Window {
    wid: u32,
}

impl Named for Monitor {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Named for Workspace {
    fn name(&self) -> &str {
        &self.name
    }
}
