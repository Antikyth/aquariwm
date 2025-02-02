// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use xcb_wm::icccm as i;

use crate::features::Atoms;

/// Wraps an [xcb::Connection] for easy interaction according to the
/// [ICCCM](https://x.org/releases/X11R7.6/doc/xorg-docs/specs/ICCCM/icccm.html).
///
/// Returns the wrapped ICCCM connection and the `WM_STATE` atom if [`Ok`]. [`Err`] means that
/// there was an error in the process of sending an [InternAtom](xcb::x::InternAtom) request to the
/// X server.
///
/// Blocking: waits for the `WM_STATE` atom.
///
/// Flushes the connection.
pub fn init(conn: &xcb::Connection) -> xcb::Result<(i::Connection, Atoms)> {
    Ok((i::Connection::connect(conn), Atoms::init(conn)?))
}
