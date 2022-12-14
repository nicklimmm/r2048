use macroquad::{color::hsl_to_rgb, prelude::*};
use r2048::grid::{Action, Grid, GRID_SIZE};

const CELL_SIZE: f32 = 100.0;
const CELL_PADDING: f32 = 10.0;
const CELL_FONT_SIZE: f32 = 40.0;
const INPUT_DELAY: f64 = 0.2;
const MAX_CELL_VAL: u32 = 2048;

#[macroquad::main(window_conf)]
async fn main() {
    let mut grid = Grid::new();
    let mut last_key_down_time = 0f64;

    loop {
        clear_background(WHITE);

        // to prevent very fast inputs
        let now = get_time();
        if now - last_key_down_time > INPUT_DELAY {
            if is_key_down(KeyCode::Down) {
                last_key_down_time = now;
                grid.update(Action::Down);
                grid.insert_random_cell();
            } else if is_key_down(KeyCode::Up) {
                last_key_down_time = now;
                grid.update(Action::Up);
                grid.insert_random_cell();
            } else if is_key_down(KeyCode::Left) {
                last_key_down_time = now;
                grid.update(Action::Left);
                grid.insert_random_cell();
            } else if is_key_down(KeyCode::Right) {
                last_key_down_time = now;
                grid.update(Action::Right);
                grid.insert_random_cell();
            }
        }

        draw_grid(&grid);

        next_frame().await
    }
}

fn draw_grid(grid: &Grid) {
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            let rect_x = 0.5 * screen_width()
                + (j as f32 - 2.0) * CELL_SIZE
                + (j as f32 - 1.5) * CELL_PADDING;
            let rect_y = 0.5 * screen_height()
                + (i as f32 - 2.0) * CELL_SIZE
                + (i as f32 - 1.5) * CELL_PADDING;

            let val = grid.cells[i][j];

            let (min_l, max_l) = (0.5, 0.975);
            let l_scale = if val == 0 {
                // handle log2(0)
                0f32
            } else {
                (val as f32).log2() / (MAX_CELL_VAL as f32).log2()
            };
            let l = max_l - l_scale * (max_l - min_l);

            draw_rectangle(
                rect_x,
                rect_y,
                CELL_SIZE,
                CELL_SIZE,
                hsl_to_rgb(0.0, 1.0, l),
            );

            draw_text(
                &val.to_string(),
                rect_x + CELL_FONT_SIZE,
                rect_y + 0.5 * CELL_SIZE + 0.25 * CELL_FONT_SIZE,
                CELL_FONT_SIZE,
                BLACK,
            );
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("R2048"),
        ..Default::default()
    }
}
