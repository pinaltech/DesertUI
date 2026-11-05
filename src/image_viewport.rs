//! Image fit, zoom and bounded panning independent of application state.
#[derive(Clone, Copy, Debug)]
pub struct ImageViewport {
    pub zoom: f32,
    pub pan: (f32, f32),
    pub viewport: (f32, f32),
    pub image: (f32, f32),
}
impl Default for ImageViewport {
    fn default() -> Self {
        Self {
            zoom: 1.,
            pan: (0., 0.),
            viewport: (1., 1.),
            image: (1., 1.),
        }
    }
}
impl ImageViewport {
    pub fn reset(&mut self) {
        self.zoom = 1.;
        self.pan = (0., 0.);
    }
    pub fn fit_size(&self) -> (f32, f32) {
        let scale =
            (self.viewport.0 / self.image.0.max(1.)).min(self.viewport.1 / self.image.1.max(1.));
        (self.image.0 * scale, self.image.1 * scale)
    }
    pub fn layout(&mut self, viewport: (f32, f32), image: (f32, f32)) {
        self.viewport = viewport;
        self.image = image;
        self.clamp();
    }
    pub fn set_zoom(&mut self, zoom: f32, anchor: (f32, f32)) {
        if !zoom.is_finite() {
            return;
        }
        let next = zoom.clamp(0.1, 8.);
        let ratio = next / self.zoom;
        self.pan = (
            anchor.0 - (anchor.0 - self.pan.0) * ratio,
            anchor.1 - (anchor.1 - self.pan.1) * ratio,
        );
        self.zoom = next;
        self.clamp();
    }
    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.pan.0 += dx;
        self.pan.1 += dy;
        self.clamp();
    }
    fn clamp(&mut self) {
        let fit = self.fit_size();
        let x = ((fit.0 * self.zoom - self.viewport.0) / 2.).max(0.);
        let y = ((fit.1 * self.zoom - self.viewport.1) / 2.).max(0.);
        self.pan.0 = self.pan.0.clamp(-x, x);
        self.pan.1 = self.pan.1.clamp(-y, y);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fit_and_pan_never_expose_empty_edges() {
        let mut v = ImageViewport::default();
        v.layout((800., 600.), (1600., 800.));
        assert_eq!(v.fit_size(), (800., 400.));
        v.translate(100., 100.);
        assert_eq!(v.pan, (0., 0.));
        v.set_zoom(2., (100., 50.));
        assert_eq!(v.pan, (-100., -50.));
        v.translate(900., 900.);
        assert_eq!(v.pan, (400., 100.));
        v.reset();
        assert_eq!(v.pan, (0., 0.));
    }
}
