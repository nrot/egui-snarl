use std::fmt;

use egui::{emath::Rot2, vec2, Pos2, Rect, Stroke, Ui, Vec2};

use super::SnarlStyle;

/// Viewport is a rectangle in graph space that is visible on screen.
pub struct Viewport {
    /// Screen-space rectangle.
    pub rect: Rect,

    /// Scale of the viewport.
    pub scale: f32,

    /// Offset of the viewport.
    pub offset: Vec2,
}

impl Viewport {
    /// Converts screen-space position to graph-space position.
    #[inline(always)]
    pub fn screen_pos_to_graph(&self, pos: Pos2) -> Pos2 {
        (pos + self.offset - self.rect.center().to_vec2()) / self.scale
    }

    /// Converts graph-space position to screen-space position.
    #[inline(always)]
    pub fn graph_pos_to_screen(&self, pos: Pos2) -> Pos2 {
        pos * self.scale - self.offset + self.rect.center().to_vec2()
    }

    /// Converts screen-space vector to graph-space vector.
    #[inline(always)]
    pub fn graph_vec_to_screen(&self, size: Vec2) -> Vec2 {
        size * self.scale
    }

    /// Converts graph-space vector to screen-space vector.
    #[inline(always)]
    pub fn screen_vec_to_graph(&self, size: Vec2) -> Vec2 {
        size / self.scale
    }

    /// Converts screen-space size to graph-space size.
    #[inline(always)]
    pub fn graph_size_to_screen(&self, size: f32) -> f32 {
        size * self.scale
    }

    /// Converts graph-space size to screen-space size.
    #[inline(always)]
    pub fn screen_size_to_graph(&self, size: f32) -> f32 {
        size / self.scale
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
///Grid background pattern.
///Use `SnarlStyle::background_pattern_stroke` for change stroke options
pub struct Grid {
    /// Spacing between grid lines.
    pub spacing: Vec2,

    /// Angle of the grid.
    pub angle: f32,
}

const DEFAULT_GRID_SPACING: Vec2 = vec2(5.0, 5.0);
macro_rules! default_grid_spacing {
    () => {
        stringify!(vec2(5.0, 5.0))
    };
}

const DEFAULT_GRID_ANGLE: f32 = 1.0;
macro_rules! default_grid_angle {
    () => {
        stringify!(1.0)
    };
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            spacing: DEFAULT_GRID_SPACING,
            angle: DEFAULT_GRID_ANGLE,
        }
    }
}

impl Grid {
    /// Create new grid with given spacing and angle.
    pub const fn new(spacing: Vec2, angle: f32) -> Self {
        Self { spacing, angle }
    }

    fn draw(&self, style: &SnarlStyle, viewport: &Viewport, ui: &mut Ui) {
        let bg_stroke = style
            .background_pattern_stroke
            .unwrap_or_else(|| ui.visuals().widgets.noninteractive.bg_stroke);

        let stroke = Stroke::new(
            bg_stroke.width * viewport.scale.max(1.0),
            bg_stroke.color.gamma_multiply(viewport.scale.min(1.0)),
        );

        let spacing = ui.spacing().icon_width * self.spacing;

        let rot = Rot2::from_angle(self.angle);
        let rot_inv = rot.inverse();

        let graph_viewport = Rect::from_min_max(
            viewport.screen_pos_to_graph(viewport.rect.min),
            viewport.screen_pos_to_graph(viewport.rect.max),
        );

        let pattern_bounds = graph_viewport.rotate_bb(rot_inv);

        let min_x = (pattern_bounds.min.x / spacing.x).ceil();
        let max_x = (pattern_bounds.max.x / spacing.x).floor();

        for x in 0..=(max_x - min_x) as i64 {
            #[allow(clippy::cast_possible_truncation)]
            let x = (x as f32 + min_x) * spacing.x;

            let top = (rot * vec2(x, pattern_bounds.min.y)).to_pos2();
            let bottom = (rot * vec2(x, pattern_bounds.max.y)).to_pos2();

            let top = viewport.graph_pos_to_screen(top);
            let bottom = viewport.graph_pos_to_screen(bottom);

            ui.painter().line_segment([top, bottom], stroke);
        }

        let min_y = (pattern_bounds.min.y / spacing.y).ceil();
        let max_y = (pattern_bounds.max.y / spacing.y).floor();

        for y in 0..=(max_y - min_y) as i64 {
            #[allow(clippy::cast_possible_truncation)]
            let y = (y as f32 + min_y) * spacing.y;

            let top = (rot * vec2(pattern_bounds.min.x, y)).to_pos2();
            let bottom = (rot * vec2(pattern_bounds.max.x, y)).to_pos2();

            let top = viewport.graph_pos_to_screen(top);
            let bottom = viewport.graph_pos_to_screen(bottom);

            ui.painter().line_segment([top, bottom], stroke);
        }
    }
}

mod with_missing_docs {
    #![allow(missing_docs)]
    use super::*;

    tiny_fn::tiny_fn! {
        /// Custom background pattern function with signature
        /// `Fn(style: &SnarlStyle, viewport: &Viewport, ui: &mut Ui)`
        pub struct CustomBackground = Fn(style: &SnarlStyle, viewport: &Viewport, ui: &mut Ui);
    }
}
pub use with_missing_docs::CustomBackground;

/// Background pattern show beneath nodes and wires.
pub enum BackgroundPattern {
    /// No pattern.
    NoPattern,

    /// Linear grid.
    Grid(Grid),

    /// Custom pattern.
    /// Contains function with signature
    /// `Fn(style: &SnarlStyle, viewport: &Viewport, ui: &mut Ui)`
    Custom(CustomBackground<'static>),
}

impl PartialEq for BackgroundPattern {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (BackgroundPattern::Grid(l), BackgroundPattern::Grid(r)) => *l == *r,
            _ => false,
        }
    }
}

impl fmt::Debug for BackgroundPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackgroundPattern::Grid(grid) => f
                .debug_tuple("BackgroundPattern::Grid")
                .field(grid)
                .finish(),
            BackgroundPattern::Custom(_) => f.write_str("BackgroundPattern::Custom"),
            BackgroundPattern::NoPattern => f.write_str("BackgroundPattern::NoPattern"),
        }
    }
}

impl Default for BackgroundPattern {
    fn default() -> Self {
        Self::Grid(Default::default())
    }
}

impl BackgroundPattern {
    /// Create new background pattern with default values.
    ///
    /// Default patter is `Grid` with spacing - `
    #[doc = default_grid_spacing!()]
    /// ` and angle - `
    #[doc = default_grid_angle!()]
    /// ` radian.
    pub const fn new() -> Self {
        Self::Grid(Grid::new(DEFAULT_GRID_SPACING, DEFAULT_GRID_ANGLE))
    }

    /// Create new grid background pattern with given spacing and angle.
    pub const fn grid(spacing: Vec2, angle: f32) -> Self {
        Self::Grid(Grid::new(spacing, angle))
    }

    /// Create new custom background pattern.
    pub fn custom<F>(f: F) -> Self
    where
        F: Fn(&SnarlStyle, &Viewport, &mut Ui) + 'static,
    {
        Self::Custom(CustomBackground::new(f))
    }

    /// Draws background pattern.
    pub(super) fn draw(&self, style: &SnarlStyle, viewport: &Viewport, ui: &mut Ui) {
        match self {
            BackgroundPattern::Grid(g) => g.draw(style, viewport, ui),
            BackgroundPattern::Custom(c) => c.call(style, viewport, ui),
            BackgroundPattern::NoPattern => {}
        }
    }
}
