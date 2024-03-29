//! Configure your application.
use crate::window;
use crate::{Antialiasing, Font, Pixels};

use std::borrow::Cow;

/// The settings of an iced [`Program`].
///
/// [`Program`]: crate::Program
#[derive(Debug, Clone)]
pub struct Settings<Flags = ()> {
    /// The identifier of the application.
    ///
    /// If provided, this identifier may be used to identify the application or
    /// communicate with it through the windowing system.
    pub id: Option<String>,

    /// The window settings.
    ///
    /// They will be ignored on the Web.
    pub window: window::Settings,

    /// The data needed to initialize the [`Program`].
    ///
    /// [`Program`]: crate::Program
    pub flags: Flags,

    /// The fonts to load on boot.
    pub fonts: Vec<Cow<'static, [u8]>>,

    /// The default [`Font`] to be used.
    ///
    /// By default, it uses [`Family::SansSerif`](crate::font::Family::SansSerif).
    pub default_font: Font,

    /// The text size that will be used by default.
    ///
    /// The default value is `16.0`.
    pub default_text_size: Pixels,

    /// The antialiasing strategy used for some primitives.
    ///
    /// Enabling it can produce a smoother result in some widgets, like the
    /// [`Canvas`], at a performance cost.
    ///
    /// By default, it is [`Antialiasing::Disabled`].
    ///
    /// [`Canvas`]: crate::widget::Canvas
    pub antialiasing: Antialiasing,
}

impl<Flags> Settings<Flags> {
    /// Initialize [`Program`] settings using the given data.
    ///
    /// [`Program`]: crate::Program
    pub fn with_flags(flags: Flags) -> Self {
        let default_settings = Settings::<()>::default();

        Self {
            flags,
            id: default_settings.id,
            window: default_settings.window,
            fonts: default_settings.fonts,
            default_font: default_settings.default_font,
            default_text_size: default_settings.default_text_size,
            antialiasing: default_settings.antialiasing,
        }
    }
}

impl<Flags> Default for Settings<Flags>
where
    Flags: Default,
{
    fn default() -> Self {
        Self {
            id: None,
            window: window::Settings::default(),
            flags: Default::default(),
            fonts: Vec::new(),
            default_font: Font::default(),
            default_text_size: Pixels(16.0),
            antialiasing: Antialiasing::default(),
        }
    }
}

impl<Flags> From<Settings<Flags>> for iced_winit::Settings<Flags> {
    fn from(settings: Settings<Flags>) -> iced_winit::Settings<Flags> {
        iced_winit::Settings {
            id: settings.id,
            window: settings.window,
            flags: settings.flags,
            fonts: settings.fonts,
        }
    }
}
