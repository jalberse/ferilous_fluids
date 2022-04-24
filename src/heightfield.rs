const DAMPENING_FACTOR: f64 = 0.998;

// Represents a heightfield for fluid surface simulation.
pub struct Heightfield {
    pub width: usize,
    pub height: usize,
    // Flattened 2-dimensional vector of width and height; the height of each cell.
    pub z: Vec<f64>,
    // Flattened 2-dimensional vector of width and height; the velocity of each cell's height.
    dz: Vec<f64>,
}

pub fn new(width: usize, height: usize) -> Heightfield {
    Heightfield {
        width,
        height,
        z: vec![0.1; width * height],
        dz:vec![0.0; width * height],
    }
}

impl Heightfield {
    pub fn set_height(&mut self, x: usize, y: usize, height: f64)
    {
        let z_idx = self.get_z_idx(x, y);
        if z_idx >= self.z.len()
        {
            return;
        }
        self.z[z_idx] = height;
    }

    fn get_z_idx(&self, x: usize, y: usize) -> usize
    {
        return y * self.width + x;
    }

    fn get_acceleration(&self, x: usize, y: usize) -> f64
    {
        let north_height: f64 = if y == 0 { self.z[self.get_z_idx(x, y)] } else { self.z[ self.get_z_idx( x, y - 1 ) ] };
        let south_height: f64 = if y == self.height - 1 { self.z[self.get_z_idx(x, y)] } else { self.z[self.get_z_idx( x, y + 1 ) ] };
        let west_height: f64 = if x == 0 { self.z[self.get_z_idx( 0, y )] } else { self.z[ self.get_z_idx( x - 1, y ) ] };
        let east_height: f64 = if x == self.width - 1 { self.z[self.get_z_idx(x, y)] } else { self.z[self.get_z_idx(x + 1, y )] };

        return (north_height + south_height + west_height + east_height) as f64 / 4.0 - self.z[self.get_z_idx(x, y)] as f64;
    }

    pub fn step(&mut self, dt: std::time::Duration) {
        // Update the velocity of each cell
        for y in 0..self.height
        {
            for x in 0..self.width
            {
                let z_idx = self.get_z_idx(x, y);

                self.dz[z_idx] += self.get_acceleration(x, y);
                self.dz[z_idx] *= DAMPENING_FACTOR;
            }
        }

        // Update the heights of each cell based on the velocities.
        // Note this must be a separate loop as the acceleration calculation
        // depends on the heights of neighboring cells.
        for y in 0..self.height
        {
            for x in 0..self.width
            {
                let z_idx = self.get_z_idx(x, y);

                self.z[z_idx] += self.dz[self.get_z_idx(x, y)] * dt.as_secs_f64();
                self.z[z_idx] = self.z[z_idx].clamp(0.0, 1.0)
            }
        }
    }
}