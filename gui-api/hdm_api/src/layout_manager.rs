pub enum Layout {
    Tiling,
    Floating,
}

pub struct Panel {
    pub name: String,
    pub layout: Layout,
}

pub struct LayoutManager {
    pub panels: Vec<Panel>,
    pub global_layout: Layout,
}

impl LayoutManager {
    pub fn apply(&self) {
        let screen_width = 1920;
        let screen_height = 1080;
        let tiling_panels: Vec<&Panel> = self.panels.iter()
            .filter(|p| matches!(p.layout, Layout::Tiling))
            .collect();

        let num = tiling_panels.len();
        for (i, panel) in tiling_panels.iter().enumerate() {
            let width = screen_width / num as u32;
            let x = i as u32 * width;
            let y = 0;
            let height = screen_height;

            // Burada gerçek smithay container position set et
            println!("Tiling Panel {} => x:{} y:{} w:{} h:{}", panel.name, x, y, width, height);
        }

        for panel in &self.panels {
            if matches!(panel.layout, Layout::Floating) {
                // Floating paneli center veya config konumuna getir
                println!("Floating Panel {} => center on screen", panel.name);
            }
        }
    }
}
