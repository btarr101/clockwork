use raw_window_handle::{ HasRawWindowHandle, HasRawDisplayHandle };

pub struct Renderer {}

impl Renderer {
    pub(crate) fn new<Window: HasRawWindowHandle + HasRawDisplayHandle>(_window: &Window) -> Self {
        Self {}
    }
}
