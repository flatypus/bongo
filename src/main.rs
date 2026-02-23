#![windows_subsystem = "windows"]

use device_query::{DeviceQuery, DeviceState, Keycode, MouseState};
use softbuffer::Buffer;
use std::num::NonZeroU32;
use std::time::{Duration, Instant};
use tao::dpi::PhysicalPosition;
use tao::dpi::PhysicalSize;
use tao::event::{Event, WindowEvent};
use tao::event_loop::ControlFlow;
use tao::event_loop::EventLoop;
use tao::window::Window;
use tao::window::WindowBuilder;
use tiny_skia::FillRule;
use tiny_skia::Transform;
use tiny_skia::{Paint, Path, PathBuilder, Pixmap, Stroke};

use colors::*;

#[cfg(target_os = "macos")]
use tao::platform::macos::WindowBuilderExtMacOS;

#[cfg(target_os = "windows")]
use tao::platform::windows::WindowBuilderExtWindows;

mod colors {
    pub const WHITE: [u8; 4] = [255, 255, 255, 255];
    pub const BLACK: [u8; 4] = [0, 0, 0, 255];
    pub const BLUE: [u8; 4] = [56, 143, 255, 150];
    pub const PINK: [u8; 4] = [255, 138, 202, 255];
    pub const PINK_STROKE: [u8; 4] = [223, 65, 143, 255];
    pub const MOUSEPAD_FILL: [u8; 4] = [169, 168, 170, 255];
    pub const MOUSEPAD_STROKE: [u8; 4] = [108, 108, 110, 255];
}

struct ClickStates {
    left_click: bool,
    right_click: bool,
    other_click: bool,
    middle_click: bool,
}

impl ClickStates {
    fn new() -> Self {
        Self {
            left_click: false,
            right_click: false,
            other_click: false,
            middle_click: false,
        }
    }
}

fn paint([r, g, b, a]: [u8; 4]) -> Paint<'static> {
    let mut p = Paint::default();
    p.set_color_rgba8(r, g, b, a);
    p.anti_alias = true;
    p
}

fn fill_path(pixmap: &mut Pixmap, path: &Path, transform: Transform, color: [u8; 4]) {
    pixmap.fill_path(path, &paint(color), FillRule::Winding, transform, None);
}

fn stroke_path(
    pixmap: &mut Pixmap,
    path: &Path,
    transform: Transform,
    stroke: &Stroke,
    color: [u8; 4],
) {
    pixmap.stroke_path(path, &paint(color), stroke, transform, None);
}

fn fill_and_stroke(
    pixmap: &mut Pixmap,
    path: &Path,
    transform: Transform,
    stroke: &Stroke,
    fill_color: [u8; 4],
    stroke_color: [u8; 4],
) {
    fill_path(pixmap, path, transform, fill_color);
    stroke_path(pixmap, path, transform, stroke, stroke_color);
}

fn lerp(start: f32, end: f32, pos: f32) -> f32 {
    return start * (1.0 - pos) + end * pos;
}

fn lerp_skew(
    topleft: (f32, f32),
    topright: (f32, f32),
    bottomleft: (f32, f32),
    bottomright: (f32, f32),
    x: f32,
    y: f32,
) -> (f32, f32) {
    let lerptop = lerp(topleft.0, topright.0, x);
    let lerpbottom = lerp(bottomleft.0, bottomright.0, x);
    let lerpedx = lerp(lerptop, lerpbottom, y);

    let lerpleft = lerp(topleft.1, bottomleft.1, y);
    let lerpright = lerp(topright.1, bottomright.1, y);
    let lerpedy = lerp(lerpleft, lerpright, x);

    return (lerpedx, lerpedy);
}

fn draw_bongo(
    buffer: &mut Buffer<'_, &Window, &Window>,
    width: u32, // window sizing
    height: u32,
    mouse_x: f32, // float 0-1
    mouse_y: f32,
    opacity: f32,
    click_states: &ClickStates,
) {
    const START_X: f32 = 0.2;
    const START_Y: f32 = -0.407;

    let armx = -3.634;
    let army = 2.338;

    let arm_abs_x = START_X + armx;
    let arm_abs_y = START_Y + army;

    // the farther left our mouse is, farther right bongocat's hand is
    // too far right, the bezier handle looks wonky, we do some black magic idek anymore
    let armx1 = -2.424 + 2.0 * mouse_x;
    let army1 = 2.681 - 1.6 * mouse_x;
    let armx2 = -1.09 - 1.0 * mouse_x;
    let army2 = 1.732 + 2.0 * mouse_x;
    let armx3 = -5.165 - 1.5 * mouse_x;
    let army3 = 0.844 - 1.0 * mouse_x;

    let left_handle_dx = armx1 - armx;
    let left_handle_dy = army1 - army;
    let right_handle_dx = armx2 - armx;
    let right_handle_dy = army2 - army;

    let (x, y) = lerp_skew(
        (-1.665, 5.405),
        (-8.363, 2.681),
        (2.281, 1.676),
        (-5.605, -0.051),
        mouse_x,
        mouse_y,
    );

    // now we want to slightly offset the whole body/eyes/mouth etc depending on the mouse
    let whole_dx = x - arm_abs_x;
    let whole_dy = y - arm_abs_y;

    const BODY_TF_SCALE: f32 = 0.04;

    let body_dx = whole_dx * BODY_TF_SCALE;
    let body_dy = whole_dy * BODY_TF_SCALE;

    let body_base = |pb: &mut PathBuilder| {
        pb.move_to(START_X, START_Y);
        pb.cubic_to(
            x + left_handle_dx,
            y + left_handle_dy,
            x + right_handle_dx,
            y + right_handle_dy,
            x,
            y,
        );
        pb.cubic_to(armx3, army3, -0.985, -5.213, 2.261, -5.721);
        pb.cubic_to(2.732, -6.014, 3.229, -6.891, 3.514, -6.899);
        pb.cubic_to(3.763, -6.887, 3.9997, -6.035, 4.257, -5.603);
        pb.cubic_to(5.875, -5.403, 7.987, -4.278, 9.31, -3.289);
        pb.cubic_to(9.449, -3.2, 10.763, -4.026, 11.005, -3.912);
        pb.cubic_to(11.182, -3.802, 11.078, -2.014, 10.467, -0.976);
        pb.cubic_to(10.996, -0.16, 11.772, 0.874, 11.793, 2.282);
    };

    let body_fill = {
        let mut pb = PathBuilder::new();
        body_base(&mut pb);
        // only need final line to complete the body, don't need for stroke
        // we subtract body_dx/body_dy so that this final line doesn't move with the body
        // otherwise funny clipping issues
        pb.line_to(-1.974 - body_dx, -0.45 - body_dy);
        pb.close();
        pb.finish().unwrap()
    };

    let body_stroke = {
        let mut pb = PathBuilder::new();
        body_base(&mut pb);
        pb.finish().unwrap()
    };

    let left_eye = {
        let mut pb = PathBuilder::new();
        pb.move_to(1.247, -2.605);
        pb.cubic_to(1.25, -2.63, 1.273, -2.981, 1.551, -2.979);
        pb.cubic_to(1.791, -2.976, 1.836, -2.693, 1.831, -2.589);
        pb.cubic_to(1.836, -2.359, 1.662, -2.239, 1.54, -2.239);
        pb.cubic_to(1.436, -2.237, 1.246, -2.336, 1.247, -2.604);
        pb.finish().unwrap()
    };

    let right_eye = {
        let mut pb = PathBuilder::new();
        pb.move_to(5.811, -1.094);
        pb.cubic_to(5.813, -1.276, 5.934, -1.531, 6.133, -1.529);
        pb.cubic_to(6.356, -1.53, 6.455, -1.276, 6.456, -1.081);
        pb.cubic_to(6.465, -0.877, 6.314, -0.687, 6.142, -0.693);
        pb.cubic_to(5.978, -0.688, 5.807, -0.849, 5.813, -1.094);
        pb.close();
        pb.finish().unwrap()
    };

    let mouth = {
        let mut pb = PathBuilder::new();
        pb.move_to(2.529, -2.441);
        pb.cubic_to(2.521, -2.191, 2.897, -1.8, 3.331, -2.133);
        pb.cubic_to(3.544, -1.458, 4.132, -1.757, 4.281, -2.005);
        pb.cubic_to(4.313, -2.046, 4.308, -2.08, 4.291, -2.095);
        pb.cubic_to(4.196, -2.163, 4.085, -1.867, 3.709, -1.884);
        pb.cubic_to(3.438, -1.889, 3.431, -2.18, 3.418, -2.217);
        pb.cubic_to(3.406, -2.273, 3.363, -2.284, 3.32, -2.258);
        pb.cubic_to(3.287, -2.245, 3.191, -2.091, 2.919, -2.165);
        pb.cubic_to(2.715, -2.26, 2.681, -2.417, 2.659, -2.483);
        pb.cubic_to(2.63, -2.602, 2.527, -2.584, 2.529, -2.44);
        pb.finish().unwrap()
    };

    let table = {
        let mut pb = PathBuilder::new();
        pb.move_to(-9.766, -1.696);
        pb.line_to(14.889, 2.846);
        pb.line_to(14.889, 6.821);
        pb.line_to(-9.766, 6.854);
        pb.close();
        pb.finish().unwrap()
    };

    let mousepad = {
        let mut pb = PathBuilder::new();
        pb.move_to(-1.141, 5.256);
        pb.cubic_to(-1.356, 5.470, -1.763, 5.471, -2.044, 5.357);
        pb.line_to(-8.169, 2.876);
        pb.cubic_to(-8.286, 2.829, -8.330, 2.595, -8.24, 2.506);
        pb.line_to(-5.722, 0.024);
        pb.cubic_to(-5.571, -0.125, -5.297, -0.103, -5.09, -0.06);
        pb.line_to(2.032, 1.425);
        pb.cubic_to(2.195, 1.459, 2.377, 1.754, 2.259, 1.872);
        pb.close();
        pb.finish().unwrap()
    };

    let mouse = {
        let mut pb = PathBuilder::new();
        pb.move_to(-4.403, 2.977);
        pb.cubic_to(-4.504, 2.931, -4.856, 2.379, -4.414, 1.363);
        pb.cubic_to(-4.079, 0.878, -3.362, 0.161, -2.463, 0.477);
        pb.cubic_to(-1.976, 0.671, -1.411, 1.452, -1.976, 2.319);
        pb.cubic_to(-2.517, 3.077, -3.355, 3.721, -4.4, 2.975);
        pb.line_to(-3.709, 2.056);
        pb.close();
        pb.finish().unwrap()
    };

    let mouse_wheel = {
        let mut pb = PathBuilder::new();
        pb.move_to(-4.107, 2.429);
        pb.cubic_to(-4.07, 2.375, -3.996, 2.346, -3.95, 2.376);
        pb.cubic_to(-3.91, 2.403, -3.907, 2.481, -3.95, 2.542);
        pb.cubic_to(-4.017, 2.633, -4.077, 2.648, -4.129, 2.61);
        pb.cubic_to(-4.174, 2.578, -4.167, 2.498, -4.107, 2.429);
        pb.close();
        pb.finish().unwrap()
    };

    let mouse_button_left = {
        let mut pb = PathBuilder::new();
        pb.move_to(-4.399, 2.976);
        pb.line_to(-3.537, 1.828);
        pb.line_to(-2.182, 2.577);
        pb.cubic_to(-2.577, 3.037, -3.354, 3.721, -4.401, 2.976);
        pb.finish().unwrap()
    };

    let mouse_button_right = {
        let mut pb = PathBuilder::new();
        pb.move_to(-4.402, 2.975);
        pb.line_to(-3.539, 1.82);
        pb.line_to(-4.361, 1.304);
        pb.cubic_to(-4.574, 1.586, -4.821, 2.562, -4.406, 2.978);
        pb.close();
        pb.finish().unwrap()
    };

    let left_hand = {
        let mut pb = PathBuilder::new();
        pb.move_to(7.063, 1.35);
        pb.cubic_to(6.898, 0.293, 6.467, -2.05, 7.581, -2.295);
        pb.cubic_to(8.715, -2.57, 9.464, -1.238, 9.717, -0.015);
        pb.finish().unwrap()
    };

    let paw_1 = {
        let mut pb = PathBuilder::new();
        pb.move_to(7.848, -1.372);
        pb.cubic_to(7.983, -1.405, 8.068, -1.284, 8.097, -1.163);
        pb.cubic_to(8.116, -1.09, 8.138, -0.897, 7.981, -0.845);
        pb.cubic_to(7.837, -0.816, 7.755, -0.974, 7.725, -1.073);
        pb.cubic_to(7.696, -1.168, 7.713, -1.332, 7.848, -1.372);
        pb.close();
        pb.finish().unwrap()
    };

    let paw_2 = {
        let mut pb = PathBuilder::new();
        pb.move_to(7.459, -0.694);
        pb.cubic_to(7.652, -0.725, 7.697, -0.489, 7.706, -0.434);
        pb.cubic_to(7.722, -0.338, 7.706, -0.112, 7.506, -0.096);
        pb.cubic_to(7.363, -0.081, 7.283, -0.23, 7.267, -0.364);
        pb.cubic_to(7.245, -0.498, 7.303, -0.677, 7.458, -0.693);
        pb.close();
        pb.finish().unwrap()
    };

    let paw_3 = {
        let mut pb = PathBuilder::new();
        pb.move_to(8.506, -0.792);
        pb.cubic_to(8.658, -0.819, 8.78, -0.692, 8.802, -0.518);
        pb.cubic_to(8.824, -0.344, 8.734, -0.204, 8.613, -0.186);
        pb.cubic_to(8.492, -0.168, 8.4, -0.262, 8.356, -0.438);
        pb.cubic_to(8.312, -0.614, 8.388, -0.767, 8.503, -0.792);
        pb.close();
        pb.finish().unwrap()
    };

    let paw_4 = {
        let mut pb = PathBuilder::new();
        pb.move_to(8.093, -0.164);
        pb.cubic_to(8.319, -0.204, 8.52, 0.06, 8.571, 0.271);
        pb.cubic_to(8.622, 0.482, 8.634, 0.944, 8.327, 0.975);
        pb.cubic_to(8.02, 1.006, 7.877, 0.642, 7.834, 0.451);
        pb.cubic_to(7.791, 0.26, 7.836, -0.1, 8.085, -0.161);
        pb.close();
        pb.finish().unwrap()
    };

    let table_line_left = {
        let mut pb = PathBuilder::new();
        pb.move_to(-9.77, -1.696);
        pb.line_to(2.992, 0.647);
        pb.finish().unwrap()
    };

    let table_line_right = {
        let mut pb = PathBuilder::new();
        pb.move_to(2.995, 0.643);
        pb.line_to(14.892, 2.852);
        pb.finish().unwrap()
    };

    const SVG_WIDTH: f32 = 24.86;
    const SVG_HEIGHT: f32 = 13.95;
    const SVG_ORIGIN_X: f32 = 9.87;
    const SVG_ORIGIN_Y: f32 = 6.999;

    let scale_x = width as f32 / SVG_WIDTH;
    let scale_y = height as f32 / SVG_HEIGHT;
    let scale = scale_x.min(scale_y);
    let offset_x = (width as f32 - SVG_WIDTH * scale) / 2.0;
    let offset_y = (height as f32 - SVG_HEIGHT * scale) / 2.0;
    let transform = Transform::from_translate(SVG_ORIGIN_X, SVG_ORIGIN_Y)
        .post_scale(scale, scale)
        .post_translate(offset_x, offset_y);

    const EYE_TF_SCALE: f32 = 0.12;
    const MOUTH_TF_SCALE: f32 = 0.105;

    let eye_tf = transform.pre_translate(whole_dx * EYE_TF_SCALE, whole_dy * EYE_TF_SCALE);
    let body_tf = transform.pre_translate(whole_dx * BODY_TF_SCALE, whole_dy * BODY_TF_SCALE);
    let mouth_tf = transform.pre_translate(whole_dx * MOUTH_TF_SCALE, whole_dy * MOUTH_TF_SCALE);

    let mouse_dx = whole_dx;
    let mouse_dy = whole_dy;
    let mouse_tf = transform.pre_translate(mouse_dx, mouse_dy);

    let mut pixmap = Pixmap::new(width, height).unwrap();

    let stroke = Stroke {
        width: 0.01 * scale,
        ..Stroke::default()
    };

    fill_and_stroke(&mut pixmap, &table, transform, &stroke, WHITE, BLACK);
    fill_and_stroke(
        &mut pixmap,
        &mousepad,
        transform,
        &stroke,
        MOUSEPAD_FILL,
        MOUSEPAD_STROKE,
    );
    fill_and_stroke(&mut pixmap, &mouse, mouse_tf, &stroke, WHITE, BLACK);
    let wheel_fill = if click_states.middle_click {
        BLUE
    } else {
        MOUSEPAD_FILL
    };
    fill_and_stroke(
        &mut pixmap,
        &mouse_wheel,
        mouse_tf,
        &stroke,
        wheel_fill,
        BLACK,
    );
    stroke_path(&mut pixmap, &table_line_left, transform, &stroke, BLACK);
    fill_path(&mut pixmap, &body_fill, body_tf, WHITE);
    stroke_path(&mut pixmap, &body_stroke, body_tf, &stroke, BLACK);
    stroke_path(&mut pixmap, &table_line_right, transform, &stroke, BLACK);

    stroke_path(&mut pixmap, &left_hand, body_tf, &stroke, BLACK);
    fill_and_stroke(&mut pixmap, &paw_1, body_tf, &stroke, PINK, PINK_STROKE);
    fill_and_stroke(&mut pixmap, &paw_2, body_tf, &stroke, PINK, PINK_STROKE);
    fill_and_stroke(&mut pixmap, &paw_3, body_tf, &stroke, PINK, PINK_STROKE);
    fill_and_stroke(&mut pixmap, &paw_4, body_tf, &stroke, PINK, PINK_STROKE);

    fill_path(&mut pixmap, &left_eye, eye_tf, BLACK);
    fill_path(&mut pixmap, &right_eye, eye_tf, BLACK);
    fill_path(&mut pixmap, &mouth, mouth_tf, BLACK);

    if click_states.left_click {
        fill_path(&mut pixmap, &mouse_button_left, mouse_tf, BLUE);
    }
    if click_states.right_click {
        fill_path(&mut pixmap, &mouse_button_right, mouse_tf, BLUE);
    }

    for (index, chunk) in pixmap.data().chunks_exact(4).enumerate() {
        let [r, g, b, a] = [
            ((chunk[0] as f32) * opacity) as u32,
            ((chunk[1] as f32) * opacity) as u32,
            ((chunk[2] as f32) * opacity) as u32,
            ((chunk[3] as f32) * opacity) as u32,
        ];
        (*buffer)[index] = b | (g << 8) | (r << 16) | (a << 24);
    }
}

fn main() {
    const OFFSET: i32 = 50;
    const WIN_HEIGHT: i32 = 240;
    const WIN_WIDTH: i32 = 360;
    const MIN_OPACITY: f32 = 0.4;
    const OPACITY_ANIM_SPEED: f32 = 0.08;

    let event_loop = EventLoop::new();
    let monitor = event_loop.primary_monitor().unwrap();
    let scale_factor = monitor.scale_factor();
    let PhysicalSize {
        width: mon_width,
        height: mon_height,
    } = monitor.size();
    let pos_x = (mon_width as i32) - (WIN_WIDTH + OFFSET);
    let pos_y = (mon_height as i32) - (WIN_HEIGHT + OFFSET);
    let window = WindowBuilder::new()
        .with_transparent(true)
        .with_position(PhysicalPosition::new(pos_x, pos_y))
        .with_inner_size(PhysicalSize::new(WIN_WIDTH, WIN_HEIGHT))
        .with_closable(false)
        .with_decorations(false)
        .with_always_on_top(true);

    #[cfg(target_os = "macos")]
    let window = window
        .with_has_shadow(false)
        .with_visible_on_all_workspaces(true)
        .with_titlebar_hidden(true);

    #[cfg(target_os = "windows")]
    let window = window
        .with_undecorated_shadow(false)
        .with_skip_taskbar(true);

    let window = window.build(&event_loop).unwrap();

    let window: Box<Window> = Box::new(window);
    let window: &'static Window = Box::leak(window);

    let context = { softbuffer::Context::new(window) }.unwrap();
    let mut surface = { softbuffer::Surface::new(&context, window) }.unwrap();
    let mut mouse_x: i32 = 0;
    let mut mouse_y: i32 = 0;

    let mut click_states = ClickStates::new();

    let mut hovering_over = false;
    let mut hovering_opacity: f32 = 1.0;
    let mut focused = false;

    let device_state = DeviceState::new();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(16));

        match event {
            Event::NewEvents(_) => {
                let keys = device_state.get_keys();
                if focused && keys.contains(&Keycode::Q) {
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                let mouse = device_state.get_mouse();
                let new_x = (mouse.coords.0 as f64 * scale_factor) as i32;
                let new_y = (mouse.coords.1 as f64 * scale_factor) as i32;
                if new_x != mouse_x || new_y != mouse_y {
                    mouse_x = new_x;
                    mouse_y = new_y;
                    window.request_redraw();
                }

                let left_click = mouse.button_pressed.get(1).copied().unwrap_or(false);
                let right_click = mouse.button_pressed.get(2).copied().unwrap_or(false);
                let middle_click = mouse.button_pressed.get(3).copied().unwrap_or(false);

                if click_states.left_click != left_click {
                    click_states.left_click = left_click;
                    window.request_redraw();
                }

                if click_states.right_click != right_click {
                    click_states.right_click = right_click;
                    window.request_redraw();
                }

                if click_states.middle_click != middle_click {
                    click_states.middle_click = middle_click;
                    window.request_redraw();
                }

                let other_click = !keys.is_empty();
                if click_states.other_click != other_click {
                    click_states.other_click = other_click;
                    window.request_redraw();
                }

                let new_hovering_opacity = if hovering_over {
                    (hovering_opacity - OPACITY_ANIM_SPEED).max(MIN_OPACITY)
                } else {
                    (hovering_opacity + OPACITY_ANIM_SPEED).min(1.0)
                };

                if new_hovering_opacity != hovering_opacity {
                    window.request_redraw();
                    hovering_opacity = new_hovering_opacity;
                }

                let new_hovering = mouse_x >= pos_x
                    && mouse_x < pos_x + WIN_WIDTH
                    && mouse_y >= pos_y
                    && mouse_y < pos_y + WIN_HEIGHT;

                if hovering_over != new_hovering {
                    window.request_redraw();
                    hovering_over = new_hovering;
                }
            }

            Event::WindowEvent {
                event: WindowEvent::Focused(f),
                ..
            } => {
                focused = f;
            }

            Event::RedrawRequested(_) => {
                surface
                    .resize(
                        NonZeroU32::new(WIN_WIDTH as u32).unwrap(),
                        NonZeroU32::new(WIN_HEIGHT as u32).unwrap(),
                    )
                    .unwrap();
                let mut buffer = surface.buffer_mut().unwrap();
                let scaled_mouse_x = (mouse_x as f32) / (mon_width as f32);
                let scaled_mouse_y = (mouse_y as f32) / (mon_height as f32);

                draw_bongo(
                    &mut buffer,
                    WIN_WIDTH as u32,
                    WIN_HEIGHT as u32,
                    scaled_mouse_x,
                    scaled_mouse_y,
                    hovering_opacity,
                    &click_states,
                );

                buffer.present().unwrap();
            }
            _ => {}
        }
    });
}
