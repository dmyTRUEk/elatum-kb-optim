//! elatum kb optim

use std::fs::read_to_string as read_file_to_string;

use clap::Parser;
use nalgebra::Vector2;
use rand::{rng, Rng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

mod macros;



#[derive(Parser, Debug)]
#[clap(
	about,
	author,
	version,
	help_template = "\
		{before-help}{name} v{version}\n\
		\n\
		{about}\n\
		\n\
		Author: {author}\n\
		\n\
		{usage-heading} {usage}\n\
		\n\
		{all-args}{after-help}\
	",
)]
struct CliArgs {
	dataset_filename: String,
}



// const SWIPE_LEN: f64 = 1.;


const QUALITY_EXCLUDE_CHARS: &str = "0123456789–—|";


fn main() {
	let cli_args = CliArgs::parse();

	let dataset = read_file_to_string(cli_args.dataset_filename).unwrap();

	let msgs: Vec<&str> = dataset
		.split('\n')
		.map(|msg| msg.trim())
		.collect();

	let q_me = Keyboard::m_e().measure_quality(&msgs);
	println!();
	println!("ME layout:\n{}", Keyboard::m_e().to_string3());
	println!();
	println!("Quality ME: {q_me:#?}");
	println!();
	println!("Quality of Random: {:#?}", Keyboard::m_e().shuffled(1.).measure_quality(&msgs));
	println!();

	const N_CORES: usize = 12;
	const N_ITERS: usize = 10_000_000;
	dbg!(N_CORES);
	dbg!(N_ITERS);
	let mut best_kbs_qs: Vec<(Keyboard, KeyboardQuality)> = (0..N_CORES)
		// .into_iter()
		.into_par_iter()
		.map(|_i| {
			let mut kb_best = Keyboard::m_e();
			let mut q_best: KeyboardQuality = q_me;
			for _ in 0..N_ITERS {
				let p = rng().random_range(0. .. 1.);
				let kb = kb_best.clone().shuffled(p);
				let q_new = kb.measure_quality(&msgs);
				if q_new.is_better(q_best) {
					q_best = q_new;
					kb_best = kb.clone();
					println!("Quality New: {q_best:#?}");
				}
			}
			(kb_best, q_best)
		})
		.collect();

	fn index_of_worst_kb(qs: &[KeyboardQuality]) -> Option<usize> {
		println!("{}", "-".repeat(42));
		if qs.len() == 1 { return None }
		'i_loop: for i in 0..qs.len() {
			for j in 0..qs.len() {
				if i == j { continue }
				let is_worse = qs[j].is_worse(qs[i]);
				println!("j={j} is worse than i={i}: {is_worse}");
				if is_worse {
					continue 'i_loop
				}
			}
			return Some(i)
		}
		None
	}

	let mut qs = best_kbs_qs.iter().map(|(_kb, q)| *q).collect::<Vec<KeyboardQuality>>();
	dbg!(qs.len(), &qs);
	while let Some(index_of_worst_kb) = index_of_worst_kb(&qs) {
		dbg!(index_of_worst_kb);
		best_kbs_qs.remove(index_of_worst_kb);
		qs.remove(index_of_worst_kb);
	}

	dbg!(qs.len(), &qs);

	for (kb, q) in best_kbs_qs {
		println!("\n\n\n");
		println!("{}", kb.to_string3());
		println!("{q:#?}");
	}
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



#[derive(Debug, Clone)]
struct Keyboard {
	symbols_locations: Vec<(Option<char>, (Position, Direction))>,
}
impl Keyboard {
	fn m_e() -> Self {
		Self {
			symbols_locations: vec![
				// center:
				(Some('o'), (Position::new(0, 0), Direction::Center)),
				(Some('b'), (Position::new(0, 0), Direction::Right)),
				(Some('u'), (Position::new(0, 0), Direction::Up)),
				(Some('c'), (Position::new(0, 0), Direction::Left)),
				(Some('d'), (Position::new(0, 0), Direction::Down)),
				(Some('p'), (Position::new(0, 0), Direction::RightUp)),
				(Some('q'), (Position::new(0, 0), Direction::LeftUp)),
				(Some('g'), (Position::new(0, 0), Direction::LeftDown)),
				(Some('j'), (Position::new(0, 0), Direction::RightDown)),

				// right:
				(Some('r'), (Position::new(1, 0), Direction::Center)),
				(Some(')'), (Position::new(1, 0), Direction::Right)),
				(None, (Position::new(1, 0), Direction::Up)),
				(Some('m'), (Position::new(1, 0), Direction::Left)),
				(None, (Position::new(1, 0), Direction::Down)),
				(Some('}'), (Position::new(1, 0), Direction::RightUp)),
				(None, (Position::new(1, 0), Direction::LeftUp)),
				(Some('@'), (Position::new(1, 0), Direction::LeftDown)),
				(Some(']'), (Position::new(1, 0), Direction::RightDown)),

				// up:
				(Some('n'), (Position::new(0, 1), Direction::Center)),
				(Some('!'), (Position::new(0, 1), Direction::Right)),
				(Some('^'), (Position::new(0, 1), Direction::Up)),
				(Some('+'), (Position::new(0, 1), Direction::Left)),
				(Some('l'), (Position::new(0, 1), Direction::Down)),
				(None, (Position::new(0, 1), Direction::RightUp)),
				(None, (Position::new(0, 1), Direction::LeftUp)),
				(Some('/'), (Position::new(0, 1), Direction::LeftDown)),
				(Some('\\'), (Position::new(0, 1), Direction::RightDown)),

				// left:
				(Some('h'), (Position::new(-1, 0), Direction::Center)),
				(Some('k'), (Position::new(-1, 0), Direction::Right)),
				(None, (Position::new(-1, 0), Direction::Up)),
				(Some('('), (Position::new(-1, 0), Direction::Left)),
				(None, (Position::new(-1, 0), Direction::Down)),
				(Some('%'), (Position::new(-1, 0), Direction::RightUp)),
				(Some('{'), (Position::new(-1, 0), Direction::LeftUp)),
				(Some('['), (Position::new(-1, 0), Direction::LeftDown)),
				(Some('_'), (Position::new(-1, 0), Direction::RightDown)),

				// down:
				(Some('e'), (Position::new(0, -1), Direction::Center)),
				(Some('z'), (Position::new(0, -1), Direction::Right)),
				(Some('w'), (Position::new(0, -1), Direction::Up)),
				(None, (Position::new(0, -1), Direction::Left)),
				(Some('.'), (Position::new(0, -1), Direction::Down)),
				(Some('\''), (Position::new(0, -1), Direction::RightUp)),
				(Some('"'), (Position::new(0, -1), Direction::LeftUp)),
				(Some(','), (Position::new(0, -1), Direction::LeftDown)),
				(Some(':'), (Position::new(0, -1), Direction::RightDown)),

				// right up:
				(Some('i'), (Position::new(1, 1), Direction::Center)),
				(None, (Position::new(1, 1), Direction::Right)),
				(None, (Position::new(1, 1), Direction::Up)),
				(Some('?'), (Position::new(1, 1), Direction::Left)),
				(Some('='), (Position::new(1, 1), Direction::Down)),
				(None, (Position::new(1, 1), Direction::RightUp)),
				(None, (Position::new(1, 1), Direction::LeftUp)),
				(Some('x'), (Position::new(1, 1), Direction::LeftDown)),
				(None, (Position::new(1, 1), Direction::RightDown)),

				// left up:
				(Some('a'), (Position::new(-1, 1), Direction::Center)),
				(Some('-'), (Position::new(-1, 1), Direction::Right)),
				(None, (Position::new(-1, 1), Direction::Up)),
				(None, (Position::new(-1, 1), Direction::Left)),
				(None, (Position::new(-1, 1), Direction::Down)),
				(None, (Position::new(-1, 1), Direction::RightUp)),
				(None, (Position::new(-1, 1), Direction::LeftUp)),
				(Some('$'), (Position::new(-1, 1), Direction::LeftDown)),
				(Some('v'), (Position::new(-1, 1), Direction::RightDown)),

				// left down:
				(Some('t'), (Position::new(-1, -1), Direction::Center)),
				(Some('*'), (Position::new(-1, -1), Direction::Right)),
				(None, (Position::new(-1, -1), Direction::Up)),
				(Some('<'), (Position::new(-1, -1), Direction::Left)),
				(None, (Position::new(-1, -1), Direction::Down)),
				(Some('y'), (Position::new(-1, -1), Direction::RightUp)),
				(Some('~'), (Position::new(-1, -1), Direction::LeftUp)),
				(None, (Position::new(-1, -1), Direction::LeftDown)),
				(None, (Position::new(-1, -1), Direction::RightDown)), // '\t'

				// right down:
				(Some('s'), (Position::new(1, -1), Direction::Center)),
				(Some('>'), (Position::new(1, -1), Direction::Right)),
				(Some('&'), (Position::new(1, -1), Direction::Up)),
				(Some('#'), (Position::new(1, -1), Direction::Left)),
				(None, (Position::new(1, -1), Direction::Down)),
				(Some('°'), (Position::new(1, -1), Direction::RightUp)),
				(Some('f'), (Position::new(1, -1), Direction::LeftUp)),
				(Some(';'), (Position::new(1, -1), Direction::LeftDown)),
				(None, (Position::new(1, -1), Direction::RightDown)),

				// space:
				(Some(' '), (Position::new(0, -2), Direction::Center)),
				(None, (Position::new(1, -1), Direction::Right)),
				(None, (Position::new(1, -1), Direction::Up)),
				(None, (Position::new(1, -1), Direction::Left)),
				(None, (Position::new(1, -1), Direction::Down)),
				(None, (Position::new(1, -1), Direction::RightUp)),
				(None, (Position::new(1, -1), Direction::LeftUp)),
				(None, (Position::new(1, -1), Direction::LeftDown)),
				(None, (Position::new(1, -1), Direction::RightDown)),
			],
		}
	}

	fn optimize(&mut self) {
		todo!()
	}

	fn optimized(mut self) -> Self {
		self.optimize();
		self
	}


	fn measure_quality(&self, dataset: &[&str]) -> KeyboardQuality {
		let mut keyboard_quality = KeyboardQuality::new();
		// relative to center button
		for dataset_element in dataset {
			let mut finger_position = Position::zeros();
			for symbol in dataset_element.chars() {
				if QUALITY_EXCLUDE_CHARS.contains(symbol) { continue }
				let (_symbol, (target_position, target_direction)): &(Option<char>, (Position, Direction)) =
					&self.symbols_locations.iter().find(|&sl| sl.0 == Some(symbol)).unwrap_or_else(|| panic!("{symbol}"));
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

	fn shuffled(mut self, p: f32) -> Self {
		self.shuffle(p);
		self
	}

	fn shuffle(&mut self, p: f32) {
		const EXCLUDE_CHARS: &str = "";
		let n = self.symbols_locations.len();
		let mut rng = rng();
		for i in 0..n {
			if rng.random_range(0. .. 1.) < p {
				let j = rng.random_range(0..n);
				if self.symbols_locations[i].0.map_or(false, |c| EXCLUDE_CHARS.contains(c)) ||
					self.symbols_locations[j].0.map_or(false, |c| EXCLUDE_CHARS.contains(c)) { continue }
				swap!(
					self.symbols_locations[i].1,
					self.symbols_locations[j].1
				);
			}
		}
	}

	fn to_string(&self) -> String {
		let x_min = self.symbols_locations.iter().map(|(_c, (p, _d))| p.x).min().unwrap();
		let x_max = self.symbols_locations.iter().map(|(_c, (p, _d))| p.x).max().unwrap();
		let y_min = self.symbols_locations.iter().map(|(_c, (p, _d))| p.y).min().unwrap();
		let y_max = self.symbols_locations.iter().map(|(_c, (p, _d))| p.y).max().unwrap();
		let x_range = x_max - x_min + 1;
		let y_range = y_max - y_min + 1;
		// assert_eq!(3, x_range);
		// assert_eq!(4, y_range);

		let string_size_x = (x_range * 4 - 1) as usize;
		let string_size_y = (y_range * 4 - 1) as usize;
		// assert_eq!(11, string_size_x);
		// assert_eq!(15, string_size_y);

		let mut s: Vec<Vec<char>> = vec![vec![' '; string_size_x]; string_size_y];
		for (c, (p, d)) in self.symbols_locations.iter() {
			let Some(c) = c else { continue };
			let x = ( (p.x - x_min)*4 + d.to_x() + 1 ) as usize;
			let y = ( (p.y - y_min)*4 + d.to_y() + 1 ) as usize;
			let c = if *c != ' ' { *c } else { '␣' };
			s[string_size_y - y - 1][x] = c;
		}

		s.iter()
			.map(|l|
				l.iter()
					.map(|c| c.to_string())
					.collect::<Vec<String>>()
					.concat()
			)
			.collect::<Vec<String>>()
			.join("\n")
	}

	fn to_string2(&self) -> String {
		self.to_string()
			.lines()
			.map(|l| {
				let mut l = l.to_string();
				for i in (0..l.chars().count()).rev() {
					if i % 4 == 3 {
						l = [&l.chars().take(i).collect::<String>(), " ", &l.chars().skip(i).collect::<String>()].concat();
					}
				}
				l
			})
			.collect::<Vec<String>>()
			.join("\n")
	}

	fn to_string3(&self) -> String {
		self.to_string()
			.lines()
			.map(|l| {
				let mut l = l.to_string();
				for i in (1..l.chars().count()).rev() {
					l = [&l.chars().take(i).collect::<String>(), " ", &l.chars().skip(i).collect::<String>()].concat();
				}
				l
			})
			.collect::<Vec<String>>()
			.join("\n")
	}
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

	fn is_better(&self, other: Self) -> bool {
		self.is_better_by_travels_than(other)
	}

	fn is_worse(&self, other: Self) -> bool {
		!self.is_better(other)
	}

	fn is_totally_better_than(&self, other: Self) -> bool {
		self.taps < other.taps &&
		self.straight_swipes < other.straight_swipes &&
		self.diagonal_swipes < other.diagonal_swipes &&
		self.straight_travels < other.straight_travels &&
		self.diagonal_travels < other.diagonal_travels
	}

	fn is_4_better_than(&self, other: Self) -> bool {
		(//self.taps < other.taps &&
		self.straight_swipes < other.straight_swipes &&
		self.diagonal_swipes < other.diagonal_swipes &&
		self.straight_travels < other.straight_travels &&
		self.diagonal_travels < other.diagonal_travels)
		||
		(self.taps < other.taps &&
		// self.straight_swipes < other.straight_swipes &&
		self.diagonal_swipes < other.diagonal_swipes &&
		self.straight_travels < other.straight_travels &&
		self.diagonal_travels < other.diagonal_travels)
		||
		(self.taps < other.taps &&
		self.straight_swipes < other.straight_swipes &&
		// self.diagonal_swipes < other.diagonal_swipes &&
		self.straight_travels < other.straight_travels &&
		self.diagonal_travels < other.diagonal_travels)
		||
		(self.taps < other.taps &&
		self.straight_swipes < other.straight_swipes &&
		self.diagonal_swipes < other.diagonal_swipes &&
		// self.straight_travels < other.straight_travels &&
		self.diagonal_travels < other.diagonal_travels)
		||
		(self.taps < other.taps &&
		self.straight_swipes < other.straight_swipes &&
		self.diagonal_swipes < other.diagonal_swipes &&
		self.straight_travels < other.straight_travels //&&
		/*self.diagonal_travels < other.diagonal_travels*/)
	}

	fn is_better_exc_taps_than(&self, other: Self) -> bool {
		// self.taps < other.taps &&
		self.straight_swipes < other.straight_swipes &&
		self.diagonal_swipes < other.diagonal_swipes &&
		self.straight_travels < other.straight_travels &&
		self.diagonal_travels < other.diagonal_travels
	}

	fn is_better_by_travels_than(&self, other: Self) -> bool {
		// self.taps < other.taps &&
		// self.straight_swipes < other.straight_swipes &&
		// self.diagonal_swipes < other.diagonal_swipes &&
		self.straight_travels < other.straight_travels &&
		self.diagonal_travels < other.diagonal_travels
	}

	// fn is_better_than(&self, other: Self) -> bool {
	// 	?
	// }
}


#[derive(Debug, Clone, Copy)]
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
impl Direction {
	fn to_x(&self) -> i8 {
		use Direction::*;
		match self {
			Center | Up | Down => 0,
			Left | LeftUp | LeftDown => -1,
			Right | RightUp | RightDown => 1,
		}
	}
	fn to_y(&self) -> i8 {
		use Direction::*;
		match self {
			Center | Right | Left => 0,
			Down | LeftDown | RightDown => -1,
			Up | LeftUp | RightUp => 1,
		}
	}
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
			Keyboard::m_e().measure_quality(&["good"])
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
			Keyboard::m_e().measure_quality(&["elatum"])
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
			Keyboard::m_e().measure_quality(&["the keyboard"])
		);
	}
}



#[cfg(test)]
mod to_string {
	use super::*;
	#[test]
	fn m_e() {
		let expected = [
			r#"     ^     "#,
			r#" a- +n! ?i "#,
			r#"$ v /l\ x= "#,
			r#"           "#,
			r#"{ % qup   }"#,
			r#"(hk cob mr)"#,
			r#"[ _ gdj @ ]"#,
			r#"           "#,
			r#"~ y "w' f&°"#,
			r#"<t*  ez #s>"#,
			r#"    ,.: ;  "#,
			r#"           "#,
			r#"           "#,
			r#"     ␣     "#,
			r#"           "#,
		].join("\n");
		let actual = Keyboard::m_e().to_string();
		println!("expected:\n{expected}");
		println!();
		println!("actual:\n{actual}");
		for (i, (ce, ca)) in expected.chars().zip(actual.chars()).enumerate() {
			assert_eq!(ce, ca, "error at index {i}");
		}
		assert_eq!(expected, actual)
	}
}



#[cfg(test)]
mod to_string2 {
	use super::*;
	#[test]
	fn m_e() {
		let expected = [
			r#"      ^      "#,
			r#" a-  +n!  ?i "#,
			r#"$ v  /l\  x= "#,
			r#"             "#,
			r#"{ %  qup    }"#,
			r#"(hk  cob  mr)"#,
			r#"[ _  gdj  @ ]"#,
			r#"             "#,
			r#"~ y  "w'  f&°"#,
			r#"<t*   ez  #s>"#,
			r#"     ,.:  ;  "#,
			r#"             "#,
			r#"             "#,
			r#"      ␣      "#,
			r#"             "#,
		].join("\n");
		let actual = Keyboard::m_e().to_string2();
		println!("expected:\n{expected}");
		println!();
		println!("actual:\n{actual}");
		for (i, (ce, ca)) in expected.chars().zip(actual.chars()).enumerate() {
			assert_eq!(ce, ca, "error at index {i}");
		}
		assert_eq!(expected, actual)
	}
}



#[cfg(test)]
mod to_string3 {
	use super::*;
	#[test]
	fn m_e() {
		let expected = [
			r#"          ^          "#,
			r#"  a -   + n !   ? i  "#,
			r#"$   v   / l \   x =  "#,
			r#"                     "#,
			r#"{   %   q u p       }"#,
			r#"( h k   c o b   m r )"#,
			r#"[   _   g d j   @   ]"#,
			r#"                     "#,
			r#"~   y   " w '   f & °"#,
			r#"< t *     e z   # s >"#,
			r#"        , . :   ;    "#,
			r#"                     "#,
			r#"                     "#,
			r#"          ␣          "#,
			r#"                     "#,
		].join("\n");
		let actual = Keyboard::m_e().to_string3();
		println!("expected:\n{expected}");
		println!();
		println!("actual:\n{actual}");
		for (i, (ce, ca)) in expected.chars().zip(actual.chars()).enumerate() {
			println!("i: {i} , ce: `{ce}` , ca: `{ca}`");
			assert_eq!(ce, ca, "error at index {i}");
		}
		assert_eq!(expected, actual)
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
