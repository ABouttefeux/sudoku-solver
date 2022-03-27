#![doc = include_str!("../README.md")]
// TODO

//#![warn(clippy::as_conversions)]
#![warn(clippy::cast_sign_loss)]
#![warn(clippy::cast_possible_truncation)]
#![warn(clippy::cast_possible_wrap)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::cognitive_complexity)]
#![warn(clippy::default_numeric_fallback)]
#![warn(clippy::float_cmp_const)]
#![warn(clippy::implicit_hasher)]
#![warn(clippy::implicit_saturating_sub)]
#![warn(clippy::imprecise_flops)]
#![warn(clippy::large_types_passed_by_value)]
#![warn(clippy::macro_use_imports)]
#![warn(clippy::manual_ok_or)]
#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::needless_pass_by_value)]
#![warn(clippy::non_ascii_literal)]
#![warn(clippy::semicolon_if_nothing_returned)]
#![warn(clippy::suboptimal_flops)]
#![warn(clippy::todo)]
#![warn(clippy::trivially_copy_pass_by_ref)]
// #![warn(clippy::type_repetition_in_bounds)]
#![warn(clippy::unreadable_literal)]
#![warn(clippy::unseparated_literal_suffix)]
#![warn(clippy::unused_self)]
#![warn(clippy::unnecessary_wraps)]
#![warn(clippy::missing_errors_doc)]
#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![doc(html_root_url = "https://docs.rs/sudoku/0.0.0")]
#![warn(clippy::all)]
#![warn(clippy::exhaustive_enums)]
#![warn(rustdoc::missing_crate_level_docs)]
//#![warn(clippy::missing_docs_in_private_items)]
//#![doc(test(attr(deny(warnings))))]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub mod cell;
pub mod error;
pub mod grid;
pub mod size;

#[cfg(test)]
mod test;

// Improvement list,
// const generics size
// test algo with sudoku found online
// optimisation ?
// improve deduction algo
// GUI ?
// draw contexte
// limit number of draw

// /// Size of a the square
// pub const SQUARE_SIZE: usize = 3;
// /// number of number per rows / collumns / square
// pub const GAME_SIZE: usize = SQUARE_SIZE * SQUARE_SIZE;

mod private {
    /// Private trait to prevent the implementation by other struct outside the crate
    pub trait Sealed {}
}
