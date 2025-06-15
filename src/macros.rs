//! Macros.
//!
//! Debug they by [`trace_macros!`]:
//! ```
//! trace_macros!(true);
//! your_macro!(…);
//! trace_macros!(false);
//! ```

#![allow(unused_macros)]



#[macro_export]
macro_rules! swap {
	($a:expr, $b:expr) => { {
		let t = $a;
		$a = $b;
		$b = t;
	} };
}



#[macro_export]
macro_rules! unmut {
	($x:tt) => {
		let $x = $x;
	};
}



// src: https://internals.rust-lang.org/t/mutually-exclusive-feature-flags/8601/7
#[macro_export]
macro_rules! assert_unique_feature {
	() => {};
	($first:tt $(,$rest:tt)* $(,)?) => {
		$(
			#[cfg(all(feature=$first, feature=$rest))]
			compile_error!(concat!("features `", $first, "` and `", $rest, "` are mutually exlusive"));
		)*
		assert_unique_feature!($($rest),*);
	}
}



/// Box::new(...)
#[macro_export]
macro_rules! bx {
	($expr:expr) => {
		Box::new($expr)
	};
	($expr1:expr, $expr2:expr) => {
		Box::new(($expr1, $expr2))
	};
}



#[cfg(test)]
mod swap {
	#[test]
	fn _4_5() {
		{
			let mut x: i32 = 4;
			let mut y: i32 = 5;
			assert!(x < y);
			swap!(x, y);
			assert!(x > y);
		}
		{
			let mut x: i32 = 4;
			let mut y: i32 = 5;
			assert!(x < y);
			swap!(y, x);
			assert!(x > y);
		}
		{
			let mut x: f32 = 4.0;
			let mut y: f32 = 5.0;
			assert!(x < y);
			swap!(x, y);
			assert!(x > y);
		}
		{
			let mut array: [i32; 5] = [0, 1, 2, 3, 4];
			assert_eq!([0, 1, 2, 3, 4], array);
			swap!(array[1], array[3]);
			assert_eq!([0, 3, 2, 1, 4], array);
		}
	}
}

#[cfg(test)]
mod unmut {
	// #[test]
	// #[compile_fail] // TODO
	// fn cant_mutate_unmuted() {
	// 	let mut x = 42;
	// 	assert_eq!(42, x);
	// 	unmut!(x);
	// 	assert_eq!(42, x);
	// 	x = 145;
	// 	// assert_eq!(145, x);
	// }
	#[test]
	fn _1() {
		let x = 42;
		assert_eq!(42, x);
		unmut!(x);
		assert_eq!(42, x);
		let x = 145;
		assert_eq!(145, x);
	}
	#[test]
	fn _2() {
		let mut x = 42;
		assert_eq!(42, x);
		x = 137;
		assert_eq!(137, x);
		unmut!(x);
		assert_eq!(137, x);
		let x = 145;
		assert_eq!(145, x);
	}
}


#[cfg(test)]
mod bx {
	#[test]
	fn _42() { assert_eq!(Box::new(42), bx![42]) }
	fn _42_137() { assert_eq!(Box::new((42, 137)), bx![42, 137]) }
}

