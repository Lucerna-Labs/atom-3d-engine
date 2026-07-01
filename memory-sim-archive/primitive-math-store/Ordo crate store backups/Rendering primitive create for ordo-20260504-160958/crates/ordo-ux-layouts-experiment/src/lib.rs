//! Renderer-neutral UXI layout composition experiments for Ordo UX.
//!
//! This crate computes layout regions for app shells, dashboards, editors,
//! side carts, bars, rails, panels, and floating zones. It emits bounds and
//! optional debug primitives, but it does not render, mount components, or own
//! window state.

use ordo_ux_primitives::{Bounds, Fill, HitRegion, Primitive, Shape, Stroke, ThemeTokens};
use peniko::Color;

/// A named area in a UXI layout.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LayoutSlot {
    /// Full viewport root.
    Root,
    /// Top application bar.
    TopBar,
    /// Bottom application bar.
    BottomBar,
    /// Left navigation rail or sidebar.
    LeftRail,
    /// Right rail or inspector strip.
    RightRail,
    /// Left side cart/panel.
    LeftCart,
    /// Right side cart/panel.
    RightCart,
    /// Main content area.
    Content,
    /// Header inside content.
    ContentHeader,
    /// Footer inside content.
    ContentFooter,
    /// Floating layer region.
    Floating,
    /// Overlay layer region.
    Overlay,
}

impl LayoutSlot {
    /// All built-in slots.
    pub const ALL: [Self; 12] = [
        Self::Root,
        Self::TopBar,
        Self::BottomBar,
        Self::LeftRail,
        Self::RightRail,
        Self::LeftCart,
        Self::RightCart,
        Self::Content,
        Self::ContentHeader,
        Self::ContentFooter,
        Self::Floating,
        Self::Overlay,
    ];
}

/// Dimension rules for a layout region.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LayoutSize {
    /// Fixed logical size.
    Fixed(f64),
    /// Fraction of available space from `0.0` to `1.0`.
    Fraction(f64),
    /// Clamp a preferred size between min and max.
    Clamp {
        /// Minimum logical size.
        min: f64,
        /// Preferred logical size.
        preferred: f64,
        /// Maximum logical size.
        max: f64,
    },
}

impl LayoutSize {
    fn resolve(self, available: f64) -> f64 {
        match self {
            Self::Fixed(size) => size,
            Self::Fraction(fraction) => available * fraction.clamp(0.0, 1.0),
            Self::Clamp {
                min,
                preferred,
                max,
            } => preferred.clamp(min, max).min(available),
        }
        .max(0.0)
    }
}

/// Visibility and interaction mode for optional panels.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RegionMode {
    /// Region is not present.
    Hidden,
    /// Region consumes layout space.
    #[default]
    Docked,
    /// Region floats above content.
    Floating,
}

/// Configuration for a single layout region.
#[derive(Clone, Debug, PartialEq)]
pub struct LayoutRegion {
    /// Region slot.
    pub slot: LayoutSlot,
    /// Developer-facing id.
    pub id: String,
    /// Size along the slot's primary axis.
    pub size: LayoutSize,
    /// Visibility/interaction mode.
    pub mode: RegionMode,
    /// Minimum spacing between this region and adjacent content.
    pub gap: f64,
    /// Whether this region should expose a debug hit region.
    pub interactive: bool,
}

impl LayoutRegion {
    /// Creates a region.
    #[must_use]
    pub fn new(slot: LayoutSlot, id: impl Into<String>, size: LayoutSize) -> Self {
        Self {
            slot,
            id: id.into(),
            size,
            mode: RegionMode::Docked,
            gap: 0.0,
            interactive: false,
        }
    }

    /// Sets the region mode.
    #[must_use]
    pub fn with_mode(mut self, mode: RegionMode) -> Self {
        self.mode = mode;
        self
    }

    /// Sets adjacent gap.
    #[must_use]
    pub fn with_gap(mut self, gap: f64) -> Self {
        self.gap = gap;
        self
    }

    /// Marks this region interactive for debug/editing.
    #[must_use]
    pub fn interactive(mut self) -> Self {
        self.interactive = true;
        self
    }
}

/// Responsive layout breakpoint.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LayoutBreakpoint {
    /// Compact phone-like viewport.
    Compact,
    /// Medium tablet/small desktop viewport.
    Medium,
    /// Expanded desktop viewport.
    Expanded,
}

impl LayoutBreakpoint {
    /// Resolves a breakpoint from viewport width.
    #[must_use]
    pub fn from_width(width: f64) -> Self {
        if width < 640.0 {
            Self::Compact
        } else if width < 1024.0 {
            Self::Medium
        } else {
            Self::Expanded
        }
    }
}

/// Outer spacing applied to the whole layout.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LayoutInsets {
    /// Left inset.
    pub left: f64,
    /// Top inset.
    pub top: f64,
    /// Right inset.
    pub right: f64,
    /// Bottom inset.
    pub bottom: f64,
}

impl LayoutInsets {
    /// No insets.
    pub const ZERO: Self = Self {
        left: 0.0,
        top: 0.0,
        right: 0.0,
        bottom: 0.0,
    };

    /// Creates equal insets.
    #[must_use]
    pub fn all(value: f64) -> Self {
        Self {
            left: value,
            top: value,
            right: value,
            bottom: value,
        }
    }
}

impl Default for LayoutInsets {
    fn default() -> Self {
        Self::ZERO
    }
}

/// A developer-composable UXI layout recipe.
#[derive(Clone, Debug, PartialEq)]
pub struct UxiLayout {
    /// Layout name.
    pub name: String,
    /// Outer viewport insets.
    pub insets: LayoutInsets,
    /// Content inner padding.
    pub content_padding: f64,
    /// Optional regions.
    pub regions: Vec<LayoutRegion>,
}

impl UxiLayout {
    /// Creates an empty layout.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            insets: LayoutInsets::ZERO,
            content_padding: 0.0,
            regions: Vec::new(),
        }
    }

    /// Creates a common application shell.
    #[must_use]
    pub fn app_shell() -> Self {
        Self::new("app-shell")
            .with_top_bar(56.0)
            .with_bottom_bar(0.0)
            .with_left_rail(72.0)
            .with_content_padding(16.0)
    }

    /// Creates a dashboard/editor layout with an inspector side cart.
    #[must_use]
    pub fn dashboard() -> Self {
        Self::new("dashboard")
            .with_top_bar(56.0)
            .with_left_rail(72.0)
            .with_right_cart(320.0)
            .with_content_header(48.0)
            .with_content_padding(16.0)
    }

    /// Sets outer insets.
    #[must_use]
    pub fn with_insets(mut self, insets: LayoutInsets) -> Self {
        self.insets = insets;
        self
    }

    /// Sets content padding.
    #[must_use]
    pub fn with_content_padding(mut self, padding: f64) -> Self {
        self.content_padding = padding;
        self
    }

    /// Adds or replaces a region.
    #[must_use]
    pub fn with_region(mut self, region: LayoutRegion) -> Self {
        self.set_region(region);
        self
    }

    /// Adds a top bar.
    #[must_use]
    pub fn with_top_bar(self, height: f64) -> Self {
        self.with_region(LayoutRegion::new(
            LayoutSlot::TopBar,
            "top-bar",
            LayoutSize::Fixed(height),
        ))
    }

    /// Adds a bottom bar.
    #[must_use]
    pub fn with_bottom_bar(self, height: f64) -> Self {
        self.with_region(LayoutRegion::new(
            LayoutSlot::BottomBar,
            "bottom-bar",
            LayoutSize::Fixed(height),
        ))
    }

    /// Adds a left rail.
    #[must_use]
    pub fn with_left_rail(self, width: f64) -> Self {
        self.with_region(LayoutRegion::new(
            LayoutSlot::LeftRail,
            "left-rail",
            LayoutSize::Fixed(width),
        ))
    }

    /// Adds a right rail.
    #[must_use]
    pub fn with_right_rail(self, width: f64) -> Self {
        self.with_region(LayoutRegion::new(
            LayoutSlot::RightRail,
            "right-rail",
            LayoutSize::Fixed(width),
        ))
    }

    /// Adds a left side cart.
    #[must_use]
    pub fn with_left_cart(self, width: f64) -> Self {
        self.with_region(LayoutRegion::new(
            LayoutSlot::LeftCart,
            "left-cart",
            LayoutSize::Fixed(width),
        ))
    }

    /// Adds a right side cart.
    #[must_use]
    pub fn with_right_cart(self, width: f64) -> Self {
        self.with_region(LayoutRegion::new(
            LayoutSlot::RightCart,
            "right-cart",
            LayoutSize::Fixed(width),
        ))
    }

    /// Adds a header inside content.
    #[must_use]
    pub fn with_content_header(self, height: f64) -> Self {
        self.with_region(LayoutRegion::new(
            LayoutSlot::ContentHeader,
            "content-header",
            LayoutSize::Fixed(height),
        ))
    }

    /// Adds a footer inside content.
    #[must_use]
    pub fn with_content_footer(self, height: f64) -> Self {
        self.with_region(LayoutRegion::new(
            LayoutSlot::ContentFooter,
            "content-footer",
            LayoutSize::Fixed(height),
        ))
    }

    /// Inserts or replaces a region.
    pub fn set_region(&mut self, region: LayoutRegion) {
        if let Some(existing) = self
            .regions
            .iter_mut()
            .find(|existing| existing.slot == region.slot)
        {
            *existing = region;
        } else {
            self.regions.push(region);
        }
    }

    /// Removes a region by slot.
    pub fn remove_region(&mut self, slot: LayoutSlot) -> Option<LayoutRegion> {
        let index = self.regions.iter().position(|region| region.slot == slot)?;
        Some(self.regions.remove(index))
    }

    /// Resolves this layout against a viewport.
    #[must_use]
    pub fn resolve(&self, viewport: Bounds) -> ResolvedLayout {
        LayoutSolver::new(self, viewport).solve()
    }
}

/// A region with concrete bounds.
#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedRegion {
    /// Region slot.
    pub slot: LayoutSlot,
    /// Developer-facing id.
    pub id: String,
    /// Concrete bounds.
    pub bounds: Bounds,
    /// Region mode.
    pub mode: RegionMode,
    /// Whether this region exposes debug interaction.
    pub interactive: bool,
}

impl ResolvedRegion {
    /// Returns a hit region for dev tooling.
    #[must_use]
    pub fn hit_region(&self) -> HitRegion {
        HitRegion::new(self.id.clone(), self.bounds).with_shape(Shape::Rect(self.bounds.rect))
    }
}

/// Concrete layout output.
#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedLayout {
    /// Layout name.
    pub name: String,
    /// Breakpoint selected from viewport width.
    pub breakpoint: LayoutBreakpoint,
    /// Viewport bounds.
    pub viewport: Bounds,
    /// Resolved regions.
    pub regions: Vec<ResolvedRegion>,
}

impl ResolvedLayout {
    /// Returns a resolved region by slot.
    #[must_use]
    pub fn region(&self, slot: LayoutSlot) -> Option<&ResolvedRegion> {
        self.regions.iter().find(|region| region.slot == slot)
    }

    /// Emits debug primitives for layout visualization.
    #[must_use]
    pub fn debug_primitives(&self, theme: &ThemeTokens) -> Vec<Primitive> {
        self.regions
            .iter()
            .flat_map(|region| {
                let color = slot_color(region.slot, theme);
                let fill = Primitive::fill(
                    Shape::Rect(region.bounds.rect),
                    Fill::new(color.with_alpha(0.10)),
                )
                .with_hit_region_option(region.interactive.then(|| region.hit_region()));
                let stroke = Primitive::stroke(
                    Shape::Rect(region.bounds.rect),
                    Stroke::new(color.with_alpha(0.85), 1.0),
                );
                [fill, stroke]
            })
            .collect()
    }
}

struct LayoutSolver<'a> {
    layout: &'a UxiLayout,
    viewport: Bounds,
    regions: Vec<ResolvedRegion>,
    remaining: Bounds,
}

impl<'a> LayoutSolver<'a> {
    fn new(layout: &'a UxiLayout, viewport: Bounds) -> Self {
        let rect = viewport.rect;
        let insets = layout.insets;
        let remaining = Bounds::from_xywh(
            rect.x0 + insets.left,
            rect.y0 + insets.top,
            (rect.width() - insets.left - insets.right).max(0.0),
            (rect.height() - insets.top - insets.bottom).max(0.0),
        );

        Self {
            layout,
            viewport,
            regions: vec![ResolvedRegion {
                slot: LayoutSlot::Root,
                id: "root".to_string(),
                bounds: viewport,
                mode: RegionMode::Docked,
                interactive: false,
            }],
            remaining,
        }
    }

    fn solve(mut self) -> ResolvedLayout {
        self.take_vertical(LayoutSlot::TopBar, true);
        self.take_vertical(LayoutSlot::BottomBar, false);
        self.take_horizontal(LayoutSlot::LeftRail, true);
        self.take_horizontal(LayoutSlot::RightRail, false);
        self.take_horizontal(LayoutSlot::LeftCart, true);
        self.take_horizontal(LayoutSlot::RightCart, false);

        let content_shell = self.remaining;
        let mut content = inset_bounds(content_shell, self.layout.content_padding);

        if let Some(header) = self.region(LayoutSlot::ContentHeader).cloned()
            && header.mode != RegionMode::Hidden
        {
            let height = header.size.resolve(content.rect.height());
            let bounds = Bounds::from_xywh(
                content.rect.x0,
                content.rect.y0,
                content.rect.width(),
                height,
            );
            self.push_region(&header, bounds);
            content = Bounds::from_xywh(
                content.rect.x0,
                content.rect.y0 + height + header.gap,
                content.rect.width(),
                (content.rect.height() - height - header.gap).max(0.0),
            );
        }

        if let Some(footer) = self.region(LayoutSlot::ContentFooter).cloned()
            && footer.mode != RegionMode::Hidden
        {
            let height = footer.size.resolve(content.rect.height());
            let bounds = Bounds::from_xywh(
                content.rect.x0,
                content.rect.y1 - height,
                content.rect.width(),
                height,
            );
            self.push_region(&footer, bounds);
            content = Bounds::from_xywh(
                content.rect.x0,
                content.rect.y0,
                content.rect.width(),
                (content.rect.height() - height - footer.gap).max(0.0),
            );
        }

        self.regions.push(ResolvedRegion {
            slot: LayoutSlot::Content,
            id: "content".to_string(),
            bounds: content,
            mode: RegionMode::Docked,
            interactive: false,
        });
        self.regions.push(ResolvedRegion {
            slot: LayoutSlot::Floating,
            id: "floating".to_string(),
            bounds: self.viewport,
            mode: RegionMode::Floating,
            interactive: false,
        });
        self.regions.push(ResolvedRegion {
            slot: LayoutSlot::Overlay,
            id: "overlay".to_string(),
            bounds: self.viewport,
            mode: RegionMode::Floating,
            interactive: false,
        });

        ResolvedLayout {
            name: self.layout.name.clone(),
            breakpoint: LayoutBreakpoint::from_width(self.viewport.rect.width()),
            viewport: self.viewport,
            regions: self.regions,
        }
    }

    fn region(&self, slot: LayoutSlot) -> Option<&LayoutRegion> {
        self.layout
            .regions
            .iter()
            .find(|region| region.slot == slot)
    }

    fn take_vertical(&mut self, slot: LayoutSlot, from_start: bool) {
        let Some(region) = self.region(slot).cloned() else {
            return;
        };
        if region.mode == RegionMode::Hidden {
            return;
        }

        let height = region.size.resolve(self.remaining.rect.height());
        let bounds = if from_start {
            let bounds = Bounds::from_xywh(
                self.remaining.rect.x0,
                self.remaining.rect.y0,
                self.remaining.rect.width(),
                height,
            );
            self.remaining = Bounds::from_xywh(
                self.remaining.rect.x0,
                self.remaining.rect.y0 + height + region.gap,
                self.remaining.rect.width(),
                (self.remaining.rect.height() - height - region.gap).max(0.0),
            );
            bounds
        } else {
            let bounds = Bounds::from_xywh(
                self.remaining.rect.x0,
                self.remaining.rect.y1 - height,
                self.remaining.rect.width(),
                height,
            );
            self.remaining = Bounds::from_xywh(
                self.remaining.rect.x0,
                self.remaining.rect.y0,
                self.remaining.rect.width(),
                (self.remaining.rect.height() - height - region.gap).max(0.0),
            );
            bounds
        };

        self.push_region(&region, bounds);
    }

    fn take_horizontal(&mut self, slot: LayoutSlot, from_start: bool) {
        let Some(region) = self.region(slot).cloned() else {
            return;
        };
        if region.mode == RegionMode::Hidden {
            return;
        }

        let width = region.size.resolve(self.remaining.rect.width());
        let bounds = if region.mode == RegionMode::Floating {
            floating_side_bounds(self.viewport, width, from_start)
        } else if from_start {
            let bounds = Bounds::from_xywh(
                self.remaining.rect.x0,
                self.remaining.rect.y0,
                width,
                self.remaining.rect.height(),
            );
            self.remaining = Bounds::from_xywh(
                self.remaining.rect.x0 + width + region.gap,
                self.remaining.rect.y0,
                (self.remaining.rect.width() - width - region.gap).max(0.0),
                self.remaining.rect.height(),
            );
            bounds
        } else {
            let bounds = Bounds::from_xywh(
                self.remaining.rect.x1 - width,
                self.remaining.rect.y0,
                width,
                self.remaining.rect.height(),
            );
            self.remaining = Bounds::from_xywh(
                self.remaining.rect.x0,
                self.remaining.rect.y0,
                (self.remaining.rect.width() - width - region.gap).max(0.0),
                self.remaining.rect.height(),
            );
            bounds
        };

        self.push_region(&region, bounds);
    }

    fn push_region(&mut self, region: &LayoutRegion, bounds: Bounds) {
        self.regions.push(ResolvedRegion {
            slot: region.slot,
            id: region.id.clone(),
            bounds,
            mode: region.mode,
            interactive: region.interactive,
        });
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

fn inset_bounds(bounds: Bounds, padding: f64) -> Bounds {
    Bounds::from_xywh(
        bounds.rect.x0 + padding,
        bounds.rect.y0 + padding,
        (bounds.rect.width() - padding * 2.0).max(0.0),
        (bounds.rect.height() - padding * 2.0).max(0.0),
    )
}

fn floating_side_bounds(viewport: Bounds, width: f64, from_start: bool) -> Bounds {
    if from_start {
        Bounds::from_xywh(
            viewport.rect.x0,
            viewport.rect.y0,
            width,
            viewport.rect.height(),
        )
    } else {
        Bounds::from_xywh(
            viewport.rect.x1 - width,
            viewport.rect.y0,
            width,
            viewport.rect.height(),
        )
    }
}

fn slot_color(slot: LayoutSlot, theme: &ThemeTokens) -> Color {
    match slot {
        LayoutSlot::Root => theme.text_secondary,
        LayoutSlot::TopBar | LayoutSlot::BottomBar => theme.accent,
        LayoutSlot::LeftRail | LayoutSlot::RightRail => theme.focus_ring,
        LayoutSlot::LeftCart | LayoutSlot::RightCart => Color::new([0.0, 0.68, 0.42, 1.0]),
        LayoutSlot::Content | LayoutSlot::ContentHeader | LayoutSlot::ContentFooter => {
            theme.text_primary
        }
        LayoutSlot::Floating | LayoutSlot::Overlay => Color::new([0.82, 0.42, 1.0, 1.0]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kurbo::{Point, Size};

    fn viewport() -> Bounds {
        Bounds::new(Point::ORIGIN, Size::new(1200.0, 800.0))
    }

    #[test]
    fn app_shell_resolves_top_left_and_content() {
        let resolved = UxiLayout::app_shell().resolve(viewport());

        assert_eq!(
            resolved
                .region(LayoutSlot::TopBar)
                .unwrap()
                .bounds
                .rect
                .height(),
            56.0
        );
        assert_eq!(
            resolved
                .region(LayoutSlot::LeftRail)
                .unwrap()
                .bounds
                .rect
                .width(),
            72.0
        );
        assert!(
            resolved
                .region(LayoutSlot::Content)
                .unwrap()
                .bounds
                .rect
                .width()
                > 1000.0
        );
    }

    #[test]
    fn dashboard_resolves_right_side_cart() {
        let resolved = UxiLayout::dashboard().resolve(viewport());
        let right_cart = resolved.region(LayoutSlot::RightCart).unwrap();
        let content = resolved.region(LayoutSlot::Content).unwrap();

        assert_eq!(right_cart.bounds.rect.width(), 320.0);
        assert!(content.bounds.rect.x1 <= right_cart.bounds.rect.x0);
    }

    #[test]
    fn floating_cart_does_not_reduce_content() {
        let docked = UxiLayout::new("docked")
            .with_right_cart(300.0)
            .resolve(viewport());
        let floating = UxiLayout::new("floating")
            .with_region(
                LayoutRegion::new(LayoutSlot::RightCart, "cart", LayoutSize::Fixed(300.0))
                    .with_mode(RegionMode::Floating),
            )
            .resolve(viewport());

        assert!(
            floating
                .region(LayoutSlot::Content)
                .unwrap()
                .bounds
                .rect
                .width()
                > docked
                    .region(LayoutSlot::Content)
                    .unwrap()
                    .bounds
                    .rect
                    .width()
        );
    }

    #[test]
    fn content_header_and_footer_reduce_content_height() {
        let resolved = UxiLayout::new("document")
            .with_content_header(48.0)
            .with_content_footer(32.0)
            .with_content_padding(8.0)
            .resolve(viewport());
        let content = resolved.region(LayoutSlot::Content).unwrap();

        assert_eq!(
            resolved
                .region(LayoutSlot::ContentHeader)
                .unwrap()
                .bounds
                .rect
                .height(),
            48.0
        );
        assert_eq!(
            resolved
                .region(LayoutSlot::ContentFooter)
                .unwrap()
                .bounds
                .rect
                .height(),
            32.0
        );
        assert!(content.bounds.rect.height() < 800.0);
    }

    #[test]
    fn breakpoint_tracks_viewport_width() {
        let compact = UxiLayout::new("compact").resolve(Bounds::from_xywh(0.0, 0.0, 390.0, 800.0));
        let medium = UxiLayout::new("medium").resolve(Bounds::from_xywh(0.0, 0.0, 800.0, 800.0));
        let expanded =
            UxiLayout::new("expanded").resolve(Bounds::from_xywh(0.0, 0.0, 1400.0, 800.0));

        assert_eq!(compact.breakpoint, LayoutBreakpoint::Compact);
        assert_eq!(medium.breakpoint, LayoutBreakpoint::Medium);
        assert_eq!(expanded.breakpoint, LayoutBreakpoint::Expanded);
    }

    #[test]
    fn region_replacement_and_removal_work() {
        let mut layout = UxiLayout::new("custom")
            .with_top_bar(48.0)
            .with_top_bar(64.0);
        let removed = layout.remove_region(LayoutSlot::TopBar).unwrap();

        assert_eq!(removed.size.resolve(100.0), 64.0);
        assert!(
            layout
                .resolve(viewport())
                .region(LayoutSlot::TopBar)
                .is_none()
        );
    }

    #[test]
    fn debug_primitives_emit_fill_and_stroke_per_region() {
        let resolved = UxiLayout::app_shell().resolve(viewport());
        let primitives = resolved.debug_primitives(&ThemeTokens::default());

        assert_eq!(primitives.len(), resolved.regions.len() * 2);
    }
}
