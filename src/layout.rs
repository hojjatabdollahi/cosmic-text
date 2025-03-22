// SPDX-License-Identifier: MIT OR Apache-2.0

use core::fmt::Display;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use crate::{math, CacheKey, CacheKeyFlags, Color};

/// A laid out glyph
#[derive(Clone, Debug)]
pub struct LayoutGlyph {
    /// Start index of cluster in original line
    pub start: usize,
    /// End index of cluster in original line
    pub end: usize,
    /// Font size of the glyph
    pub font_size: f32,
    /// Line height of the glyph, will override buffer setting
    pub line_height_opt: Option<f32>,
    /// Font id of the glyph
    pub font_id: fontdb::ID,
    /// Font id of the glyph
    pub glyph_id: u16,
    /// X offset of hitbox
    pub x: f32,
    /// Y offset of hitbox
    pub y: f32,
    /// Width of hitbox
    pub w: f32,
    /// Unicode `BiDi` embedding level, character is left-to-right if `level` is divisible by 2
    pub level: unicode_bidi::Level,
    /// X offset in line
    ///
    /// If you are dealing with physical coordinates, use [`Self::physical`] to obtain a
    /// [`PhysicalGlyph`] for rendering.
    ///
    /// This offset is useful when you are dealing with logical units and you do not care or
    /// cannot guarantee pixel grid alignment. For instance, when you want to use the glyphs
    /// for vectorial text, apply linear transformations to the layout, etc.
    pub x_offset: f32,
    /// Y offset in line
    ///
    /// If you are dealing with physical coordinates, use [`Self::physical`] to obtain a
    /// [`PhysicalGlyph`] for rendering.
    ///
    /// This offset is useful when you are dealing with logical units and you do not care or
    /// cannot guarantee pixel grid alignment. For instance, when you want to use the glyphs
    /// for vectorial text, apply linear transformations to the layout, etc.
    pub y_offset: f32,
    /// Optional color override
    pub color_opt: Option<Color>,
    /// Metadata from `Attrs`
    pub metadata: usize,
    /// [`CacheKeyFlags`]
    pub cache_key_flags: CacheKeyFlags,
}

#[derive(Clone, Debug)]
pub struct PhysicalGlyph {
    /// Cache key, see [`CacheKey`]
    pub cache_key: CacheKey,
    /// Integer component of X offset in line
    pub x: i32,
    /// Integer component of Y offset in line
    pub y: i32,
}

impl LayoutGlyph {
    pub fn physical(&self, offset: (f32, f32), scale: f32) -> PhysicalGlyph {
        let x_offset = self.font_size * self.x_offset;
        let y_offset = self.font_size * self.y_offset;

        let (cache_key, x, y) = CacheKey::new(
            self.font_id,
            self.glyph_id,
            self.font_size * scale,
            (
                (self.x + x_offset) * scale + offset.0,
                math::truncf((self.y - y_offset) * scale + offset.1), // Hinting in Y axis
            ),
            self.cache_key_flags,
        );

        PhysicalGlyph { cache_key, x, y }
    }
}

/// A line of laid out glyphs
#[derive(Clone, Debug)]
pub struct LayoutLine {
    /// Width of the line
    pub w: f32,
    /// Maximum ascent of the glyphs in line
    pub max_ascent: f32,
    /// Maximum descent of the glyphs in line
    pub max_descent: f32,
    /// Maximum line height of any spans in line
    pub line_height_opt: Option<f32>,
    /// Glyphs in line
    pub glyphs: Vec<LayoutGlyph>,
}

/// Wrapping mode
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Wrap {
    /// No wrapping
    None,
    /// Wraps at a glyph level
    Glyph,
    /// Wraps at the word level
    Word,
    /// Wraps at the word level, or fallback to glyph level if a word can't fit on a line by itself
    WordOrGlyph,
}

impl Display for Wrap {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::None => write!(f, "No Wrap"),
            Self::Word => write!(f, "Word Wrap"),
            Self::WordOrGlyph => write!(f, "Word Wrap or Character"),
            Self::Glyph => write!(f, "Character"),
        }
    }
}

/// The maximum allowed number of lines before ellipsizing
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum HeightLimit {
    /// Default is a single line
    Default,
    /// Number of maximum lines allowed, `Line(0)`, or `Line(1)`, would be the same as `Default`,
    /// which is a single line. The ellipsis will show at the end of the last line.
    Lines(u8),
    /// The amount of text is limited to the current buffer height. Depending on the font size,
    /// fewer or more lines will be displayed.
    Height,
}

/// Ellipsize mode, `Start` and `End` respect the paragraph text direction
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Ellipsize {
    /// No Ellipsize
    None,
    /// Ellipsis at the start, will create a single line with ellipsis at the start
    Start,
    /// Ellipsis in the middle, will create a single line with ellipsis in the middle
    Middle,
    /// Ellipsis at the end, can create one, or more lines depending on the `HeighLimit`
    End(HeightLimit),
}

/// Align or justify
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Align {
    Left,
    Right,
    Center,
    Justified,
    End,
}

impl Display for Align {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Left => write!(f, "Left"),
            Self::Right => write!(f, "Right"),
            Self::Center => write!(f, "Center"),
            Self::Justified => write!(f, "Justified"),
            Self::End => write!(f, "End"),
        }
    }
}
