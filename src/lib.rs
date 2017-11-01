// Aldaron's Format Interface / Aldaron's Document Format (docf)
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/lib.rs

//! Aldaron's Format Interface / docf is a library developed by Plop Grizzly for
//! reading and writing docf (Aldaron's Document Format) files.

/// Text alignment
#[repr(u8)] #[derive(PartialEq, Copy, Clone)]
pub enum Align {
	/// Left aligned
	Left = 0u8,
	/// Horizontally centered
	Centered = 1u8,
	/// Right aligned
	Right = 2u8,
	/// Justified
	Justified = 3u8,
}

/// Text emphasis
#[repr(u8)] #[derive(PartialEq, Copy, Clone)]
pub enum Emphasis {
	/// Regular
	None = 0b_0000_0000_u8,
	/// Strikethrough
	StrikeOut = 0b_0000_0001_u8,
	/// Overline
	Overline = 0b_0000_0010_u8,
	/// Underline Continuous
	Underline = 0b_0000_0100_u8,
	/// Underline Discontinuous
	UnderlineDC = 0b_0000_1000_u8,
	/// Double Underline
	UnderlineX2 = 0b_0001_0000_u8,
	/// Invert Colors
	InvertColor = 0b_0010_0000_u8,
	/// Bold
	Bold = 0b_0100_0000_u8,
	/// Italic
	Italic = 0b_1000_0000_u8,
}

/// Text color
#[repr(u8)] #[derive(PartialEq, Copy, Clone)]
pub enum FontColor {
	/// Black on light background, or white on dark background
	Default,
	/// RGBA 32 bits
	RgbaInt(u8, u8, u8, u8),
	/// RGBA Floating Point
	RgbaFloat(f32, f32, f32, f32),
}
