use std::sync::Arc;
use std::time::Duration;

use smithay::reexports::wayland_server::DispatchData;
use smithay::wayland::seat::Seat;
use smithay::wayland::shell::xdg::{xdg_shell_init, XdgRequest};
use smithay::wayland::output::{Output, PhysicalProperties};
use smithay::utils::Size;
use smithay::reexports::wayland_server::protocol::wl_output::Subpixel;

use crate::ipc::HyprlandIPC;
use slog;

pub struct Compositor {
    pub display: smithay::reexports::wayland_server::Display,
    pub seat: Seat,
    pub output: Output,
    running: bool,
}

impl Compositor {
    pub fn new() -> Result<Self, String> {
        let mut display = smithay::reexports::wayland_server::Display::new();

        let logger: Option<slog::Logger> = None;

        let physical_properties = PhysicalProperties {
            size: Size::from((1920, 1080)),
            subpixel: Subpixel::Unknown,
            make: "HDM".into(),
            model: "Output".into(),
        };

        let (seat, _seat_global) = Seat::new(&mut display, "HDM Seat".into(), logger.clone());
        let (output, _output_global) =
            Output::new(&mut display, "HDM Output".into(), physical_properties, logger.clone());

        xdg_shell_init(&mut display, |_req: XdgRequest, _dispatch: DispatchData| {}, logger);

        Ok(Self {
            display,
            seat,
            output,
            running: false,
        })
    }

    pub fn run_with_ipc(&mut self, ipc: Option<Arc<HyprlandIPC>>) -> Result<(), String> {
        self.running = true;
        while self.running {
            self.display.dispatch(Duration::from_millis(16), &mut ()).unwrap();
            self.display.flush_clients(&mut ());

            if let Some(ipc_ref) = &ipc {
                if let Ok(active) = ipc_ref.get_status() {
                    println!("Active window: {}", active);
                }
            }

            std::thread::sleep(Duration::from_millis(16));
        }
        Ok(())
    }

    pub fn stop(&mut self) {
        self.running = false;
    }
}
