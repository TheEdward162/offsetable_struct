pub mod util;

/// Creates a `repr(C)` struct and a companion offsets struct which represents byte offsets of the fields.
///
/// ```
/// # #[macro_use] extern crate offsetable_struct;
/// offsetable_struct! {
/// 	#[derive(Debug)]
/// 	pub struct Name {
/// 		pub a: f32,
/// 		pub b: [f32; 4],
/// 		c: u8
/// 	} repr(C) as NameOffsets
/// }
/// ```
///
/// expands to:
/// ```
/// #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
/// pub struct NameOffsets {
/// 	pub a: usize,
/// 	pub b: usize,
/// 	c: usize
/// }
///
/// #[derive(Debug)]
/// #[repr(C)]
/// pub struct Name {
/// 	pub a: f32,
/// 	pub b: [f32; 4],
/// 	c: u8
/// }
/// impl Name {
/// 	#[allow(unused_variables)]
/// 	pub const fn offsets() -> NameOffsets {
/// 		let current_offset: usize = 0;
///
/// 		let a = offsetable_struct::util::align_up(
/// 			current_offset,
/// 			std::mem::align_of::<f32>()
/// 		);
/// 		let current_offset = a + std::mem::size_of::<f32>();
///
/// 		let b = offsetable_struct::util::align_up(
/// 			current_offset,
/// 			std::mem::align_of::<[f32; 4]>()
/// 		);
/// 		let current_offset = b + std::mem::size_of::<[f32; 4]>();
///
/// 		let c = offsetable_struct::util::align_up(
/// 			current_offset,
/// 			std::mem::align_of::<u8>()
/// 		);
/// 		let current_offset = c + std::mem::size_of::<u8>();
///
/// 		NameOffsets { a, b, c }
/// 	}
/// }
/// ```
#[macro_export]
macro_rules! offsetable_struct {
	(
		$( #[$attribute: meta] )*
		$struct_vis: vis struct $name: ident {
			$(
				$field_vis: vis $field: ident: $ftype: ty
			),*
		} repr(C) as $offsets_name: ident
	) => {
		#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
		$struct_vis struct $offsets_name {
			$(
				$field_vis $field: usize
			),*
		}

		$( #[$attribute] )*
		#[repr(C)]
		$struct_vis struct $name {
			$(
				$field_vis $field: $ftype
			),*
		}
		impl $name {
			/// Returns a struct describing offsets of each field from the start of the struct.
			///
			/// This is mainly useful for things like vertex data.
			#[allow(unused_variables)]
			pub const fn offsets() -> $offsets_name {
				let current_offset: usize = 0;

				$(
					let $field = $crate::util::align_up(current_offset, std::mem::align_of::<$ftype>());
					let current_offset = $field + std::mem::size_of::<$ftype>();
				)*

				$offsets_name {
					$(
						$field
					),*
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		offsetable_struct! {
			#[derive(Debug)]
			pub struct Foo {
				pub a: f32,
				pub b: [f32; 4],
				c: u8
			} repr(C) as FooOffsets
		}

		assert_eq!(Foo::offsets().a, 0);
		assert_eq!(Foo::offsets().b, 4);
		assert_eq!(Foo::offsets().c, 5 * 4);
	}
}
