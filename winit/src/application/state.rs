use crate::application;
use crate::conversion;
use crate::core::mouse;
use crate::core::{Color, Size};
use crate::graphics::{Target, Viewport};
use crate::runtime::Debug;
use crate::Application;

use winit::event::{Touch, WindowEvent};
use winit::window::Window;

/// The state of a windowed [`Application`].
#[allow(missing_debug_implementations)]
pub struct State<A: Application>
where
    A::Theme: application::DefaultStyle,
{
    title: String,
    scale_factor: f64,
    target: Target,
    viewport_version: usize,
    cursor_position: Option<winit::dpi::PhysicalPosition<f64>>,
    modifiers: winit::keyboard::ModifiersState,
    theme: A::Theme,
    appearance: application::Appearance,
}

impl<A: Application> State<A>
where
    A::Theme: application::DefaultStyle,
{
    /// Creates a new [`State`] for the provided [`Application`] and window.
    pub fn new(application: &A, window: &Window) -> Self {
        let title = application.title();
        let scale_factor = application.scale_factor();
        let theme = application.theme();
        let appearance = application.style(&theme);

        let window_scale_factor = window.scale_factor();

        let viewport = {
            let physical_size = window.inner_size();

            Viewport::with_physical_size(
                Size::new(physical_size.width, physical_size.height),
                window_scale_factor * scale_factor,
            )
        };

        let target = Target {
            scale_factor: window_scale_factor,
            viewport,
        };

        Self {
            title,
            scale_factor,
            target,
            viewport_version: 0,
            cursor_position: None,
            modifiers: winit::keyboard::ModifiersState::default(),
            theme,
            appearance,
        }
    }

    /// Returns the current [`Target`] of the [`State`] with its [`Viewport`].
    pub fn target(&self) -> &Target {
        &self.target
    }

    /// Returns the version of the [`Viewport`] of the [`State`].
    ///
    /// The version is incremented every time the [`Viewport`] changes.
    pub fn viewport_version(&self) -> usize {
        self.viewport_version
    }

    /// Returns the physical [`Size`] of the [`Viewport`] of the [`State`].
    pub fn physical_size(&self) -> Size<u32> {
        self.target.viewport.physical_size()
    }

    /// Returns the logical [`Size`] of the [`Viewport`] of the [`State`].
    pub fn logical_size(&self) -> Size<f32> {
        self.target.viewport.logical_size()
    }

    /// Returns the current scale factor of the [`Viewport`] of the [`State`].
    pub fn scale_factor(&self) -> f64 {
        self.target.viewport.scale_factor()
    }

    /// Returns the current cursor position of the [`State`].
    pub fn cursor(&self) -> mouse::Cursor {
        self.cursor_position
            .map(|cursor_position| {
                conversion::cursor_position(
                    cursor_position,
                    self.target.viewport.scale_factor(),
                )
            })
            .map(mouse::Cursor::Available)
            .unwrap_or(mouse::Cursor::Unavailable)
    }

    /// Returns the current keyboard modifiers of the [`State`].
    pub fn modifiers(&self) -> winit::keyboard::ModifiersState {
        self.modifiers
    }

    /// Returns the current theme of the [`State`].
    pub fn theme(&self) -> &A::Theme {
        &self.theme
    }

    /// Returns the current background [`Color`] of the [`State`].
    pub fn background_color(&self) -> Color {
        self.appearance.background_color
    }

    /// Returns the current text [`Color`] of the [`State`].
    pub fn text_color(&self) -> Color {
        self.appearance.text_color
    }

    /// Processes the provided window event and updates the [`State`]
    /// accordingly.
    pub fn update(
        &mut self,
        window: &Window,
        event: &WindowEvent,
        _debug: &mut Debug,
    ) {
        match event {
            WindowEvent::Resized(new_size) => {
                let size = Size::new(new_size.width, new_size.height);
                let new_scale_factor = window.scale_factor();

                self.target = Target {
                    scale_factor: new_scale_factor,
                    viewport: Viewport::with_physical_size(
                        size,
                        new_scale_factor * self.scale_factor,
                    ),
                };

                self.viewport_version = self.viewport_version.wrapping_add(1);
            }
            WindowEvent::ScaleFactorChanged {
                scale_factor: new_scale_factor,
                ..
            } => {
                let size = self.target.viewport.physical_size();

                self.target = Target {
                    scale_factor: *new_scale_factor,
                    viewport: Viewport::with_physical_size(
                        size,
                        new_scale_factor * self.scale_factor,
                    ),
                };

                self.viewport_version = self.viewport_version.wrapping_add(1);
            }
            WindowEvent::CursorMoved { position, .. }
            | WindowEvent::Touch(Touch {
                location: position, ..
            }) => {
                self.cursor_position = Some(*position);
            }
            WindowEvent::CursorLeft { .. } => {
                self.cursor_position = None;
            }
            WindowEvent::ModifiersChanged(new_modifiers) => {
                self.modifiers = new_modifiers.state();
            }
            #[cfg(feature = "debug")]
            WindowEvent::KeyboardInput {
                event:
                    winit::event::KeyEvent {
                        logical_key:
                            winit::keyboard::Key::Named(
                                winit::keyboard::NamedKey::F12,
                            ),
                        state: winit::event::ElementState::Pressed,
                        ..
                    },
                ..
            } => _debug.toggle(),
            _ => {}
        }
    }

    /// Synchronizes the [`State`] with its [`Application`] and its respective
    /// window.
    ///
    /// Normally an [`Application`] should be synchronized with its [`State`]
    /// and window after calling [`crate::application::update`].
    pub fn synchronize(&mut self, application: &A, window: &Window) {
        // Update window title
        let new_title = application.title();

        if self.title != new_title {
            window.set_title(&new_title);

            self.title = new_title;
        }

        // Update scale factor and size
        let new_scale_factor = application.scale_factor();
        let new_size = window.inner_size();
        let current_size = self.target.viewport.physical_size();

        if self.scale_factor != new_scale_factor
            || (current_size.width, current_size.height)
                != (new_size.width, new_size.height)
        {
            self.target.viewport = Viewport::with_physical_size(
                Size::new(new_size.width, new_size.height),
                self.target.scale_factor * new_scale_factor,
            );
            self.viewport_version = self.viewport_version.wrapping_add(1);

            self.scale_factor = new_scale_factor;
        }

        // Update theme and appearance
        self.theme = application.theme();
        self.appearance = application.style(&self.theme);
    }
}
