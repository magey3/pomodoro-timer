use raylib::core::text::measure_text;
use raylib::prelude::*;
use std::time::{Duration, SystemTime};

#[derive(Clone, Copy, PartialEq)]
enum State {
	Focus,
	Break,
	LongBreak,
	NotRunning,
}

fn state_to_string(state: &State) -> &str {
	match state {
		State::Focus => "Focus",
		State::Break => "Break",
		State::LongBreak => "Long break",
		State::NotRunning => "Not running",
	}
}

fn draw_text_centered(text: &str, d: &mut RaylibDrawHandle) {
	d.draw_text(
		text,
		d.get_screen_width() / 2 - measure_text(&text, d.get_screen_width() / 8) / 2,
		d.get_screen_height() / 2 - d.get_screen_height() / 8,
		d.get_screen_width() / 8,
		Color::WHITE,
	);
}

fn sec_to_m_sec(t: u64) -> String {
	(t / 60).to_string() + ":" + &(t % 60).to_string()
}

fn main() {
	let (mut rl, thread) = raylib::init()
		.size(640, 480)
		.resizable()
		.title("Pomodoro")
		.build();

	rl.set_target_fps(20);

	let focus_time = 25 * 60;
	let break_time = 5 * 60;
	let long_break_time = 15 * 60;
	let mut focus_amount = 0;

	let mut state = State::NotRunning;
	let mut next_state = State::Focus;
	let mut start = SystemTime::now();
	while !rl.window_should_close() {
		let mut d = rl.begin_drawing(&thread);

		match state {
			State::NotRunning => {
				d.clear_background(Color::DARKGRAY);
				draw_text_centered(
					&("Next: ".to_owned() + state_to_string(&next_state)),
					&mut d,
				);

				let rec = Rectangle::new(
					(d.get_screen_width() / 4) as f32,
					(2 * d.get_screen_height() / 3) as f32,
					(d.get_screen_width() / 2) as f32,
					(d.get_screen_height() / 5) as f32,
				);

				d.draw_rectangle_rec(rec, Color::WHITE);
				let text = "Start!";
				let vertical_margin = 10.0;
				let font_size = (rec.height - vertical_margin) as i32;
				d.draw_text(
					text,
					(rec.width as i32 - measure_text(text, font_size)) / 2 + rec.x as i32,
					(rec.y + vertical_margin) as i32,
					font_size,
					Color::DARKGRAY,
				);

				if d.is_key_pressed(KeyboardKey::KEY_SPACE)
					|| (rec.check_collision_point_rec(d.get_mouse_position())
						&& d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON))
				{
					state = next_state.to_owned();
					start = SystemTime::now();
				}
			}
			State::Focus => {
				d.clear_background(Color::RED);
				let elapsed_time = start.elapsed().unwrap().as_secs();
				if elapsed_time >= focus_time {
					state = State::NotRunning;

					focus_amount += 1;
					if focus_amount >= 4 {
						next_state = State::LongBreak;
						focus_amount = 0;
					} else {
						next_state = State::Break;
					}
				}
				let text = sec_to_m_sec(focus_time - start.elapsed().unwrap().as_secs());
				draw_text_centered(&text, &mut d);
			}
			State::Break => {
				d.clear_background(Color::BLUE);

				let elapsed_time = start.elapsed().unwrap().as_secs();
				if elapsed_time >= break_time {
					state = State::NotRunning;
					next_state = State::Focus;
				}
				let text = sec_to_m_sec(break_time - start.elapsed().unwrap().as_secs());
				draw_text_centered(&text, &mut d);
			}
			State::LongBreak => {
				d.clear_background(Color::DARKBLUE);
				let elapsed_time = start.elapsed().unwrap().as_secs();
				if elapsed_time >= long_break_time {
					state = State::NotRunning;
					next_state = State::Focus;
				}
				let text = sec_to_m_sec(long_break_time - start.elapsed().unwrap().as_secs());
				draw_text_centered(&text, &mut d);
			}
		}
	}
}
