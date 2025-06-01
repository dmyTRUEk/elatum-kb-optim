//! elatum kb optim

use nalgebra::Vector2;


// const SWIPE_LEN: f64 = 1.;


fn main() {
	println!("Hello, world!");
}


#[derive(Debug, PartialEq, Eq)]
enum Constraint<T> {
	Free(T),
	Fixed(T),
}


#[derive(Debug, PartialEq, Eq)]
enum KeyboardAction {
	Text { text: char },
}


type Position = Vector2<i8>;


struct Keyboard {
	symbols_locations: Vec<(char, (Position, Direction))>,
}
impl Keyboard {
	fn m_e() -> Self {
		Self {
			symbols_locations: vec![
				// center:
				('o', (Position::new(0, 0), Direction::Center)),
				('b', (Position::new(0, 0), Direction::Right)),
				('u', (Position::new(0, 0), Direction::Up)),
				('c', (Position::new(0, 0), Direction::Left)),
				('d', (Position::new(0, 0), Direction::Down)),
				('p', (Position::new(0, 0), Direction::RightUp)),
				('q', (Position::new(0, 0), Direction::LeftUp)),
				('g', (Position::new(0, 0), Direction::LeftDown)),
				('j', (Position::new(0, 0), Direction::RightDown)),

				// right:
				('r', (Position::new(1, 0), Direction::Center)),
				(')', (Position::new(1, 0), Direction::Right)),
				// ('', (Position::new(1, 0), Direction::Up)),
				('m', (Position::new(1, 0), Direction::Left)),
				// ('', (Position::new(1, 0), Direction::Down)),
				('}', (Position::new(1, 0), Direction::RightUp)),
				// ('', (Position::new(1, 0), Direction::LeftUp)),
				('@', (Position::new(1, 0), Direction::LeftDown)),
				(']', (Position::new(1, 0), Direction::RightDown)),

				// up:
				('n', (Position::new(0, 1), Direction::Center)),
				('!', (Position::new(0, 1), Direction::Right)),
				('^', (Position::new(0, 1), Direction::Up)),
				('+', (Position::new(0, 1), Direction::Left)),
				('l', (Position::new(0, 1), Direction::Down)),
				// ('', (Position::new(0, 1), Direction::RightUp)),
				// ('', (Position::new(0, 1), Direction::LeftUp)),
				('/', (Position::new(0, 1), Direction::LeftDown)),
				('\\', (Position::new(0, 1), Direction::RightDown)),

				// left:
				('h', (Position::new(-1, 0), Direction::Center)),
				('k', (Position::new(-1, 0), Direction::Right)),
				// ('', (Position::new(-1, 0), Direction::Up)),
				('(', (Position::new(-1, 0), Direction::Left)),
				// ('', (Position::new(-1, 0), Direction::Down)),
				('%', (Position::new(-1, 0), Direction::RightUp)),
				('{', (Position::new(-1, 0), Direction::LeftUp)),
				('[', (Position::new(-1, 0), Direction::LeftDown)),
				('_', (Position::new(-1, 0), Direction::RightDown)),

				// down:
				('e', (Position::new(0, -1), Direction::Center)),
				('z', (Position::new(0, -1), Direction::Right)),
				('w', (Position::new(0, -1), Direction::Up)),
				// ('', (Position::new(0, -1), Direction::Left)),
				('.', (Position::new(0, -1), Direction::Down)),
				('\'', (Position::new(0, -1), Direction::RightUp)),
				('"', (Position::new(0, -1), Direction::LeftUp)),
				(',', (Position::new(0, -1), Direction::LeftDown)),
				(':', (Position::new(0, -1), Direction::RightDown)),

				// right up:
				('i', (Position::new(1, 1), Direction::Center)),
				// ('', (Position::new(1, 1), Direction::Right)),
				// ('', (Position::new(1, 1), Direction::Up)),
				('?', (Position::new(1, 1), Direction::Left)),
				('=', (Position::new(1, 1), Direction::Down)),
				// ('', (Position::new(1, 1), Direction::RightUp)),
				// ('', (Position::new(1, 1), Direction::LeftUp)),
				('x', (Position::new(1, 1), Direction::LeftDown)),
				// ('', (Position::new(1, 1), Direction::RightDown)),

				// left up:
				('a', (Position::new(-1, 1), Direction::Center)),
				('-', (Position::new(-1, 1), Direction::Right)),
				// ('', (Position::new(-1, 1), Direction::Up)),
				// ('', (Position::new(-1, 1), Direction::Left)),
				// ('', (Position::new(-1, 1), Direction::Down)),
				// ('', (Position::new(-1, 1), Direction::RightUp)),
				// ('', (Position::new(-1, 1), Direction::LeftUp)),
				('$', (Position::new(-1, 1), Direction::LeftDown)),
				('v', (Position::new(-1, 1), Direction::RightDown)),

				// left down:
				('t', (Position::new(-1, -1), Direction::Center)),
				('*', (Position::new(-1, -1), Direction::Right)),
				// ('', (Position::new(-1, -1), Direction::Up)),
				('<', (Position::new(-1, -1), Direction::Left)),
				// ('', (Position::new(-1, -1), Direction::Down)),
				('y', (Position::new(-1, -1), Direction::RightUp)),
				('~', (Position::new(-1, -1), Direction::LeftUp)),
				// ('', (Position::new(-1, -1), Direction::LeftDown)),
				('\t', (Position::new(-1, -1), Direction::RightDown)),

				// right down:
				('s', (Position::new(1, -1), Direction::Center)),
				('>', (Position::new(1, -1), Direction::Right)),
				('&', (Position::new(1, -1), Direction::Up)),
				('#', (Position::new(1, -1), Direction::Left)),
				// ('', (Position::new(1, -1), Direction::Down)),
				('°', (Position::new(1, -1), Direction::RightUp)),
				('f', (Position::new(1, -1), Direction::LeftUp)),
				(';', (Position::new(1, -1), Direction::LeftDown)),
				// ('', (Position::new(1, -1), Direction::RightDown)),

				// space:
				(' ', (Position::new(0, -2), Direction::Center)),
			],
		}
	}

	fn measure_quality(&self, dataset: Vec<String>) -> KeyboardQuality {
		let mut keyboard_quality = KeyboardQuality::new();
		// relative to center button
		for dataset_element in dataset {
			let mut finger_position = Position::zeros();
			for symbol in dataset_element.chars() {
				let (_symbol, (target_position, target_direction)): &(char, (Position, Direction)) =
					&self.symbols_locations.iter().find(|&sl| sl.0 == symbol).unwrap();
				// println!(
				//     "target_position = ({tpx}, {tpy}), target_direction = {target_direction:?}",
				//     tpx=target_position.x,
				//     tpy=target_position.y,
				// );
				let (straight_travels, diagonal_travels) = count_travels(finger_position, *target_position);
				keyboard_quality.straight_travels += straight_travels as u64;
				keyboard_quality.diagonal_travels += diagonal_travels as u64;
				finger_position = *target_position;
				match target_direction {
					Direction::Center => {
						keyboard_quality.taps += 1;
					}
					Direction::Right | Direction::Up | Direction::Left | Direction::Down => {
						keyboard_quality.straight_swipes += 1;
						finger_position += match target_direction {
							Direction::Right => Position::new(1, 0),
							Direction::Up => Position::new(0, 1),
							Direction::Left => Position::new(-1, 0),
							Direction::Down => Position::new(0, -1),
							_ => unreachable!()
						};
					}
					Direction::RightUp | Direction::LeftUp | Direction::LeftDown | Direction::RightDown => {
						keyboard_quality.diagonal_swipes += 1;
						finger_position += match target_direction {
							Direction::RightUp => Position::new(1, 1),
							Direction::LeftUp => Position::new(-1, 1),
							Direction::LeftDown => Position::new(-1, -1),
							Direction::RightDown => Position::new(1, -1),
							_ => unreachable!()
						};
					}
				}
			}
		}
		keyboard_quality
	}
}


#[derive(Debug, PartialEq, Eq)]
struct KeyboardQuality {
	taps: u64,
	straight_swipes: u64,
	diagonal_swipes: u64,

	/// Measured in `button sizes`.
	///
	/// If finger traveled from Up button to Down button,
	/// then `straight_travels` increase by 2.
	straight_travels: u64,

	/// Measured in `√2 * button sizes`.
	///
	/// If finger traveled from Left Up button to Right Down button,
	/// then `diagonal_travels` increase by 2.
	diagonal_travels: u64,
}
impl KeyboardQuality {
	fn new() -> Self {
		Self {
			taps: 0,
			straight_swipes: 0,
			diagonal_swipes: 0,
			straight_travels: 0,
			diagonal_travels: 0,
		}
	}
}


#[derive(Debug)]
enum Direction {
	Center,
	Right,
	Up,
	Left,
	Down,
	RightUp,
	LeftUp,
	LeftDown,
	RightDown,
}


/// Returns number of `straight_travels` and `diagonal_travels`
/// required to get from `position_1` to `position_2`.
fn count_travels(position_1: Position, position_2: Position) -> (u32, u32) {
	fn calc_delta_vector(position_1: Position, position_2: Position) -> Vector2<i32> {
		position_2.cast::<i32>() - position_1.cast::<i32>()
	}
	fn delta_to_travels(mut delta: Vector2<i32>) -> (u32, u32) {
		// delta = Vector2::new(delta.x.abs(), delta.y.abs());
		delta = delta.abs();
		let (x, y) = (delta.x as u32, delta.y as u32);
		let diagonal = x.min(y);
		(x.max(y) - diagonal, diagonal)
	}
	delta_to_travels(calc_delta_vector(position_1, position_2))
}





#[cfg(test)]
mod measure_quality {
	use super::{KeyboardQuality, Keyboard};

	#[test]
	fn good() {
		assert_eq!(
			KeyboardQuality {
				taps: 2,
				straight_swipes: 1,
				diagonal_swipes: 1,
				straight_travels: 0,
				diagonal_travels: 1,
			},
			Keyboard::m_e().measure_quality(vec!["good".to_string()])
		);
	}

	#[test]
	fn elatum() {
		assert_eq!(
			KeyboardQuality {
				taps: 3,
				straight_swipes: 3,
				diagonal_swipes: 0,
				straight_travels: 5,
				diagonal_travels: 3,
			},
			Keyboard::m_e().measure_quality(vec!["elatum".to_string()])
		);
	}

	#[test]
	fn the_keyboard() {
		assert_eq!(
			KeyboardQuality {
				taps: 8,
				straight_swipes: 3,
				diagonal_swipes: 1,
				straight_travels: 8,
				diagonal_travels: 5,
			},
			Keyboard::m_e().measure_quality(vec!["the keyboard".to_string()])
		);
	}
}



#[cfg(test)]
mod count_travels {
	use super::{count_travels, Position};

	// trivial case
	#[test] fn from_0_0_to_0_0() { assert_eq!((0, 0), count_travels(Position::new(0, 0), Position::new(0, 0))); }

	// one straight
	#[test] fn from_0_0_to_0_1() { assert_eq!((1, 0), count_travels(Position::new(0, 0), Position::new(0, 1))) }
	#[test] fn from_0_0_to_1_0() { assert_eq!((1, 0), count_travels(Position::new(0, 0), Position::new(1, 0))) }
	#[test] fn from_0_0_to_0_m1() { assert_eq!((1, 0), count_travels(Position::new(0, 0), Position::new(0, -1))) }
	#[test] fn from_0_0_to_m1_0() { assert_eq!((1, 0), count_travels(Position::new(0, 0), Position::new(-1, 0))) }

	// one diagonal
	#[test] fn from_0_0_to_1_1() { assert_eq!((0, 1), count_travels(Position::new(0, 0), Position::new(1, 1))) }
	#[test] fn from_0_0_to_1_m1() { assert_eq!((0, 1), count_travels(Position::new(0, 0), Position::new(1, -1))) }
	#[test] fn from_0_0_to_m1_1() { assert_eq!((0, 1), count_travels(Position::new(0, 0), Position::new(-1, 1))) }
	#[test] fn from_0_0_to_m1_m1() { assert_eq!((0, 1), count_travels(Position::new(0, 0), Position::new(-1, -1))) }

	// two straight from zero
	#[test] fn from_0_0_to_0_2() { assert_eq!((2, 0), count_travels(Position::new(0, 0), Position::new(0, 2))) }
	#[test] fn from_0_0_to_2_0() { assert_eq!((2, 0), count_travels(Position::new(0, 0), Position::new(2, 0))) }
	#[test] fn from_0_0_to_0_m2() { assert_eq!((2, 0), count_travels(Position::new(0, 0), Position::new(0, -2))) }
	#[test] fn from_0_0_to_m2_0() { assert_eq!((2, 0), count_travels(Position::new(0, 0), Position::new(-2, 0))) }

	// two straight
	#[test] fn from_0_m1_to_0_1() { assert_eq!((2, 0), count_travels(Position::new(0, -1), Position::new(0, 1))) }
	#[test] fn from_0_1_to_0_m1() { assert_eq!((2, 0), count_travels(Position::new(0, 1), Position::new(0, -1))) }
	#[test] fn from_m1_0_to_1_0() { assert_eq!((2, 0), count_travels(Position::new(-1, 0), Position::new(1, 0))) }
	#[test] fn from_1_0_to_m1_0() { assert_eq!((2, 0), count_travels(Position::new(1, 0), Position::new(-1, 0))) }

	// two diagonals
	#[test] fn from_1_1_to_m1_m1() { assert_eq!((0, 2), count_travels(Position::new(1, 1), Position::new(-1, -1))) }
	#[test] fn from_1_m1_to_m1_1() { assert_eq!((0, 2), count_travels(Position::new(1, -1), Position::new(-1, 1))) }
	#[test] fn from_m1_1_to_1_m1() { assert_eq!((0, 2), count_travels(Position::new(-1, 1), Position::new(1, -1))) }
	#[test] fn from_m1_m1_to_1_1() { assert_eq!((0, 2), count_travels(Position::new(-1, -1), Position::new(1, 1))) }

	// one straight & one diagonal
	#[test] fn from_m1_0_to_1_1() { assert_eq!((1, 1), count_travels(Position::new(-1, 0), Position::new(1, 1))) }
	#[test] fn from_m1_0_to_1_m1() { assert_eq!((1, 1), count_travels(Position::new(-1, 0), Position::new(1, -1))) }
	#[test] fn from_1_1_to_m1_0() { assert_eq!((1, 1), count_travels(Position::new(1, 1), Position::new(-1, 0))) }
	#[test] fn from_1_m1_to_m1_0() { assert_eq!((1, 1), count_travels(Position::new(1, -1), Position::new(-1, 0))) }

	#[test] fn from_0_m1_to_1_1() { assert_eq!((1, 1), count_travels(Position::new(0, -1), Position::new(1, 1))) }
	#[test] fn from_0_m1_to_m1_1() { assert_eq!((1, 1), count_travels(Position::new(0, -1), Position::new(-1, 1))) }
	#[test] fn from_1_1_to_0_m1() { assert_eq!((1, 1), count_travels(Position::new(1, 1), Position::new(0, -1))) }
	#[test] fn from_m1_1_to_0_m1() { assert_eq!((1, 1), count_travels(Position::new(-1, 1), Position::new(0, -1))) }

	// long:
	#[test] fn from_0_0_to_42_0() { assert_eq!((42, 0), count_travels(Position::new(0, 0), Position::new(42, 0))) }
	#[test] fn from_0_0_to_0_42() { assert_eq!((42, 0), count_travels(Position::new(0, 0), Position::new(0, 42))) }
	#[test] fn from_m21_0_to_21_0() { assert_eq!((42, 0), count_travels(Position::new(-21, 0), Position::new(21, 0))) }
	#[test] fn from_0_m21_to_0_21() { assert_eq!((42, 0), count_travels(Position::new(0, -21), Position::new(0, 21))) }
	#[test] fn from_42_0_to_0_0() { assert_eq!((42, 0), count_travels(Position::new(42, 0), Position::new(0, 0))) }
	#[test] fn from_0_42_to_0_0() { assert_eq!((42, 0), count_travels(Position::new(0, 42), Position::new(0, 0))) }
	#[test] fn from_21_0_to_m21_0() { assert_eq!((42, 0), count_travels(Position::new(21, 0), Position::new(-21, 0))) }
	#[test] fn from_0_21_to_0_m21() { assert_eq!((42, 0), count_travels(Position::new(0, 21), Position::new(0, -21))) }

	// complex:
	#[test] fn from_0_0_to_10_20() { assert_eq!((10, 10), count_travels(Position::new(0, 0), Position::new(10, 20))) }
	#[test] fn from_52_m73_to_m79_46() { assert_eq!((12, 119), count_travels(Position::new(52, -73), Position::new(-79, 46))) }
	#[test] fn from_m41_m71_to_61_m20() { assert_eq!((51, 51), count_travels(Position::new(-41, -71), Position::new(61, -20))) }
	#[test] fn from_m78_5_to_19_m94() { assert_eq!((2, 97), count_travels(Position::new(-78, 5), Position::new(19, -94))) }
}
