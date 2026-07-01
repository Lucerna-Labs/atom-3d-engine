//! Dev-mode internal drag and drop experiments for Ordo UX.
//!
//! This crate is intentionally not a renderer and not a windowing layer. It
//! models the interaction state needed for a design/dev tool that can visually
//! reposition UX elements, then emits renderer-neutral primitives for overlays.
//!
//! Shipping applications should either avoid enabling this crate's `dev-dnd`
//! feature or set [`DesignModeConfig::enabled`] to `false`.

use kurbo::{Point, Vec2};
use ordo_ux_primitives::{
    Bounds, CursorHint, Fill, HitRegion, LineJoin, Primitive, Shape, Stroke, Transform,
};
use peniko::Color;

/// Runtime switch for the visual editing experience.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DesignModeConfig {
    /// Whether design-mode dragging should react to pointer input.
    pub enabled: bool,
    /// Whether overlay primitives should be emitted while design mode is active.
    pub show_overlays: bool,
    /// Grid size used when snapping is enabled.
    pub snap_grid: Option<f64>,
}

impl DesignModeConfig {
    /// Creates an enabled design-mode configuration for development builds.
    #[must_use]
    pub fn enabled() -> Self {
        Self {
            enabled: true,
            show_overlays: true,
            snap_grid: None,
        }
    }

    /// Creates a disabled configuration suitable for shipping builds.
    #[must_use]
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            show_overlays: false,
            snap_grid: None,
        }
    }

    /// Enables grid snapping.
    #[must_use]
    pub fn with_snap_grid(mut self, grid: f64) -> Self {
        self.snap_grid = Some(grid);
        self
    }
}

impl Default for DesignModeConfig {
    fn default() -> Self {
        Self::disabled()
    }
}

/// A UI element that can be repositioned in dev mode.
#[derive(Clone, Debug, PartialEq)]
pub struct Draggable {
    /// Stable element identifier.
    pub id: String,
    /// Current visual bounds.
    pub bounds: Bounds,
    /// Optional group or component name for tooling.
    pub group: Option<String>,
    /// Whether this element can currently be dragged.
    pub locked: bool,
}

impl Draggable {
    /// Creates an unlocked draggable element.
    #[must_use]
    pub fn new(id: impl Into<String>, bounds: Bounds) -> Self {
        Self {
            id: id.into(),
            bounds,
            group: None,
            locked: false,
        }
    }

    /// Adds a tooling group name.
    #[must_use]
    pub fn grouped(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    /// Locks the element against drag changes.
    #[must_use]
    pub fn locked(mut self) -> Self {
        self.locked = true;
        self
    }

    /// Returns the hit region for this draggable element.
    #[must_use]
    pub fn hit_region(&self) -> HitRegion {
        HitRegion::new(self.id.clone(), self.bounds)
            .with_shape(Shape::Rect(self.bounds.rect))
            .with_cursor(CursorHint::Grab)
    }
}

/// A place where dragged elements may be dropped.
#[derive(Clone, Debug, PartialEq)]
pub struct DropZone {
    /// Stable drop-zone identifier.
    pub id: String,
    /// Zone bounds.
    pub bounds: Bounds,
    /// Optional group accepted by this drop zone.
    pub accepts_group: Option<String>,
}

impl DropZone {
    /// Creates a drop zone.
    #[must_use]
    pub fn new(id: impl Into<String>, bounds: Bounds) -> Self {
        Self {
            id: id.into(),
            bounds,
            accepts_group: None,
        }
    }

    /// Restricts this drop zone to one draggable group.
    #[must_use]
    pub fn accepting_group(mut self, group: impl Into<String>) -> Self {
        self.accepts_group = Some(group.into());
        self
    }

    fn accepts(&self, draggable: &Draggable) -> bool {
        match (&self.accepts_group, &draggable.group) {
            (Some(accepted), Some(group)) => accepted == group,
            (Some(_), None) => false,
            (None, _) => true,
        }
    }
}

/// Pointer input consumed by the drag controller.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PointerEvent {
    /// Pointer pressed.
    Down { position: Point },
    /// Pointer moved.
    Move { position: Point },
    /// Pointer released.
    Up { position: Point },
    /// Pointer gesture cancelled.
    Cancel,
}

/// Drag lifecycle messages emitted by the controller.
#[derive(Clone, Debug, PartialEq)]
pub enum DragEvent {
    /// A draggable element began moving.
    Started {
        /// Draggable identifier.
        id: String,
        /// Pointer position where drag began.
        origin: Point,
    },
    /// A draggable element moved.
    Moved {
        /// Draggable identifier.
        id: String,
        /// Current pointer position.
        position: Point,
        /// Movement from drag origin, after snapping.
        delta: Vec2,
    },
    /// A draggable element was dropped.
    Dropped {
        /// Draggable identifier.
        id: String,
        /// Drop zone under the pointer, if any.
        zone_id: Option<String>,
        /// Final movement from drag origin, after snapping.
        delta: Vec2,
    },
    /// The active drag was cancelled.
    Cancelled {
        /// Draggable identifier.
        id: String,
    },
}

/// Current active drag state.
#[derive(Clone, Debug, PartialEq)]
pub struct ActiveDrag {
    /// Draggable identifier.
    pub id: String,
    /// Bounds when the drag began.
    pub initial_bounds: Bounds,
    /// Pointer position where the drag began.
    pub origin: Point,
    /// Last pointer position seen by the controller.
    pub current: Point,
    /// Movement from drag origin, after snapping.
    pub delta: Vec2,
}

/// State machine for internal dev-mode drag and drop.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DragController {
    active: Option<ActiveDrag>,
}

impl DragController {
    /// Creates a new idle controller.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the active drag, if any.
    #[must_use]
    pub fn active(&self) -> Option<&ActiveDrag> {
        self.active.as_ref()
    }

    /// Handles pointer input and returns drag lifecycle events.
    pub fn handle_pointer_event(
        &mut self,
        config: DesignModeConfig,
        draggables: &[Draggable],
        drop_zones: &[DropZone],
        event: PointerEvent,
    ) -> Option<DragEvent> {
        if !config.enabled {
            self.active = None;
            return None;
        }

        match event {
            PointerEvent::Down { position } => self.start_drag(draggables, position),
            PointerEvent::Move { position } => self.move_drag(config, position),
            PointerEvent::Up { position } => {
                self.drop_drag(config, draggables, drop_zones, position)
            }
            PointerEvent::Cancel => self.cancel_drag(),
        }
    }

    /// Emits visual overlay primitives for the current design-mode state.
    #[must_use]
    pub fn overlay_primitives(
        &self,
        config: DesignModeConfig,
        draggables: &[Draggable],
        drop_zones: &[DropZone],
    ) -> Vec<Primitive> {
        if !config.enabled || !config.show_overlays {
            return Vec::new();
        }

        let mut primitives = Vec::new();

        for zone in drop_zones {
            primitives.push(Primitive::stroke(
                Shape::Rect(zone.bounds.rect),
                Stroke::new(Color::new([0.1, 0.45, 0.9, 1.0]), 1.0)
                    .with_line_join(LineJoin::Round)
                    .with_dashes(0.0, vec![4.0, 4.0]),
            ));
        }

        for draggable in draggables {
            let color = if self
                .active
                .as_ref()
                .is_some_and(|active| active.id == draggable.id)
            {
                Color::new([0.0, 0.65, 0.35, 1.0])
            } else {
                Color::new([0.2, 0.2, 0.2, 0.7])
            };

            primitives.push(Primitive::stroke(
                Shape::Rect(draggable.bounds.rect),
                Stroke::new(color, 1.0),
            ));
        }

        if let Some(active) = &self.active {
            let preview_bounds = active
                .initial_bounds
                .transformed(Transform::translate(active.delta.x, active.delta.y));
            primitives.push(Primitive::fill(
                Shape::Rect(preview_bounds.rect),
                Fill::new(Color::new([0.0, 0.65, 0.35, 0.14])),
            ));
        }

        primitives
    }

    fn start_drag(&mut self, draggables: &[Draggable], position: Point) -> Option<DragEvent> {
        let draggable = draggables
            .iter()
            .rev()
            .find(|draggable| !draggable.locked && draggable.bounds.contains(position))?;

        self.active = Some(ActiveDrag {
            id: draggable.id.clone(),
            initial_bounds: draggable.bounds,
            origin: position,
            current: position,
            delta: Vec2::ZERO,
        });

        Some(DragEvent::Started {
            id: draggable.id.clone(),
            origin: position,
        })
    }

    fn move_drag(&mut self, config: DesignModeConfig, position: Point) -> Option<DragEvent> {
        let active = self.active.as_mut()?;
        active.current = position;
        active.delta = snapped_delta(active.origin, position, config.snap_grid);

        Some(DragEvent::Moved {
            id: active.id.clone(),
            position,
            delta: active.delta,
        })
    }

    fn drop_drag(
        &mut self,
        config: DesignModeConfig,
        draggables: &[Draggable],
        drop_zones: &[DropZone],
        position: Point,
    ) -> Option<DragEvent> {
        let mut active = self.active.take()?;
        active.current = position;
        active.delta = snapped_delta(active.origin, position, config.snap_grid);

        let draggable = draggables
            .iter()
            .find(|draggable| draggable.id == active.id)?;
        let zone_id = drop_zones
            .iter()
            .rev()
            .find(|zone| zone.bounds.contains(position) && zone.accepts(draggable))
            .map(|zone| zone.id.clone());

        Some(DragEvent::Dropped {
            id: active.id,
            zone_id,
            delta: active.delta,
        })
    }

    fn cancel_drag(&mut self) -> Option<DragEvent> {
        let active = self.active.take()?;
        Some(DragEvent::Cancelled { id: active.id })
    }
}

fn snapped_delta(origin: Point, position: Point, snap_grid: Option<f64>) -> Vec2 {
    let mut delta = position - origin;

    if let Some(grid) = snap_grid
        && grid > 0.0
    {
        delta.x = (delta.x / grid).round() * grid;
        delta.y = (delta.y / grid).round() * grid;
    }

    delta
}

#[cfg(test)]
mod tests {
    use super::*;

    fn draggable() -> Draggable {
        Draggable::new("hero-card", Bounds::from_xywh(10.0, 10.0, 100.0, 40.0)).grouped("cards")
    }

    #[test]
    fn disabled_design_mode_ignores_input() {
        let mut controller = DragController::new();
        let event = controller.handle_pointer_event(
            DesignModeConfig::disabled(),
            &[draggable()],
            &[],
            PointerEvent::Down {
                position: Point::new(20.0, 20.0),
            },
        );

        assert_eq!(event, None);
        assert_eq!(controller.active(), None);
    }

    #[test]
    fn drag_lifecycle_emits_started_moved_and_dropped() {
        let mut controller = DragController::new();
        let config = DesignModeConfig::enabled();
        let draggables = [draggable()];
        let zones = [
            DropZone::new("main-canvas", Bounds::from_xywh(0.0, 0.0, 500.0, 500.0))
                .accepting_group("cards"),
        ];

        let started = controller.handle_pointer_event(
            config,
            &draggables,
            &zones,
            PointerEvent::Down {
                position: Point::new(20.0, 20.0),
            },
        );
        assert!(matches!(started, Some(DragEvent::Started { .. })));

        let moved = controller.handle_pointer_event(
            config,
            &draggables,
            &zones,
            PointerEvent::Move {
                position: Point::new(35.0, 45.0),
            },
        );
        assert!(matches!(
            moved,
            Some(DragEvent::Moved {
                delta,
                ..
            }) if delta == Vec2::new(15.0, 25.0)
        ));

        let dropped = controller.handle_pointer_event(
            config,
            &draggables,
            &zones,
            PointerEvent::Up {
                position: Point::new(35.0, 45.0),
            },
        );
        assert!(matches!(
            dropped,
            Some(DragEvent::Dropped {
                zone_id: Some(zone_id),
                ..
            }) if zone_id == "main-canvas"
        ));
        assert_eq!(controller.active(), None);
    }

    #[test]
    fn snap_grid_rounds_drag_delta() {
        let mut controller = DragController::new();
        let config = DesignModeConfig::enabled().with_snap_grid(8.0);
        let draggables = [draggable()];

        controller.handle_pointer_event(
            config,
            &draggables,
            &[],
            PointerEvent::Down {
                position: Point::new(20.0, 20.0),
            },
        );

        let moved = controller.handle_pointer_event(
            config,
            &draggables,
            &[],
            PointerEvent::Move {
                position: Point::new(31.0, 35.0),
            },
        );

        assert!(matches!(
            moved,
            Some(DragEvent::Moved {
                delta,
                ..
            }) if delta == Vec2::new(8.0, 16.0)
        ));
    }

    #[test]
    fn overlay_primitives_are_dev_mode_only() {
        let controller = DragController::new();
        let draggables = [draggable()];
        let zones = [DropZone::new(
            "canvas",
            Bounds::from_xywh(0.0, 0.0, 200.0, 200.0),
        )];

        assert!(
            controller
                .overlay_primitives(DesignModeConfig::disabled(), &draggables, &zones)
                .is_empty()
        );
        assert_eq!(
            controller
                .overlay_primitives(DesignModeConfig::enabled(), &draggables, &zones)
                .len(),
            2
        );
    }

    #[test]
    fn locked_draggables_do_not_start_drag() {
        let mut controller = DragController::new();
        let locked = [draggable().locked()];
        let event = controller.handle_pointer_event(
            DesignModeConfig::enabled(),
            &locked,
            &[],
            PointerEvent::Down {
                position: Point::new(20.0, 20.0),
            },
        );

        assert_eq!(event, None);
    }
}
