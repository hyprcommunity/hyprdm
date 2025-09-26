use config::parser::HDMConfig;

pub enum Layout {
    Tiling,
    Floating,
}

pub struct Panel {
    pub name: String, 
    pub layout: Layout,
}

pub struct LayoutManager {
    pub panel: Panel,
    pub global_layout: Layout,
}

impl LayoutManager {
    pub fn new(config: &HDMConfig) -> Self {
        Self {
            panel: Panel {
                name: config.default_session.clone(),
                layout: Layout::Tiling,
            },
            global_layout: Layout::Tiling,
        }
    }

    pub fn apply(&self, width: u32, height: u32, x: u32, y: u32) {
        if matches!(self.panel.layout, Layout::Tiling) {
            println!(
                "Launching QuickShell '{}' => x:{} y:{} w:{} h:{}",
                self.panel.name, x, y, width, height
            );
        } else {
            println!(
                "Launching QuickShell '{}' in floating mode, centered on screen",
                self.panel.name
            );
        }
    }
} 
