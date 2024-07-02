use chip_8_rendering::window::Window;

fn main() -> Result<(), String> {
    let mut window = match Window::new("CHIP-8 emulator", 700, 400) {
        Ok(w) => w,
        Err(e) => return Err(e),
    };

    let renderer = window.borrow_mut_renderer();

    match renderer.init_resources() {
        Ok(_) => {}
        Err(err) => {
            return Err(err);
        }
    }

    let mut y = 0;

    for _ in 0..2 {
        for x in 0..64 {
            if let Err(err) = renderer.toggle_grid_pixel(x, y) {
                return Err(err);
            }
        }
        y = 31;
    }

    let mut x = 0;

    for _ in 0..2 {
        for y in 1..31 {
            if let Err(err) = renderer.toggle_grid_pixel(x, y) {
                return Err(err);
            }
        }
        x = 63;
    }

    match window.run() {
        Ok(_) => {}
        Err(err) => return Err(err),
    }

    Ok(())
}
