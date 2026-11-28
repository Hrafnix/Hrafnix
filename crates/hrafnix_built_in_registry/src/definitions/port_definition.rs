use hrafnix_common::{port::PortKind, rotation::Rotation};
use hrafnix_keys::ConstPortKey;

/// Re-exports the [`port_key!`] macro for use in [`port_definition!`].
#[allow(unused_imports)]
pub(crate) use hrafnix_keys::port_key as __port_key;

/// Static placement metadata for a component port.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PortDefinition {
    /// Stable identifier for the port.
    id: ConstPortKey,
    /// Port name to be displayed.
    display_name: &'static str,
    /// Port kind.
    kind: PortKind,
    /// Horizontal position, normalized to the icon width.
    x: f32,
    /// Vertical position, normalized to the icon height.
    y: f32,
    /// Original placement in icon-local canvas units.
    canvas_position: (u8, u8),
    /// Port orientation.
    rotation: Rotation,
    /// Whether the port is required.
    required: bool,
}

impl PortDefinition {
    /// Backing constructor for [`port_definition!`].
    #[must_use]
    pub(crate) const fn __new(
        id: ConstPortKey,
        display_name: &'static str,
        kind: PortKind,
        canvas_position: (u8, u8),
        icon_dimensions: (u8, u8),
        rotation: Rotation,
        required: bool,
    ) -> Self {
        #[allow(clippy::as_conversions, clippy::float_arithmetic)]
        let x = canvas_position.0 as f32 / icon_dimensions.0 as f32;
        #[allow(clippy::as_conversions, clippy::float_arithmetic)]
        let y = canvas_position.1 as f32 / icon_dimensions.1 as f32;
        Self {
            id,
            display_name,
            kind,
            x,
            y,
            canvas_position,
            rotation,
            required,
        }
    }

    /// Returns the port identifier.
    #[must_use]
    pub const fn id(&self) -> ConstPortKey {
        self.id
    }

    /// Returns the port name.
    #[must_use]
    pub const fn display_name(&self) -> &'static str {
        self.display_name
    }

    /// Returns the port kind.
    #[must_use]
    pub const fn kind(&self) -> PortKind {
        self.kind
    }

    /// Returns the horizontal position of the port, normalized to the icon width.
    #[must_use]
    pub const fn x(&self) -> f32 {
        self.x
    }

    /// Returns the vertical position of the port, normalized to the icon height.
    #[must_use]
    pub const fn y(&self) -> f32 {
        self.y
    }

    /// Returns the original horizontal position in icon-local canvas units.
    #[must_use]
    pub const fn canvas_x(&self) -> u8 {
        self.canvas_position.0
    }

    /// Returns the original vertical position in icon-local canvas units.
    #[must_use]
    pub const fn canvas_y(&self) -> u8 {
        self.canvas_position.1
    }

    /// Returns the orientation of the port.
    #[must_use]
    pub const fn rotation(&self) -> Rotation {
        self.rotation
    }

    /// Returns whether this input port must be connected.
    #[must_use]
    pub const fn required(&self) -> bool {
        self.required
    }
}

/// Creates a [`PortDefinition`] and normalizes its icon-relative coordinates at
/// compile time.
///
/// Expansion is wrapped in a `const` block, so every argument must be a const-compatible
/// (`'static`) expression. The coordinates and icon dimensions are validated at compile
/// time even when the result is bound with a plain `let` instead of `const`.
///
/// # Syntax
/// ```text
/// port_definition!(id, name, kind, (x, y), (icon_width, icon_height), rotation, required)
/// ```
///
/// # Arguments
/// - `id`: `&'static str` used as the stable identifier for the port.
/// - `name`: `&'static str` name displayed for the port.
/// - `kind`: [`PortKind`] describing whether the port accepts or produces a signal.
/// - `(x, y)`: `u8` coordinates in canvas units, measured from the icon's top-left edge.
/// - `(icon_width, icon_height)`: `u8` dimensions used to normalize `x` and `y` to the inclusive range `0.0..=1.0`.
/// - `rotation`: [`Rotation`] indicating the direction in which the port faces.
/// - `required`: `bool` indicating whether the port is required.
///
/// The icon dimensions must be greater than zero, and each coordinate must not exceed
/// its corresponding dimension.
///
/// # Examples
/// ```text
/// const INPUT: PortDefinition = port_definition!(
///     "input",
///     "Input",
///     PortKind::SignalInput,
///     (0, 16),
///     (32, 32),
///     Rotation::Degrees0,
///     true
/// );
///
/// assert_eq!(INPUT.x(), 0.0);
/// assert_eq!(INPUT.y(), 0.5);
/// ```
macro_rules! port_definition {
    (
        $id:literal,
        $display_name:expr,
        $kind:expr,
        ($x:expr, $y:expr),
        ($length:expr, $width:expr),
        $rotation:expr,
        $required:literal $(,)?
    ) => {
        const {
            let x: u8 = $x;
            let y: u8 = $y;
            let length: u8 = $length;
            let width: u8 = $width;

            assert!(length > 10, "port length must be greater than 10");
            assert!(width > 10, "port width must be greater than 10");
            assert!(x <= length, "port x coordinate must not exceed its length");
            assert!(y <= width, "port y coordinate must not exceed its width");

            #[allow(clippy::disallowed_methods)]
            $crate::PortDefinition::__new(
                $crate::definitions::port_definition::__port_key!($id),
                $display_name,
                $kind,
                (x, y),
                (length, width),
                $rotation,
                $required,
            )
        }
    };
}
pub(crate) use port_definition;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macro_normalizes_coordinates() {
        const PORT: PortDefinition = port_definition!(
            "input",
            "Input",
            PortKind::SignalInput,
            (8, 24),
            (32, 48),
            Rotation::Degrees0,
            true
        );

        assert_eq!(PORT.x().to_bits(), 0.25_f32.to_bits());
        assert_eq!(PORT.y().to_bits(), 0.5_f32.to_bits());
        assert_eq!(PORT.canvas_x(), 8);
        assert_eq!(PORT.canvas_y(), 24);
    }
}
