# macwindow

NoteFeratu implements a custom window titlebar by configuring `titleBarStyle: Overlay` and `hiddenTitle: true` (see: [Tauri Configuration Files](https://v2.tauri.app/develop/configuration-files/)). This removes the OS's default titlebar and allows components within the webview to take its place.

The advantage of this approach is that the MacOS window controls, window appearance effects (shadow, fade when out of focus, etc.), and window border radius are preserved. All that's left is to vertically center the window controls in the custom title bar (by default, they float slightly above and to the left of the center).

The **macwindow** plugin does exactly that: it listens to Tauri's new window event and then spawns a "positioner" that dynamically updates the position of the MacOS window controls. When the window is resized and the controls fall out of place, the positioner automatically repositions them.

This plugin is derived from [Yaak's source code](https://github.com/mountain-loop/yaak/tree/aadfbfdfca5759b9c4722cc5d7715c9833b0794b/src-tauri/yaak-mac-window), which is distributed under the MIT open source license.
