use ggez::*;

mod heightfield;

const SCREEN_SIZE: (f32, f32) = (
    400.0,
    400.0
);

fn main() {
    let c = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("ferilous_fluids", "John Alberse")
        .default_conf(c)
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()
        .unwrap();

    let size = graphics::window(&ctx).inner_size();

    let width = usize::try_from(size.width).unwrap();
    let height = usize::try_from(size.height).unwrap();

    let state = State {
        dt: std::time::Duration::new(0, 0),
        heightfield: heightfield::new(width, height),
        heightfield_pixels: vec![0; width * height * 4]
    };

    event::run(ctx, event_loop, state);
}

struct State {
    dt: std::time::Duration,
    heightfield: heightfield::Heightfield,
    heightfield_pixels: Vec<u8>,
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context ) -> GameResult {
        self.dt = timer::delta(ctx);

        if input::mouse::button_pressed(ctx, event::MouseButton::Left)
        {
            let mouse_pos = input::mouse::position(ctx);
            let x_pos = usize::try_from(mouse_pos.x as u64).unwrap();
            let y_pos = usize::try_from(mouse_pos.y as u64).unwrap();
            self.heightfield.set_height(x_pos, y_pos, 1.0);
        }

        self.heightfield.step(self.dt);

        let mut pixel_idx = 0;
        for height in &self.heightfield.z
        {
            let red_idx = pixel_idx;
            let green_idx = pixel_idx + 1;
            let blue_idx = pixel_idx + 2;
            let alpha_idx = pixel_idx + 3;

            self.heightfield_pixels[red_idx] = (height * 255.0) as u8;
            self.heightfield_pixels[green_idx] = (height * 255.0) as u8;
            self.heightfield_pixels[blue_idx] = (height * 255.0) as u8;
            self.heightfield_pixels[alpha_idx] = 255;

            pixel_idx += 4;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);

        let image = graphics::Image::from_rgba8
            (
            ctx,
            self.heightfield.width as u16,
            self.heightfield.height as u16,
            &self.heightfield_pixels
            ).unwrap();
        graphics::draw(ctx, &image, graphics::DrawParam::new())?;

        graphics::present(ctx)?;
        timer::yield_now();

        Ok(())
    }
}