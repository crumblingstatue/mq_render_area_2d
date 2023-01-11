//! Provides a 2d render area with a camera.
//!
//! The render area has a "virtual" size that is independent of the actual window size.
//! Then it can be scaled to fit the actual window size.

#![warn(missing_docs)]

use macroquad::prelude::*;

/// Acts as a regular screen with the specified dimensions when drawing to it,
/// but has functionality to be easily rescaled and centered.
///
/// Example usage:
///
/// ```
/// use macroquad::prelude::*;
/// use mq_render_area_2d::RenderArea2D;
///
/// #[macroquad::main("renderarea2dtest")]
/// async fn main() {
///     let mut ra = RenderArea2D::new(400, 300);
///     ra.center_camera(0., 0.);
///     loop {
///
///         if is_key_down(KeyCode::Left) {
///             ra.move_camera(-2., 0.);
///         }
///         if is_key_down(KeyCode::Up) {
///             ra.move_camera(0., -2.);
///         }
///         if is_key_down(KeyCode::Right) {
///             ra.move_camera(2., 0.);
///         }
///         if is_key_down(KeyCode::Down) {
///             ra.move_camera(0., 2.);
///         }
///
///         ra.set();
///         clear_background(BLACK);
///         draw_circle(0.0, 0.0, 100.0, YELLOW);
///         set_default_camera();
///         ra.draw();
///         next_frame().await;
///     }
/// }
/// ```
pub struct RenderArea2D {
    render_target: RenderTarget,
    width: u16,
    height: u16,
    scale: u8,
    camera: Camera2D,
}

fn target(width: u16, height: u16) -> Vec2 {
    vec2(f32::from(width) / 2.0, f32::from(height) / 2.0)
}

impl RenderArea2D {
    /// Create a new render area with the specified virtual resolution.
    pub fn new(width: u16, height: u16) -> Self {
        let rt = render_target(width.into(), height.into());
        let cam = Camera2D {
            render_target: Some(rt),
            zoom: vec2(2. / f32::from(width), 2. / f32::from(height)),
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
    /// Sets this render area for drawing.
    ///
    /// Call this before drawing into the render area.
    pub fn set(&self) {
        set_camera(&self.camera);
    }
    /// Set the scale to an integer amount. 2 is 2x zoom for example.
    pub fn set_scale(&mut self, amount: u8) {
        self.scale = amount;
    }
    /// Set the scale automatically to fit the window size.
    pub fn set_scale_auto(&mut self) {
        self.scale = self.auto_scale();
    }
    /// Get the biggest scale that still fits on the screen
    pub fn auto_scale(&self) -> u8 {
        let hor_ratio = screen_width() / f32::from(self.width);
        let ver_ratio = screen_height() / f32::from(self.height);
        (if hor_ratio < ver_ratio {
            hor_ratio
        } else {
            ver_ratio
        }) as u8
    }
    /// Get the integer scale
    pub fn scale(&self) -> u8 {
        self.scale
    }
    /// Draw this render area to the window.
    ///
    /// You need to first set the default camera with macroquad's `set_default_camera()`.
    pub fn draw(&self) {
        let params = DrawTextureParams {
            dest_size: Some(vec2(
                f32::from(self.width) * f32::from(self.scale),
                f32::from(self.height) * f32::from(self.scale),
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
            (mx - x_off) / f32::from(self.scale),
            (my - y_off) / f32::from(self.scale),
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
            (screen_width() - f32::from(self.width) * f32::from(self.scale)) / 2.0,
            (screen_height() - f32::from(self.height) * f32::from(self.scale)) / 2.0,
        )
    }
    /// Move the camera (x, y) by the specified amounts
    pub fn move_camera(&mut self, x: f32, y: f32) {
        self.camera.target += vec2(x, y);
    }
    /// Center the camera on (x, y)
    pub fn center_camera(&mut self, x: f32, y: f32) {
        self.camera.target = vec2(x + 16.0, y + 16.0);
    }
}
