//! Application trait & lifecycle.
//!
//! [`Application`] wraps GPUI's [`gpui::Application`] with the Yaru theme and
//! libadwaita-style window management. The platform backend (X11/Wayland on
//! Linux, WebGL2 canvas on WebAssembly) is provided by `gpui_platform`.
use anyhow::Result;
use gpui::{App, Bounds, Context, Global, Render, WindowBounds, WindowOptions, px, size};
use libgalaxy_theme::Theme;

use crate::theme;

/// The GPUI `App` context, re-exported for convenience.
pub use gpui::App as AppContext;

/// The platform application, re-exported so apps can call
/// [`platform::application`](gpui_platform::application) when they need
/// direct access to GPUI.
pub use gpui_platform as platform;

/// A GPUI window handle, re-exported for convenience.
pub use gpui::WindowHandle;

/// The application identifier stored as a global, mirroring `GApplication`'s
/// application ID concept (`org.example.App`).
pub struct ApplicationId(pub String);

impl Global for ApplicationId {}

/// The LibGalaxy application.
///
/// ```rust,ignore
/// libgalaxy::Application::new("org.example.MyApp")
///     .set_theme(libgalaxy::Theme::yaru_light())
///     .run::<MyWindow>();
/// ```
pub struct Application {
    inner: gpui::Application,
    app_id: String,
    theme: Option<Theme>,
    window_options: WindowOptions,
}

impl Application {
    /// Create a new application with the given application identifier.
    ///
    /// `app_id` follows the D-Bus style reverse-DNS convention
    /// (e.g. `"org.gnome.Adwaita1.Demo"`).
    pub fn new(app_id: &str) -> Self {
        let inner = platform::application();
        Self {
            inner,
            app_id: app_id.to_string(),
            theme: None,
            window_options: default_window_options(),
        }
    }

    /// Provide an asset source for loading fonts, icons and other resources.
    pub fn with_assets(mut self, source: impl gpui::AssetSource + 'static) -> Self {
        self.inner = self.inner.with_assets(source);
        self
    }

    /// Set the initial theme applied to every window.
    pub fn set_theme(mut self, theme: Theme) -> Self {
        self.theme = Some(theme);
        self
    }

    /// Override the window options used to open the main window.
    pub fn with_window_options(mut self, options: WindowOptions) -> Self {
        self.window_options = options;
        self
    }

    /// Run the application, opening a single main window whose root view is
    /// built from `Window::new`.
    pub fn run<W>(self) -> Result<()>
    where
        W: ApplicationWindow,
    {
        let theme = self.theme;
        let mut options = self.window_options;
        self.run_with(move |cx| {
            if let Some(theme) = theme {
                theme::set_theme(cx, theme);
            }
            if options.window_bounds.is_none() {
                options.window_bounds = Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(1000.), px(700.)),
                    cx,
                )));
            }
            let _handle = cx.open_window(options, |window, cx| {
                let view = cx.new(|cx| W::new(cx));
                window.set_window_title(W::default_title());
                view
            })?;
            cx.activate(true);
            Ok(())
        })
    }

    /// Run the application with full control over the setup closure.
    ///
    /// The closure is invoked once GPUI is ready and receives `&mut App`.
    /// Open windows and install globals from here.
    pub fn run_with<F>(self, on_launch: F) -> Result<()>
    where
        F: FnOnce(&mut App) -> Result<()> + 'static,
    {
        self.inner.run(|cx: &mut App| {
            install_application_id(cx, &self.app_id);
            if let Err(error) = on_launch(cx) {
                log::error!("application launch failed: {error:#}");
                cx.quit();
            }
        });
        Ok(())
    }

    /// Access the wrapped GPUI application.
    pub fn inner(&self) -> &gpui::Application {
        &self.inner
    }
}

/// A window whose root view is managed by LibGalaxy.
///
/// Mirror of `AdwApplicationWindow` / `GtkApplicationWindow`: a single content
/// view, a title, and (optionally) a header bar.
pub trait ApplicationWindow: 'static + Render {
    /// Build the window's root view.
    fn new(cx: &mut Context<Self>) -> Self;

    /// The window title shown in the title bar.
    fn default_title() -> &'static str {
        "LibGalaxy Window"
    }
}

/// Default window options for LibGalaxy windows: a 1000x700 resizable window
/// with a standard title bar.
pub fn default_window_options() -> WindowOptions {
    WindowOptions {
        window_bounds: None,
        focus: true,
        show: true,
        is_resizable: true,
        ..Default::default()
    }
}

// ---------------------------------------------------------------------------
// Application-global helpers
// ---------------------------------------------------------------------------

/// The application identifier set via [`Application::new`].
pub fn application_id(cx: &App) -> Option<&str> {
    cx.try_global::<ApplicationId>().map(|g| g.0.as_str())
}

/// Register the application identifier global (called by [`Application::run`]).
pub(crate) fn install_application_id(cx: &mut App, app_id: &str) {
    cx.set_global(ApplicationId(app_id.to_string()));
}
