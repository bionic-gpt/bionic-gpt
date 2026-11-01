"""Adapt the pinned upstream web build to Bionic's opaque-origin, view-only iframe.

Every replacement is checked: incompatible upstream changes must fail the build.
"""
from pathlib import Path
import sys

root = Path(sys.argv[1]) / 'apps/cadcraft-web/src'

def replace(source, old, new):
    if source.count(old) != 1:
        raise RuntimeError(f'Upstream web bridge changed: expected one occurrence of {old!r}')
    return source.replace(old, new)

main = (root / 'main.rs').read_text()
main = replace(main,
    'let bridge = if query_param("host").as_deref() == Some("parent") { crate::host::Bridge::new() } else { None };',
    'let bridge = crate::host::Bridge::new();')
main = replace(main,
    'self.0.raw_input_hook(ctx, raw);',
    '''// Model-driven snapshot viewer: permit only pointer motion, wheel/pinch zoom and
            // middle-button panning. Menus, keyboard commands and file drops cannot edit it.
            raw.events.retain(|event| matches!(event,
                egui::Event::PointerMoved(_) | egui::Event::PointerGone |
                egui::Event::MouseWheel { .. } | egui::Event::Zoom(_) |
                egui::Event::PointerButton { button: egui::PointerButton::Middle, .. }
            ));
            raw.dropped_files.clear();
            raw.hovered_files.clear();
            self.0.raw_input_hook(ctx, raw);''')
main = replace(main,
    '    fn query_param(key: &str) -> Option<String> {\n        web_sys::UrlSearchParams::new_with_str(&query()).ok()?.get(key)\n    }\n\n',
    '')
(root / 'main.rs').write_text(main)

host = (root / 'host.rs').read_text()
# Only the loader in the iframe can inject drawings. It verifies parent messages first.
host = replace(host, 'let parent = self.parent.clone();', 'let parent = self.window.clone();')
host = replace(host,
    'if event.origin() != origin || !event.source().is_some_and(|s| js_sys::Object::is(&s, &parent)) {',
    'if !event.source().is_some_and(|s| js_sys::Object::is(&s, &parent)) {')
host = replace(host, 'let origin = self.origin.clone();', '')
host = replace(host,
    'message(fields).is_some_and(|m| self.parent.post_message(&m, &self.origin).is_ok())',
    'message(fields).is_some_and(|m| self.window.post_message(&m, "*").is_ok())')
(root / 'host.rs').write_text(host)
