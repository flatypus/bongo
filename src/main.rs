#![windows_subsystem = "windows"]

use colors::*;
use device_query::{DeviceQuery, DeviceState, Keycode};
use rand::Rng;
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
mod paths;

#[cfg(target_os = "macos")]
use tao::platform::macos::WindowBuilderExtMacOS;

#[cfg(target_os = "windows")]
use tao::platform::windows::WindowBuilderExtWindows;

mod colors {
    pub const WHITE: [u8; 4] = [255, 255, 255, 255];
    pub const BLACK: [u8; 4] = [0, 0, 0, 255];
    pub const BLUE: [u8; 4] = [56, 143, 255, 150];
    pub const DARK_BLUE: [u8; 4] = [56, 143, 255, 255];
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

    let eyes = paths::eyes();
    let mouth = paths::mouth();
    let table = paths::table();
    let mousepad = paths::mousepad();
    let mouse = paths::mouse();
    let mouse_wheel = paths::mouse_wheel();
    let mouse_button_left = paths::mouse_button_left();
    let mouse_button_right = paths::mouse_button_right();
    let left_hand = paths::left_hand();
    let paws = paths::paws();
    let table_line_left = paths::table_line_left();
    let table_line_right = paths::table_line_right();
    let keys = [
        paths::key_space(),
        paths::key_d(),
        paths::key_s(),
        paths::key_a(),
        paths::key_r(),
        paths::key_e(),
        paths::key_w(),
        paths::key_q(),
        paths::key_7(),
        paths::key_6(),
        paths::key_5(),
        paths::key_4(),
        paths::key_3(),
        paths::key_2(),
        paths::key_1(),
    ];

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

    // little body jiggle
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

    let key_centers: Vec<(f32, f32)> = keys
        .iter()
        .map(|k| {
            let b = k.bounds();
            ((b.left() + b.right()) / 2.0, (b.top() + b.bottom()) / 2.0)
        })
        .collect();

    if click_states.other_click {
        // we randomize because we do NOT want this to be a key logger lmao
        let idx = rand::rng().random_range(0..keys.len());
        let (center_x, center_y) = key_centers[idx];

        for (i, key) in keys.iter().enumerate() {
            if i == idx {
                fill_and_stroke(&mut pixmap, key, transform, &stroke, BLUE, DARK_BLUE);
            } else {
                fill_and_stroke(&mut pixmap, key, transform, &stroke, WHITE, BLACK);
            }
        }

        const PAW_MIDDLE: (f32, f32) = (6.858, 5.153);
        let dx = center_x - PAW_MIDDLE.0;
        let dy = center_y - PAW_MIDDLE.1;
        let paw_scale = center_y / 5.0 + 0.5;

        let left_hand_down = {
            let mut pb = PathBuilder::new();
            pb.move_to(9.988, 0.782);
            pb.cubic_to(
                10.523,
                2.788,
                8.389 + (dx * paw_scale),
                5.554 + dy,
                PAW_MIDDLE.0 + dx,
                PAW_MIDDLE.1 + dy,
            );
            pb.cubic_to(
                5.969 + (dx * paw_scale),
                4.923 + dy,
                6.05,
                2.833,
                7.016,
                1.288,
            );
            pb.finish().unwrap()
        };

        fill_and_stroke(&mut pixmap, &left_hand_down, body_tf, &stroke, WHITE, BLACK);
    } else {
        for key in &keys {
            fill_and_stroke(&mut pixmap, key, transform, &stroke, WHITE, BLACK);
        }
        stroke_path(&mut pixmap, &left_hand, body_tf, &stroke, BLACK);
        fill_and_stroke(&mut pixmap, &paws, body_tf, &stroke, PINK, PINK_STROKE);
    }

    fill_path(&mut pixmap, &eyes, eye_tf, BLACK);
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
