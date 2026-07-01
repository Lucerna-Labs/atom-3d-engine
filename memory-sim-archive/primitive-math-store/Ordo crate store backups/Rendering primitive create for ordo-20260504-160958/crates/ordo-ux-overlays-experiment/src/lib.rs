//! Renderer-neutral overlay primitive experiments for Ordo UX.
//!
//! This crate models tooltips, popovers, modals, panels, command palettes, and
//! HUD overlays as data that emits [`ordo_ux_primitives::Primitive`] values. It
//! does not render, create windows, trap focus, or own input routing.

use kurbo::{Point, Rect, Size};
use ordo_ux_primitives::{
    Bounds, CursorHint, Fill, HitRegion, Layer, Primitive, Shape, Stroke, TextAlign, TextRun,
    ThemeTokens,
};
use peniko::Color;

/// High-level overlay surface category.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OverlayKind {
    /// Small contextual text surface.
    Tooltip,
    /// Anchored contextual surface.
    #[default]
    Popover,
    /// Blocking dialog with scrim.
    Modal,
    /// Non-blocking floating panel.
    Panel,
    /// Centered command palette.
    CommandPalette,
    /// Lightweight status surface.
    Hud,
}

/// Preferred placement relative to an anchor.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OverlayPlacement {
    /// Above the anchor.
    Top,
    /// Below the anchor.
    #[default]
    Bottom,
    /// Left of the anchor.
    Left,
    /// Right of the anchor.
    Right,
    /// Centered in the viewport.
    Center,
}

/// Alignment along the cross axis.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OverlayAlign {
    /// Leading edge.
    Start,
    /// Centered.
    #[default]
    Center,
    /// Trailing edge.
    End,
}

/// Visual treatment for an overlay surface.
#[derive(Clone, Debug, PartialEq)]
pub struct OverlayChrome {
    /// Surface fill.
    pub fill: Color,
    /// Border color.
    pub border: Color,
    /// Text color.
    pub text: Color,
    /// Optional scrim color for modal overlays.
    pub scrim: Option<Color>,
    /// Corner radius.
    pub radius: f64,
    /// Border width.
    pub border_width: f64,
    /// Padding inside the overlay.
    pub padding: f64,
}

impl OverlayChrome {
    /// Creates overlay chrome from theme tokens.
    #[must_use]
    pub fn from_theme(theme: &ThemeTokens) -> Self {
        Self {
            fill: theme.surface,
            border: theme.border,
            text: theme.text_primary,
            scrim: Some(Color::new([0.0, 0.0, 0.0, 0.42])),
            radius: theme.radius_md,
            border_width: 1.0,
            padding: theme.spacing_unit * 1.5,
        }
    }

    /// Sets the scrim color.
    #[must_use]
    pub fn with_scrim(mut self, scrim: Option<Color>) -> Self {
        self.scrim = scrim;
        self
    }
}

/// Layout constraints for overlay placement.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OverlayLayout {
    /// Viewport bounds.
    pub viewport: Bounds,
    /// Desired overlay size.
    pub size: Size,
    /// Gap between anchor and overlay.
    pub gap: f64,
    /// Minimum viewport margin.
    pub margin: f64,
}

impl OverlayLayout {
    /// Creates overlay layout constraints.
    #[must_use]
    pub fn new(viewport: Bounds, size: Size) -> Self {
        Self {
            viewport,
            size,
            gap: 8.0,
            margin: 8.0,
        }
    }

    /// Sets anchor gap.
    #[must_use]
    pub fn with_gap(mut self, gap: f64) -> Self {
        self.gap = gap;
        self
    }

    /// Sets viewport margin.
    #[must_use]
    pub fn with_margin(mut self, margin: f64) -> Self {
        self.margin = margin;
        self
    }
}

/// A single overlay definition.
#[derive(Clone, Debug, PartialEq)]
pub struct Overlay {
    /// Stable overlay identifier.
    pub id: String,
    /// Overlay kind.
    pub kind: OverlayKind,
    /// Primary text content.
    pub title: Option<String>,
    /// Optional body text.
    pub body: Option<String>,
    /// Optional anchor bounds.
    pub anchor: Option<Bounds>,
    /// Placement preference.
    pub placement: OverlayPlacement,
    /// Cross-axis alignment.
    pub align: OverlayAlign,
    /// Whether overlay accepts pointer interaction.
    pub interactive: bool,
    /// Whether the overlay should draw a pointer arrow toward its anchor.
    pub arrow: bool,
    /// Visual treatment.
    pub chrome: OverlayChrome,
}

impl Overlay {
    /// Creates an overlay.
    #[must_use]
    pub fn new(id: impl Into<String>, kind: OverlayKind, chrome: OverlayChrome) -> Self {
        Self {
            id: id.into(),
            kind,
            title: None,
            body: None,
            anchor: None,
            placement: OverlayPlacement::default(),
            align: OverlayAlign::default(),
            interactive: true,
            arrow: false,
            chrome,
        }
    }

    /// Creates a tooltip with text.
    #[must_use]
    pub fn tooltip(id: impl Into<String>, text: impl Into<String>, chrome: OverlayChrome) -> Self {
        Self::new(id, OverlayKind::Tooltip, chrome)
            .with_body(text)
            .with_arrow(true)
            .interactive(false)
    }

    /// Creates a modal overlay.
    #[must_use]
    pub fn modal(id: impl Into<String>, title: impl Into<String>, chrome: OverlayChrome) -> Self {
        Self::new(id, OverlayKind::Modal, chrome)
            .with_title(title)
            .with_placement(OverlayPlacement::Center)
    }

    /// Sets title text.
    #[must_use]
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets body text.
    #[must_use]
    pub fn with_body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// Sets anchor bounds.
    #[must_use]
    pub fn anchored_to(mut self, anchor: Bounds) -> Self {
        self.anchor = Some(anchor);
        self
    }

    /// Sets placement.
    #[must_use]
    pub fn with_placement(mut self, placement: OverlayPlacement) -> Self {
        self.placement = placement;
        self
    }

    /// Sets alignment.
    #[must_use]
    pub fn with_align(mut self, align: OverlayAlign) -> Self {
        self.align = align;
        self
    }

    /// Sets whether to draw an anchor arrow.
    #[must_use]
    pub fn with_arrow(mut self, arrow: bool) -> Self {
        self.arrow = arrow;
        self
    }

    /// Sets whether this overlay accepts pointer interaction.
    #[must_use]
    pub fn interactive(mut self, interactive: bool) -> Self {
        self.interactive = interactive;
        self
    }

    /// Computes placed overlay bounds inside the viewport.
    #[must_use]
    pub fn placed_bounds(&self, layout: OverlayLayout) -> Bounds {
        let viewport = layout.viewport.rect;
        let anchor = self.anchor.map(|bounds| bounds.rect);
        let mut origin = match (self.placement, anchor) {
            (OverlayPlacement::Center, _) | (_, None) => Point::new(
                viewport.x0 + (viewport.width() - layout.size.width) / 2.0,
                viewport.y0 + (viewport.height() - layout.size.height) / 2.0,
            ),
            (OverlayPlacement::Top, Some(anchor)) => Point::new(
                cross_axis_x(anchor, layout.size.width, self.align),
                anchor.y0 - layout.gap - layout.size.height,
            ),
            (OverlayPlacement::Bottom, Some(anchor)) => Point::new(
                cross_axis_x(anchor, layout.size.width, self.align),
                anchor.y1 + layout.gap,
            ),
            (OverlayPlacement::Left, Some(anchor)) => Point::new(
                anchor.x0 - layout.gap - layout.size.width,
                cross_axis_y(anchor, layout.size.height, self.align),
            ),
            (OverlayPlacement::Right, Some(anchor)) => Point::new(
                anchor.x1 + layout.gap,
                cross_axis_y(anchor, layout.size.height, self.align),
            ),
        };

        origin.x = origin.x.clamp(
            viewport.x0 + layout.margin,
            viewport.x1 - layout.margin - layout.size.width,
        );
        origin.y = origin.y.clamp(
            viewport.y0 + layout.margin,
            viewport.y1 - layout.margin - layout.size.height,
        );

        Bounds::new(origin, layout.size)
    }

    /// Emits renderer-neutral primitives for this overlay.
    #[must_use]
    pub fn primitives(&self, layout: OverlayLayout) -> Vec<Primitive> {
        let bounds = self.placed_bounds(layout);
        let mut layer = Layer::new().named(format!("overlay:{}", self.id));

        if self.kind == OverlayKind::Modal
            && let Some(scrim) = self.chrome.scrim
        {
            layer.push(Primitive::fill(
                Shape::Rect(layout.viewport.rect),
                Fill::new(scrim),
            ));
        }

        layer.push(
            Primitive::fill(
                Shape::rounded_rect(
                    bounds.rect.x0,
                    bounds.rect.y0,
                    bounds.rect.width(),
                    bounds.rect.height(),
                    self.chrome.radius,
                ),
                Fill::new(self.chrome.fill),
            )
            .with_hit_region_option(self.hit_region(bounds)),
        );
        layer.push(Primitive::stroke(
            Shape::rounded_rect(
                bounds.rect.x0,
                bounds.rect.y0,
                bounds.rect.width(),
                bounds.rect.height(),
                self.chrome.radius,
            ),
            Stroke::new(self.chrome.border, self.chrome.border_width),
        ));

        if self.arrow
            && let Some(anchor) = self.anchor
        {
            layer.push(Primitive::fill(
                self.arrow_shape(bounds, anchor),
                Fill::new(self.chrome.fill),
            ));
        }

        let text_x = bounds.rect.x0 + self.chrome.padding;
        let mut text_y = bounds.rect.y0 + self.chrome.padding + 14.0;

        if let Some(title) = &self.title {
            layer.push(Primitive::text(
                TextRun::new(title.clone(), Point::new(text_x, text_y))
                    .with_font("System", 16.0)
                    .with_fill(Fill::new(self.chrome.text))
                    .with_bounds(Bounds::from_xywh(
                        text_x,
                        bounds.rect.y0 + self.chrome.padding,
                        bounds.rect.width() - self.chrome.padding * 2.0,
                        22.0,
                    )),
            ));
            text_y += 24.0;
        }

        if let Some(body) = &self.body {
            layer.push(Primitive::text(
                TextRun::new(body.clone(), Point::new(text_x, text_y))
                    .with_font("System", 13.0)
                    .with_align(TextAlign::Start)
                    .with_fill(Fill::new(self.chrome.text.with_alpha(0.76)))
                    .with_bounds(Bounds::from_xywh(
                        text_x,
                        text_y - 14.0,
                        bounds.rect.width() - self.chrome.padding * 2.0,
                        bounds.rect.height() - self.chrome.padding * 2.0,
                    )),
            ));
        }

        vec![Primitive::layer(layer)]
    }

    fn hit_region(&self, bounds: Bounds) -> Option<HitRegion> {
        self.interactive.then(|| {
            HitRegion::new(self.id.clone(), bounds)
                .with_shape(Shape::rounded_rect(
                    bounds.rect.x0,
                    bounds.rect.y0,
                    bounds.rect.width(),
                    bounds.rect.height(),
                    self.chrome.radius,
                ))
                .with_cursor(CursorHint::Default)
        })
    }

    fn arrow_shape(&self, bounds: Bounds, anchor: Bounds) -> Shape {
        let overlay = bounds.rect;
        let anchor = anchor.rect;
        let size = 8.0;
        let mut path = kurbo::BezPath::new();

        match self.placement {
            OverlayPlacement::Top => {
                let x = anchor.x0 + anchor.width() / 2.0;
                path.move_to((x - size, overlay.y1));
                path.line_to((x + size, overlay.y1));
                path.line_to((x, overlay.y1 + size));
            }
            OverlayPlacement::Bottom => {
                let x = anchor.x0 + anchor.width() / 2.0;
                path.move_to((x - size, overlay.y0));
                path.line_to((x + size, overlay.y0));
                path.line_to((x, overlay.y0 - size));
            }
            OverlayPlacement::Left => {
                let y = anchor.y0 + anchor.height() / 2.0;
                path.move_to((overlay.x1, y - size));
                path.line_to((overlay.x1, y + size));
                path.line_to((overlay.x1 + size, y));
            }
            OverlayPlacement::Right => {
                let y = anchor.y0 + anchor.height() / 2.0;
                path.move_to((overlay.x0, y - size));
                path.line_to((overlay.x0, y + size));
                path.line_to((overlay.x0 - size, y));
            }
            OverlayPlacement::Center => {
                return Shape::Rect(Rect::ZERO);
            }
        }

        path.close_path();
        Shape::path(path)
    }
}

/// Overlay stack ordering and primitive emission.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OverlayStack {
    /// Overlays in z-order, low to high.
    pub overlays: Vec<Overlay>,
}

impl OverlayStack {
    /// Creates an empty stack.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Pushes an overlay above existing overlays.
    pub fn push(&mut self, overlay: Overlay) {
        self.overlays.push(overlay);
    }

    /// Removes an overlay by id.
    pub fn remove(&mut self, id: &str) -> Option<Overlay> {
        let index = self.overlays.iter().position(|overlay| overlay.id == id)?;
        Some(self.overlays.remove(index))
    }

    /// Returns the top-most overlay.
    #[must_use]
    pub fn top(&self) -> Option<&Overlay> {
        self.overlays.last()
    }

    /// Emits primitives for every overlay in z-order.
    #[must_use]
    pub fn primitives(&self, layout: OverlayLayout) -> Vec<Primitive> {
        self.overlays
            .iter()
            .flat_map(|overlay| overlay.primitives(layout))
            .collect()
    }
}

trait PrimitiveOptionExt {
    fn with_hit_region_option(self, hit_region: Option<HitRegion>) -> Self;
}

impl PrimitiveOptionExt for Primitive {
    fn with_hit_region_option(self, hit_region: Option<HitRegion>) -> Self {
        match hit_region {
            Some(hit_region) => self.with_hit_region(hit_region),
            None => self,
        }
    }
}

fn cross_axis_x(anchor: Rect, width: f64, align: OverlayAlign) -> f64 {
    match align {
        OverlayAlign::Start => anchor.x0,
        OverlayAlign::Center => anchor.x0 + (anchor.width() - width) / 2.0,
        OverlayAlign::End => anchor.x1 - width,
    }
}

fn cross_axis_y(anchor: Rect, height: f64, align: OverlayAlign) -> f64 {
    match align {
        OverlayAlign::Start => anchor.y0,
        OverlayAlign::Center => anchor.y0 + (anchor.height() - height) / 2.0,
        OverlayAlign::End => anchor.y1 - height,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chrome() -> OverlayChrome {
        OverlayChrome::from_theme(&ThemeTokens::default())
    }

    fn viewport() -> Bounds {
        Bounds::from_xywh(0.0, 0.0, 400.0, 300.0)
    }

    #[test]
    fn popover_places_below_anchor() {
        let overlay = Overlay::new("menu", OverlayKind::Popover, chrome())
            .anchored_to(Bounds::from_xywh(100.0, 50.0, 40.0, 20.0));
        let bounds = overlay.placed_bounds(OverlayLayout::new(viewport(), Size::new(160.0, 80.0)));

        assert_eq!(bounds.rect.y0, 78.0);
    }

    #[test]
    fn placement_clamps_to_viewport_margin() {
        let overlay = Overlay::new("edge", OverlayKind::Popover, chrome())
            .anchored_to(Bounds::from_xywh(380.0, 280.0, 20.0, 20.0))
            .with_placement(OverlayPlacement::Right);
        let bounds = overlay.placed_bounds(OverlayLayout::new(viewport(), Size::new(120.0, 80.0)));

        assert!(bounds.rect.x1 <= 392.0);
        assert!(bounds.rect.y1 <= 292.0);
    }

    #[test]
    fn tooltip_emits_layer_primitive() {
        let tooltip = Overlay::tooltip("tip", "Hello", chrome())
            .anchored_to(Bounds::from_xywh(100.0, 50.0, 40.0, 20.0));
        let primitives = tooltip.primitives(OverlayLayout::new(viewport(), Size::new(120.0, 48.0)));

        assert_eq!(primitives.len(), 1);
        assert!(matches!(primitives.first(), Some(Primitive::Layer(_))));
    }

    #[test]
    fn modal_emits_scrim_inside_layer() {
        let modal = Overlay::modal("confirm", "Confirm", chrome());
        let primitives = modal.primitives(OverlayLayout::new(viewport(), Size::new(220.0, 140.0)));

        match primitives.first() {
            Some(Primitive::Layer(layer)) => assert!(layer.primitives.len() >= 4),
            _ => panic!("expected layer primitive"),
        }
    }

    #[test]
    fn stack_orders_and_removes_overlays() {
        let mut stack = OverlayStack::new();
        stack.push(Overlay::new("first", OverlayKind::Panel, chrome()));
        stack.push(Overlay::new("second", OverlayKind::Hud, chrome()));

        assert_eq!(
            stack.top().map(|overlay| overlay.id.as_str()),
            Some("second")
        );
        assert_eq!(
            stack.remove("first").map(|overlay| overlay.id),
            Some("first".to_string())
        );
        assert_eq!(stack.overlays.len(), 1);
    }

    #[test]
    fn stack_emits_one_layer_per_overlay() {
        let mut stack = OverlayStack::new();
        stack.push(Overlay::new("panel", OverlayKind::Panel, chrome()));
        stack.push(Overlay::new("hud", OverlayKind::Hud, chrome()));

        let primitives = stack.primitives(OverlayLayout::new(viewport(), Size::new(120.0, 80.0)));

        assert_eq!(primitives.len(), 2);
    }
}
