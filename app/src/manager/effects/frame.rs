pub struct Frame {
    pub rgb: [u8; 12],
}

impl Frame {
    pub fn new() -> Self {
        Self { rgb: [0; 12] }
    }

    pub fn clear(&mut self) {
        self.rgb = [0; 12];
    }

    pub fn set_zone(&mut self, zone: usize, color: [u8; 3], brightness: f32) {
        if zone >= 4 {
            return;
        }

        let i = zone * 3;

        self.rgb[i] = (color[0] as f32 * brightness)
            .round()
            .clamp(0.0, 255.0) as u8;

        self.rgb[i + 1] = (color[1] as f32 * brightness)
            .round()
            .clamp(0.0, 255.0) as u8;

        self.rgb[i + 2] = (color[2] as f32 * brightness)
            .round()
            .clamp(0.0, 255.0) as u8;
    }
}