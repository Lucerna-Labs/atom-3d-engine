//! Renderer-neutral input handling experiments for Ordo UX.
//!
//! This crate models normalized input events, hit testing, focus traversal, and
//! simple gesture recognition. It does not own a window, event loop, platform
//! device, or renderer.

use kurbo::Point;
use ordo_ux_primitives::HitRegion;

/// Pointer button.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PointerButton {
    /// Primary button.
    Primary,
    /// Secondary button.
    Secondary,
    /// Middle button.
    Middle,
}

/// Normalized input event.
#[derive(Clone, Debug, PartialEq)]
pub enum InputEvent {
    /// Pointer pressed.
    PointerDown {
        /// Pointer position.
        position: Point,
        /// Button.
        button: PointerButton,
    },
    /// Pointer moved.
    PointerMove {
        /// Pointer position.
        position: Point,
    },
    /// Pointer released.
    PointerUp {
        /// Pointer position.
        position: Point,
        /// Button.
        button: PointerButton,
    },
    /// Keyboard key pressed.
    KeyDown {
        /// Logical key.
        key: String,
    },
    /// Keyboard key released.
    KeyUp {
        /// Logical key.
        key: String,
    },
}

/// Hit-test result.
#[derive(Clone, Debug, PartialEq)]
pub struct HitResult {
    /// Hit region id.
    pub id: String,
}

/// Hit tests regions in reverse paint order.
#[must_use]
pub fn hit_test(regions: &[HitRegion], position: Point) -> Option<HitResult> {
    regions
        .iter()
        .rev()
        .find(|region| region.enabled && region.bounds.contains(position))
        .map(|region| HitResult {
            id: region.id.clone(),
        })
}

/// Focus traversal order.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FocusOrder {
    /// Region ids in focus order.
    pub ids: Vec<String>,
    /// Current index.
    pub current: Option<usize>,
}

impl FocusOrder {
    /// Creates focus order.
    #[must_use]
    pub fn new(ids: Vec<String>) -> Self {
        Self { ids, current: None }
    }

    /// Moves focus forward.
    pub fn next(&mut self) -> Option<&str> {
        if self.ids.is_empty() {
            self.current = None;
            return None;
        }
        let next = self.current.map_or(0, |index| (index + 1) % self.ids.len());
        self.current = Some(next);
        self.ids.get(next).map(String::as_str)
    }

    /// Moves focus backward.
    pub fn previous(&mut self) -> Option<&str> {
        if self.ids.is_empty() {
            self.current = None;
            return None;
        }
        let previous = self.current.map_or(self.ids.len() - 1, |index| {
            (index + self.ids.len() - 1) % self.ids.len()
        });
        self.current = Some(previous);
        self.ids.get(previous).map(String::as_str)
    }
}

/// Simple pointer gesture recognizer.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GestureState {
    start: Option<Point>,
    current: Option<Point>,
}

impl GestureState {
    /// Handles input and returns a gesture event when recognized.
    pub fn handle(&mut self, event: InputEvent) -> Option<GestureEvent> {
        match event {
            InputEvent::PointerDown { position, .. } => {
                self.start = Some(position);
                self.current = Some(position);
                Some(GestureEvent::TapStart { position })
            }
            InputEvent::PointerMove { position } => {
                let start = self.start?;
                self.current = Some(position);
                Some(GestureEvent::Drag {
                    start,
                    current: position,
                    delta: position - start,
                })
            }
            InputEvent::PointerUp { position, .. } => {
                let start = self.start.take()?;
                self.current = None;
                let delta = position - start;
                if delta.hypot() < 4.0 {
                    Some(GestureEvent::Tap { position })
                } else {
                    Some(GestureEvent::DragEnd {
                        start,
                        end: position,
                        delta,
                    })
                }
            }
            _ => None,
        }
    }
}

/// Recognized gesture event.
#[derive(Clone, Debug, PartialEq)]
pub enum GestureEvent {
    /// Tap started.
    TapStart {
        /// Position.
        position: Point,
    },
    /// Tap completed.
    Tap {
        /// Position.
        position: Point,
    },
    /// Dragging.
    Drag {
        /// Start.
        start: Point,
        /// Current.
        current: Point,
        /// Delta.
        delta: kurbo::Vec2,
    },
    /// Drag ended.
    DragEnd {
        /// Start.
        start: Point,
        /// End.
        end: Point,
        /// Delta.
        delta: kurbo::Vec2,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use ordo_ux_primitives::Bounds;

    #[test]
    fn hit_test_uses_reverse_order() {
        let regions = vec![
            HitRegion::new("back", Bounds::from_xywh(0.0, 0.0, 20.0, 20.0)),
            HitRegion::new("front", Bounds::from_xywh(0.0, 0.0, 20.0, 20.0)),
        ];
        assert_eq!(
            hit_test(&regions, Point::new(4.0, 4.0)).map(|hit| hit.id),
            Some("front".to_string())
        );
    }

    #[test]
    fn focus_order_wraps() {
        let mut focus = FocusOrder::new(vec!["a".into(), "b".into()]);
        assert_eq!(focus.next(), Some("a"));
        assert_eq!(focus.next(), Some("b"));
        assert_eq!(focus.next(), Some("a"));
    }

    #[test]
    fn gesture_recognizes_drag_end() {
        let mut state = GestureState::default();
        state.handle(InputEvent::PointerDown {
            position: Point::new(0.0, 0.0),
            button: PointerButton::Primary,
        });
        let event = state.handle(InputEvent::PointerUp {
            position: Point::new(10.0, 0.0),
            button: PointerButton::Primary,
        });
        assert!(matches!(event, Some(GestureEvent::DragEnd { .. })));
    }
}
