// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// This module provides an assortment of utility traits to ease interaction with [xcb].
mod extensions;

/// Contains setup and utilities for
/// [ICCCM](https://x.org/releases/X11R7.6/doc/xorg-docs/specs/ICCCM/icccm.html) compliance.
pub mod icccm;
/// Contains setup and utilities for
/// [EWMH](https://specifications.freedesktop.org/wm-spec/latest) compliance.
pub mod ewmh;

// Re-export extensions module.
pub use extensions::*;

use tracing::trace;

use xcb::x::{self, ConfigWindow, Window};
use xcb::{Connection, Xid};

/// Initializes the given [window](x::Window) by requesting to receive certain events on it.
///
/// This function grabs button press events on the window when the Super key is held (used for
/// beginning window manipulation) and registers for
/// [`FOCUS_CHANGE`](xcb::x::EventMask::FOCUS_CHANGE) and
/// [`ENTER_WINDOW`](xcb::x::EventMask::ENTER_WINDOW) events on the window.
///
/// While the grab is only for button _presses_ when the window is initialised, all button motion
/// and button release events are grabbed once a window manipulation has started and ungrabbed
/// when it ends.
///
/// Does not flush the connection.
pub fn init_window(window: Window, conn: &Connection, icccm_wm_state: x::Atom) {
	// Set the ICCCM's `WM_STATE` property to `NormalState` when the window is mapped. This is for
	// ICCCM compliance.
	conn.send_request(&x::ChangeProperty {
		mode: x::PropMode::Replace,
		window,
		// WM_STATE
		property: icccm_wm_state,
		// WM_STATE
	    r#type: icccm_wm_state,
		// data[0] = WM_STATE state, one of:
		//   0  WithdrawnState
		//   1  NormalState
		//   2  IconicState
		//
		// data[1] = window of the 'icon'... we don't support icons, certainly not yet, so we'll
		// just set this to NONE.
	    data: &[1u32, x::WINDOW_NONE.resource_id()],
	});

	init_grabs(conn, window);

	trace!(
		window = window.resource_id(),
		"Registering for events on window"
	);
	conn.send_request(&x::ChangeWindowAttributes {
		window,
		value_list: &[x::Cw::EventMask(
			x::EventMask::FOCUS_CHANGE | x::EventMask::ENTER_WINDOW,
		)],
	});
}

/// Sends a [GetGeometry](xcb::x::GetGeometry) request for the given window and returns its reply.
///
/// Does not flush the connection.
pub fn get_geometry(conn: &Connection, window: Window) -> x::GetGeometryCookie {
	trace!(window = window.resource_id(), "Requesting window geometry");
	conn.send_request(&x::GetGeometry {
		drawable: x::Drawable::Window(window),
	})
}

/// Sends a [ConfigureWindow](xcb::x::ConfigureWindow) request to change the coordinates of the
/// given window.
///
/// Does not flush the connection.
pub fn set_position(conn: &Connection, window: Window, coords: (i32, i32)) {
	trace!(
		window = window.resource_id(),
		x = coords.0,
		y = coords.1,
		"Configuring window coordinates"
	);
	conn.send_request(&x::ConfigureWindow {
		window,
		value_list: &[ConfigWindow::X(coords.0), ConfigWindow::Y(coords.1)],
	});
}

/// Sends a [ConfigureWindow](xcb::x::ConfigureWindow) request to change the dimensions of the
/// given window.
///
/// Does not flush the connection.
pub fn set_dimensions(conn: &Connection, window: Window, dimensions: (u32, u32)) {
	trace!(
		window = window.resource_id(),
		width = dimensions.0,
		y = dimensions.1,
		"Configuring window dimensions"
	);
	conn.send_request(&x::ConfigureWindow {
		window,
		value_list: &[
			ConfigWindow::Width(dimensions.0),
			ConfigWindow::Height(dimensions.1),
		],
	});
}

/// Grabs button presses on the given window when the Super key is held.
///
/// Used to initiate window manipulations.
///
/// Does not flush the connection.
pub fn init_grabs(conn: &Connection, window: Window) {
	trace!(
		window = window.resource_id(),
		"Grabbing button presses on window"
	);
	conn.send_request(&x::GrabButton {
		owner_events: false,
		grab_window: window,
		event_mask: x::EventMask::BUTTON_PRESS,
		pointer_mode: x::GrabMode::Async,
		keyboard_mode: x::GrabMode::Async,
		confine_to: x::WINDOW_NONE,
		cursor: x::CURSOR_NONE,
		button: x::ButtonIndex::Any,
		modifiers: x::ModMask::N4,
	});
}

/// Grabs button motion and releases on the given window.
///
/// Used when a window is being manipulated.
///
/// Does not flush the connection.
pub fn grab_manip_buttons(conn: &Connection, window: Window) {
	trace!(
		window = window.resource_id(),
		"Grabbing button motion and releases on window"
	);
	conn.send_request(&x::GrabButton {
		owner_events: false,
		grab_window: window,
		event_mask: x::EventMask::BUTTON_MOTION | x::EventMask::BUTTON_RELEASE,
		pointer_mode: x::GrabMode::Async,
		keyboard_mode: x::GrabMode::Async,
		confine_to: x::WINDOW_NONE,
		cursor: x::CURSOR_NONE,
		button: x::ButtonIndex::Any,
		modifiers: x::ModMask::ANY,
	});
}
