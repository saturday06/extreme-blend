// Copyright © 2008-2011 Kristian Høgsberg
// Copyright © 2010-2011 Intel Corporation
// Copyright © 2012-2013 Collabora, Ltd.
//
// Permission is hereby granted, free of charge, to any person
// obtaining a copy of this software and associated documentation files
// (the "Software"), to deal in the Software without restriction,
// including without limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of the Software,
// and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial
// portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT.  IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// drag and drop actions
//
// This is a bitmask of the available/preferred actions in a
// drag-and-drop operation.
//
// In the compositor, the selected action is a result of matching the
// actions offered by the source and destination sides.  "action" events
// with a "none" action will be sent to both source and destination if
// there is no match. All further checks will effectively happen on
// (source actions ∩ destination actions).
//
// In addition, compositors may also pick different actions in
// reaction to key modifiers being pressed. One common design that
// is used in major toolkits (and the behavior recommended for
// compositors) is:
//
// - If no modifiers are pressed, the first match (in bit order)
//   will be used.
// - Pressing Shift selects "move", if enabled in the mask.
// - Pressing Control selects "copy", if enabled in the mask.
//
// Behavior beyond that is considered implementation-dependent.
// Compositors may for example bind other modifiers (like Alt/Meta)
// or drags initiated with other buttons than BTN_LEFT to specific
// actions (e.g. "ask").
pub enum DndAction {
    None = 0, // no action
    Copy = 1, // copy action
    Move = 2, // move action
    Ask = 4,  // ask action
}
