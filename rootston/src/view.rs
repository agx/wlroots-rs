pub struct RootsView {
    view: View
}

pub enum View {
    // TODO Fill in
    /// Wayland shell surface
    WLShell,
    /// XDG shell v6 surface
    XDG,
    /// XWayland shell surface
    XWayland
}
