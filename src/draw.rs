use piston_window::graphics::Context;
use piston_window::graphics::rectangle;
use piston_window::graphics::types::Color;
use piston_window::wgpu_graphics::WgpuGraphics;


const BLOCK_SIZE: f64 = 25.0;

pub fn to_coord(coord: i32) -> f64 {
    (coord as f64) * BLOCK_SIZE
}

pub fn draw_block(color: Color, x: i32, y: i32, context: &Context, g: &mut WgpuGraphics) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(color, [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE], context.transform, g);
}

pub fn draw_rectangle(color: Color, x: i32, y: i32, w:i32, h:i32, con:&Context, g: &mut WgpuGraphics) {
    let  x = to_coord(x);
    let y = to_coord(y) ;

    rectangle(color, [x, y, (w as f64) * BLOCK_SIZE, (h as f64) * BLOCK_SIZE], con.transform, g);
}
