use macroquad::prelude::*;

/// Acts as a regular screen with the specified dimensions when drawing to it,
/// but has functionality to be easily rescaled and centered.
pub struct RenderArea2D {
    render_target: RenderTarget,
    width: u32,
    height: u32,
    scale: u8,
    camera: Camera2D,
}

fn target(width: u32, height: u32) -> Vec2 {
    vec2(width as f32 / 2.0, height as f32 / 2.0)
}

impl RenderArea2D {
    pub fn new(width: u32, height: u32) -> Self {
        let rt = render_target(width, height);
        let cam = Camera2D {
            render_target: Some(rt),
            zoom: vec2(2. / width as f32, 2. / height as f32),
            target: target(width, height),
            ..Default::default()
        };
        let mut s = Self {
            width,
            height,
            render_target: rt,
            scale: 0,
            camera: cam,
        };
        s.render_target.texture.set_filter(FilterMode::Nearest);
        s.set_scale_auto();
        s
    }
    pub fn set(&self) {
        set_camera(&self.camera);
    }
    pub fn set_scale(&mut self, amount: u8) {
        self.scale = amount;
    }
    pub fn set_scale_auto(&mut self) {
        self.scale = self.auto_scale();
    }
    /// Get the biggest scale that still fits on the screen
    pub fn auto_scale(&self) -> u8 {
        let hor_ratio = screen_width() / self.width as f32;
        let ver_ratio = screen_height() / self.height as f32;
        (if hor_ratio < ver_ratio {
            hor_ratio
        } else {
            ver_ratio
        }) as u8
    }
    pub fn scale(&self) -> u8 {
        self.scale
    }
    pub fn draw(&self) {
        let params = DrawTextureParams {
            dest_size: Some(vec2(
                self.width as f32 * self.scale as f32,
                self.height as f32 * self.scale as f32,
            )),
            ..Default::default()
        };
        let (x_off, y_off) = self.screen_offset();
        draw_texture_ex(self.render_target.texture, x_off, y_off, WHITE, params);
    }
    /// Gives mouse position translated to the render area coordinates
    pub fn mouse_position(&self) -> (f32, f32) {
        let (mx, my) = mouse_position();
        let (x_off, y_off) = self.screen_offset();
        (
            (mx - x_off) / self.scale as f32,
            (my - y_off) / self.scale as f32,
        )
    }
    /// Gives mouse position translated to the render area coordinates, including camera offset
    pub fn mouse_position_cam(&self) -> (f32, f32) {
        let (mx, my) = self.mouse_position();
        let offs = self.camera.target - target(self.width, self.height);
        (mx + offs.x, my + offs.y)
    }
    fn screen_offset(&self) -> (f32, f32) {
        (
            (screen_width() - self.width as f32 * self.scale as f32) / 2.0,
            (screen_height() - self.height as f32 * self.scale as f32) / 2.0,
        )
    }
    pub fn move_camera(&mut self, x: f32, y: f32) {
        self.camera.target += vec2(x, y);
    }
    pub fn center_camera(&mut self, x: f32, y: f32) {
        self.camera.target = vec2(x + 16.0, y + 16.0);
    }
}
