//! Renderer-neutral dropdown menu and submenu experiments for Ordo UX.
//!
//! This crate models menu trees, dropdown state, submenu opening, keyboard-like
//! navigation, and primitive output. It does not render, own focus, listen to
//! platform events, or execute actions.

use kurbo::{Point, Size};
use ordo_ux_overlays_experiment::{Overlay, OverlayChrome, OverlayLayout, OverlayPlacement};
use ordo_ux_primitives::{
    Bounds, CursorHint, Fill, HitRegion, Layer, Primitive, Shape, Stroke, TextAlign, TextRun,
    ThemeTokens,
};
use peniko::Color;

/// Menu item behavior.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MenuItemKind {
    /// A normal command item.
    #[default]
    Command,
    /// A checkbox-style item.
    Checkbox,
    /// A radio item.
    Radio,
    /// A visual separator.
    Separator,
    /// An item that opens a submenu.
    Submenu,
}

/// Menu item state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MenuItemState {
    /// Whether the item can be activated.
    pub enabled: bool,
    /// Whether the item is checked or selected.
    pub checked: bool,
    /// Whether the item is currently highlighted.
    pub highlighted: bool,
}

impl MenuItemState {
    /// Enabled item state.
    pub const ENABLED: Self = Self {
        enabled: true,
        checked: false,
        highlighted: false,
    };

    /// Disabled item state.
    pub const DISABLED: Self = Self {
        enabled: false,
        checked: false,
        highlighted: false,
    };
}

impl Default for MenuItemState {
    fn default() -> Self {
        Self::ENABLED
    }
}

/// A menu item, including optional submenu children.
#[derive(Clone, Debug, PartialEq)]
pub struct MenuItem {
    /// Stable item id.
    pub id: String,
    /// User-visible label.
    pub label: String,
    /// Optional accelerator text.
    pub shortcut: Option<String>,
    /// Item kind.
    pub kind: MenuItemKind,
    /// Item state.
    pub state: MenuItemState,
    /// Optional radio group id.
    pub radio_group: Option<String>,
    /// Child submenu items.
    pub children: Vec<MenuItem>,
}

impl MenuItem {
    /// Creates a command item.
    #[must_use]
    pub fn command(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            shortcut: None,
            kind: MenuItemKind::Command,
            state: MenuItemState::default(),
            radio_group: None,
            children: Vec::new(),
        }
    }

    /// Creates a separator item.
    #[must_use]
    pub fn separator() -> Self {
        Self {
            id: "separator".to_string(),
            label: String::new(),
            shortcut: None,
            kind: MenuItemKind::Separator,
            state: MenuItemState::DISABLED,
            radio_group: None,
            children: Vec::new(),
        }
    }

    /// Creates a submenu item.
    #[must_use]
    pub fn submenu(
        id: impl Into<String>,
        label: impl Into<String>,
        children: Vec<MenuItem>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            shortcut: None,
            kind: MenuItemKind::Submenu,
            state: MenuItemState::default(),
            radio_group: None,
            children,
        }
    }

    /// Converts this item into a checkbox.
    #[must_use]
    pub fn checkbox(mut self, checked: bool) -> Self {
        self.kind = MenuItemKind::Checkbox;
        self.state.checked = checked;
        self
    }

    /// Converts this item into a radio item.
    #[must_use]
    pub fn radio(mut self, group: impl Into<String>, checked: bool) -> Self {
        self.kind = MenuItemKind::Radio;
        self.radio_group = Some(group.into());
        self.state.checked = checked;
        self
    }

    /// Adds shortcut text.
    #[must_use]
    pub fn with_shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    /// Disables this item.
    #[must_use]
    pub fn disabled(mut self) -> Self {
        self.state.enabled = false;
        self
    }

    /// Returns true if the item can receive highlight.
    #[must_use]
    pub fn is_focusable(&self) -> bool {
        self.state.enabled && self.kind != MenuItemKind::Separator
    }
}

/// Menu presentation metrics.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MenuMetrics {
    /// Menu width.
    pub width: f64,
    /// Height of a normal row.
    pub row_height: f64,
    /// Height of a separator.
    pub separator_height: f64,
    /// Horizontal padding.
    pub padding_x: f64,
    /// Icon/check column width.
    pub check_column: f64,
    /// Shortcut column width.
    pub shortcut_column: f64,
}

impl Default for MenuMetrics {
    fn default() -> Self {
        Self {
            width: 240.0,
            row_height: 34.0,
            separator_height: 9.0,
            padding_x: 10.0,
            check_column: 24.0,
            shortcut_column: 72.0,
        }
    }
}

/// Menu visual treatment.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MenuChrome {
    /// Surface fill.
    pub surface: Color,
    /// Border color.
    pub border: Color,
    /// Text color.
    pub text: Color,
    /// Disabled text color.
    pub disabled_text: Color,
    /// Highlight fill.
    pub highlight: Color,
    /// Checkmark/accent color.
    pub accent: Color,
    /// Corner radius.
    pub radius: f64,
}

impl MenuChrome {
    /// Creates menu chrome from theme tokens.
    #[must_use]
    pub fn from_theme(theme: &ThemeTokens) -> Self {
        Self {
            surface: theme.surface,
            border: theme.border,
            text: theme.text_primary,
            disabled_text: theme.text_secondary.with_alpha(0.48),
            highlight: theme.accent.with_alpha(0.12),
            accent: theme.accent,
            radius: theme.radius_md,
        }
    }
}

/// A dropdown menu tree.
#[derive(Clone, Debug, PartialEq)]
pub struct DropdownMenu {
    /// Stable menu id.
    pub id: String,
    /// Top-level menu items.
    pub items: Vec<MenuItem>,
    /// Presentation metrics.
    pub metrics: MenuMetrics,
    /// Visual treatment.
    pub chrome: MenuChrome,
}

impl DropdownMenu {
    /// Creates a menu.
    #[must_use]
    pub fn new(id: impl Into<String>, items: Vec<MenuItem>, chrome: MenuChrome) -> Self {
        Self {
            id: id.into(),
            items,
            metrics: MenuMetrics::default(),
            chrome,
        }
    }

    /// Sets presentation metrics.
    #[must_use]
    pub fn with_metrics(mut self, metrics: MenuMetrics) -> Self {
        self.metrics = metrics;
        self
    }

    /// Returns a menu item by path.
    #[must_use]
    pub fn item_at_path(&self, path: &[usize]) -> Option<&MenuItem> {
        item_at_path(&self.items, path)
    }

    /// Returns a mutable menu item by path.
    #[must_use]
    pub fn item_at_path_mut(&mut self, path: &[usize]) -> Option<&mut MenuItem> {
        item_at_path_mut(&mut self.items, path)
    }

    /// Computes total menu height for an item slice.
    #[must_use]
    pub fn height_for_items(&self, items: &[MenuItem]) -> f64 {
        items
            .iter()
            .map(|item| {
                if item.kind == MenuItemKind::Separator {
                    self.metrics.separator_height
                } else {
                    self.metrics.row_height
                }
            })
            .sum()
    }

    /// Returns primitives for the open menu stack.
    #[must_use]
    pub fn primitives(&self, state: &MenuState, layout: OverlayLayout) -> Vec<Primitive> {
        let mut primitives = Vec::new();
        if !state.open {
            return primitives;
        }

        let root_bounds = self.menu_bounds(&self.items, layout);
        primitives.push(self.menu_layer(&self.items, &[], root_bounds));

        let mut parent_bounds = root_bounds;
        let mut path = Vec::new();
        for &index in &state.open_submenus {
            path.push(index);
            let Some(item) = self.item_at_path(&path) else {
                break;
            };
            if item.children.is_empty() {
                break;
            }

            let row_bounds = self.row_bounds(
                parent_bounds,
                index,
                self.items_for_parent_path(&path[..path.len() - 1]),
            );
            let submenu_bounds = self.submenu_bounds(&item.children, row_bounds, layout.viewport);
            primitives.push(self.menu_layer(&item.children, &path, submenu_bounds));
            parent_bounds = submenu_bounds;
        }

        primitives
    }

    fn menu_bounds(&self, items: &[MenuItem], layout: OverlayLayout) -> Bounds {
        let height = self.height_for_items(items);
        let overlay = Overlay::new(
            format!("menu:{}", self.id),
            ordo_ux_overlays_experiment::OverlayKind::Popover,
            OverlayChrome::from_theme(&ThemeTokens::default()).with_scrim(None),
        )
        .anchored_to(layout.viewport)
        .with_placement(OverlayPlacement::Bottom);

        overlay.placed_bounds(OverlayLayout {
            size: Size::new(self.metrics.width, height),
            ..layout
        })
    }

    fn submenu_bounds(&self, items: &[MenuItem], row_bounds: Bounds, viewport: Bounds) -> Bounds {
        let height = self.height_for_items(items);
        let width = self.metrics.width;
        let mut x = row_bounds.rect.x1 + 6.0;
        if x + width > viewport.rect.x1 - 8.0 {
            x = row_bounds.rect.x0 - width - 6.0;
        }
        let y = row_bounds
            .rect
            .y0
            .clamp(viewport.rect.y0 + 8.0, viewport.rect.y1 - height - 8.0);

        Bounds::from_xywh(x, y, width, height)
    }

    fn menu_layer(&self, items: &[MenuItem], path_prefix: &[usize], bounds: Bounds) -> Primitive {
        let mut layer = Layer::new().named(format!("menu:{}", self.id));
        layer.push(Primitive::fill(
            Shape::rounded_rect(
                bounds.rect.x0,
                bounds.rect.y0,
                bounds.rect.width(),
                bounds.rect.height(),
                self.chrome.radius,
            ),
            Fill::new(self.chrome.surface),
        ));
        layer.push(Primitive::stroke(
            Shape::rounded_rect(
                bounds.rect.x0,
                bounds.rect.y0,
                bounds.rect.width(),
                bounds.rect.height(),
                self.chrome.radius,
            ),
            Stroke::new(self.chrome.border, 1.0),
        ));

        let mut y = bounds.rect.y0;
        for (index, item) in items.iter().enumerate() {
            let height = if item.kind == MenuItemKind::Separator {
                self.metrics.separator_height
            } else {
                self.metrics.row_height
            };
            let row = Bounds::from_xywh(bounds.rect.x0, y, bounds.rect.width(), height);
            self.push_item_primitives(&mut layer, item, row, path_prefix, index);
            y += height;
        }

        Primitive::layer(layer)
    }

    fn push_item_primitives(
        &self,
        layer: &mut Layer,
        item: &MenuItem,
        row: Bounds,
        path_prefix: &[usize],
        index: usize,
    ) {
        if item.kind == MenuItemKind::Separator {
            let y = row.rect.y0 + row.rect.height() / 2.0;
            layer.push(Primitive::stroke(
                Shape::line(
                    Point::new(row.rect.x0 + 10.0, y),
                    Point::new(row.rect.x1 - 10.0, y),
                ),
                Stroke::new(self.chrome.border, 1.0),
            ));
            return;
        }

        if item.state.highlighted {
            layer.push(Primitive::fill(
                Shape::rounded_rect(
                    row.rect.x0 + 4.0,
                    row.rect.y0 + 2.0,
                    row.rect.width() - 8.0,
                    row.rect.height() - 4.0,
                    6.0,
                ),
                Fill::new(self.chrome.highlight),
            ));
        }

        let mut path = path_prefix.to_vec();
        path.push(index);
        let hit_region = HitRegion::new(path_to_id(&self.id, &path), row)
            .with_shape(Shape::Rect(row.rect))
            .with_cursor(CursorHint::Pointer);

        if item.state.checked {
            let marker = if item.kind == MenuItemKind::Radio {
                "•"
            } else {
                "✓"
            };
            layer.push(Primitive::text(
                TextRun::new(
                    marker,
                    Point::new(row.rect.x0 + self.metrics.padding_x, row.rect.y0 + 22.0),
                )
                .with_font("System", 13.0)
                .with_fill(Fill::new(self.chrome.accent))
                .with_bounds(row),
            ));
        }

        let text_color = if item.state.enabled {
            self.chrome.text
        } else {
            self.chrome.disabled_text
        };
        layer.push(
            Primitive::text(
                TextRun::new(
                    item.label.clone(),
                    Point::new(
                        row.rect.x0 + self.metrics.padding_x + self.metrics.check_column,
                        row.rect.y0 + 22.0,
                    ),
                )
                .with_font("System", 13.0)
                .with_fill(Fill::new(text_color))
                .with_bounds(row),
            )
            .with_hit_region(hit_region),
        );

        if let Some(shortcut) = &item.shortcut {
            layer.push(Primitive::text(
                TextRun::new(
                    shortcut.clone(),
                    Point::new(
                        row.rect.x1 - self.metrics.shortcut_column,
                        row.rect.y0 + 22.0,
                    ),
                )
                .with_font("System", 12.0)
                .with_align(TextAlign::End)
                .with_fill(Fill::new(text_color.with_alpha(0.66)))
                .with_bounds(row),
            ));
        }

        if item.kind == MenuItemKind::Submenu {
            layer.push(Primitive::text(
                TextRun::new("›", Point::new(row.rect.x1 - 20.0, row.rect.y0 + 22.0))
                    .with_font("System", 15.0)
                    .with_fill(Fill::new(text_color.with_alpha(0.78)))
                    .with_bounds(row),
            ));
        }
    }

    fn row_bounds(&self, menu_bounds: Bounds, index: usize, items: &[MenuItem]) -> Bounds {
        let y_offset: f64 = items
            .iter()
            .take(index)
            .map(|item| {
                if item.kind == MenuItemKind::Separator {
                    self.metrics.separator_height
                } else {
                    self.metrics.row_height
                }
            })
            .sum();
        let height = items.get(index).map_or(self.metrics.row_height, |item| {
            if item.kind == MenuItemKind::Separator {
                self.metrics.separator_height
            } else {
                self.metrics.row_height
            }
        });

        Bounds::from_xywh(
            menu_bounds.rect.x0,
            menu_bounds.rect.y0 + y_offset,
            menu_bounds.rect.width(),
            height,
        )
    }

    fn items_for_parent_path(&self, parent_path: &[usize]) -> &[MenuItem] {
        if parent_path.is_empty() {
            &self.items
        } else {
            self.item_at_path(parent_path)
                .map(|item| item.children.as_slice())
                .unwrap_or(&[])
        }
    }
}

/// Open/highlight state for a dropdown menu.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MenuState {
    /// Whether the root menu is open.
    pub open: bool,
    /// Highlighted path into the menu tree.
    pub highlighted_path: Vec<usize>,
    /// Open submenu indexes from root downward.
    pub open_submenus: Vec<usize>,
}

impl MenuState {
    /// Creates closed state.
    #[must_use]
    pub fn closed() -> Self {
        Self::default()
    }

    /// Opens the root menu.
    #[must_use]
    pub fn opened() -> Self {
        Self {
            open: true,
            highlighted_path: Vec::new(),
            open_submenus: Vec::new(),
        }
    }

    /// Closes all menus.
    pub fn close(&mut self) {
        self.open = false;
        self.highlighted_path.clear();
        self.open_submenus.clear();
    }

    /// Opens a submenu path.
    pub fn open_submenu_path(&mut self, path: Vec<usize>) {
        self.open = true;
        self.open_submenus = path;
    }
}

/// Menu navigation commands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuNav {
    /// Move highlight down.
    Next,
    /// Move highlight up.
    Previous,
    /// Open highlighted submenu.
    OpenSubmenu,
    /// Close current submenu.
    CloseSubmenu,
    /// Activate highlighted item.
    Activate,
    /// Close all menus.
    Escape,
}

/// Output from menu navigation.
#[derive(Clone, Debug, PartialEq)]
pub enum MenuNavOutcome {
    /// Highlight moved.
    Highlighted(Vec<usize>),
    /// Submenu opened.
    SubmenuOpened(Vec<usize>),
    /// Submenu closed or root closed.
    Closed,
    /// Item activated.
    Activated(String),
    /// Nothing changed.
    Ignored,
}

/// Handles menu navigation state transitions.
pub fn navigate(menu: &DropdownMenu, state: &mut MenuState, nav: MenuNav) -> MenuNavOutcome {
    if !state.open && nav != MenuNav::Escape {
        state.open = true;
    }

    match nav {
        MenuNav::Next => move_highlight(menu, state, 1),
        MenuNav::Previous => move_highlight(menu, state, -1),
        MenuNav::OpenSubmenu => open_highlighted_submenu(menu, state),
        MenuNav::CloseSubmenu => {
            if state.open_submenus.pop().is_some() {
                MenuNavOutcome::SubmenuOpened(state.open_submenus.clone())
            } else {
                state.close();
                MenuNavOutcome::Closed
            }
        }
        MenuNav::Activate => activate_highlighted(menu, state),
        MenuNav::Escape => {
            state.close();
            MenuNavOutcome::Closed
        }
    }
}

fn move_highlight(menu: &DropdownMenu, state: &mut MenuState, direction: isize) -> MenuNavOutcome {
    let items = if state.open_submenus.is_empty() {
        &menu.items
    } else {
        menu.item_at_path(&state.open_submenus)
            .map(|item| item.children.as_slice())
            .unwrap_or(&menu.items)
    };
    let parent = if state.open_submenus.is_empty() {
        Vec::new()
    } else {
        state.open_submenus.clone()
    };
    let focusable: Vec<_> = items
        .iter()
        .enumerate()
        .filter(|(_, item)| item.is_focusable())
        .map(|(index, _)| index)
        .collect();

    if focusable.is_empty() {
        return MenuNavOutcome::Ignored;
    }

    let current = state.highlighted_path.last().copied();
    let next_position =
        match current.and_then(|current| focusable.iter().position(|index| *index == current)) {
            Some(position) => {
                (position as isize + direction).rem_euclid(focusable.len() as isize) as usize
            }
            None if direction >= 0 => 0,
            None => focusable.len() - 1,
        };
    let mut next_path = parent;
    next_path.push(focusable[next_position]);
    state.highlighted_path = next_path.clone();
    MenuNavOutcome::Highlighted(next_path)
}

fn open_highlighted_submenu(menu: &DropdownMenu, state: &mut MenuState) -> MenuNavOutcome {
    let Some(item) = menu.item_at_path(&state.highlighted_path) else {
        return MenuNavOutcome::Ignored;
    };
    if item.kind != MenuItemKind::Submenu || item.children.is_empty() {
        return MenuNavOutcome::Ignored;
    }

    state.open_submenu_path(state.highlighted_path.clone());
    MenuNavOutcome::SubmenuOpened(state.open_submenus.clone())
}

fn activate_highlighted(menu: &DropdownMenu, state: &mut MenuState) -> MenuNavOutcome {
    let Some(item) = menu.item_at_path(&state.highlighted_path) else {
        return MenuNavOutcome::Ignored;
    };
    if !item.is_focusable() || item.kind == MenuItemKind::Submenu {
        return MenuNavOutcome::Ignored;
    }

    MenuNavOutcome::Activated(item.id.clone())
}

fn item_at_path<'a>(items: &'a [MenuItem], path: &[usize]) -> Option<&'a MenuItem> {
    let (first, rest) = path.split_first()?;
    let item = items.get(*first)?;
    if rest.is_empty() {
        Some(item)
    } else {
        item_at_path(&item.children, rest)
    }
}

fn item_at_path_mut<'a>(items: &'a mut [MenuItem], path: &[usize]) -> Option<&'a mut MenuItem> {
    let (first, rest) = path.split_first()?;
    let item = items.get_mut(*first)?;
    if rest.is_empty() {
        Some(item)
    } else {
        item_at_path_mut(&mut item.children, rest)
    }
}

fn path_to_id(menu_id: &str, path: &[usize]) -> String {
    let path = path
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join(".");
    format!("{menu_id}:{path}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn menu() -> DropdownMenu {
        DropdownMenu::new(
            "file",
            vec![
                MenuItem::command("new", "New").with_shortcut("Ctrl+N"),
                MenuItem::command("open", "Open"),
                MenuItem::separator(),
                MenuItem::submenu(
                    "export",
                    "Export",
                    vec![
                        MenuItem::command("png", "PNG"),
                        MenuItem::command("svg", "SVG").disabled(),
                    ],
                ),
                MenuItem::command("autosave", "Auto Save").checkbox(true),
                MenuItem::command("compact", "Compact").radio("density", false),
            ],
            MenuChrome::from_theme(&ThemeTokens::default()),
        )
    }

    fn layout() -> OverlayLayout {
        OverlayLayout::new(
            Bounds::from_xywh(40.0, 40.0, 400.0, 300.0),
            Size::new(240.0, 120.0),
        )
    }

    #[test]
    fn menu_items_support_variants() {
        let menu = menu();

        assert_eq!(menu.items[2].kind, MenuItemKind::Separator);
        assert_eq!(menu.items[3].kind, MenuItemKind::Submenu);
        assert!(menu.items[4].state.checked);
        assert_eq!(menu.items[5].radio_group.as_deref(), Some("density"));
    }

    #[test]
    fn item_lookup_by_path_reaches_submenu_children() {
        let menu = menu();

        assert_eq!(
            menu.item_at_path(&[3, 0]).map(|item| item.id.as_str()),
            Some("png")
        );
        assert!(
            menu.item_at_path(&[3, 1])
                .is_some_and(|item| !item.state.enabled)
        );
    }

    #[test]
    fn navigation_skips_separators_and_disabled_items() {
        let menu = menu();
        let mut state = MenuState::opened();

        assert_eq!(
            navigate(&menu, &mut state, MenuNav::Next),
            MenuNavOutcome::Highlighted(vec![0])
        );
        assert_eq!(
            navigate(&menu, &mut state, MenuNav::Next),
            MenuNavOutcome::Highlighted(vec![1])
        );
        assert_eq!(
            navigate(&menu, &mut state, MenuNav::Next),
            MenuNavOutcome::Highlighted(vec![3])
        );
    }

    #[test]
    fn navigation_opens_submenus_and_activates_child_items() {
        let menu = menu();
        let mut state = MenuState::opened();
        state.highlighted_path = vec![3];

        assert_eq!(
            navigate(&menu, &mut state, MenuNav::OpenSubmenu),
            MenuNavOutcome::SubmenuOpened(vec![3])
        );
        assert_eq!(
            navigate(&menu, &mut state, MenuNav::Next),
            MenuNavOutcome::Highlighted(vec![3, 0])
        );
        assert_eq!(
            navigate(&menu, &mut state, MenuNav::Activate),
            MenuNavOutcome::Activated("png".to_string())
        );
    }

    #[test]
    fn closed_menu_emits_no_primitives() {
        let menu = menu();

        assert!(menu.primitives(&MenuState::closed(), layout()).is_empty());
    }

    #[test]
    fn open_menu_emits_root_and_submenu_layers() {
        let menu = menu();
        let mut state = MenuState::opened();
        state.open_submenu_path(vec![3]);

        let primitives = menu.primitives(&state, layout());

        assert_eq!(primitives.len(), 2);
        assert!(matches!(primitives.first(), Some(Primitive::Layer(_))));
    }

    #[test]
    fn mutable_path_lookup_can_update_items() {
        let mut menu = menu();
        menu.item_at_path_mut(&[4]).unwrap().state.checked = false;

        assert!(!menu.item_at_path(&[4]).unwrap().state.checked);
    }
}
