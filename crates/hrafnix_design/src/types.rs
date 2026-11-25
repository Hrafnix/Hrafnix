//! Editable design value types decoupled from `hrafnix_files` on-disk snapshots.
//!
//! Each type converts to and from its corresponding `hrafnix_files` representation
//! at the file I/O boundary rather than sharing file-snapshot structs directly.

use camino::{Utf8Path, Utf8PathBuf};
use hrafnix_common::port::PortType;
use hrafnix_files::builder;
use hrafnix_files::data::connection as file_connection;
use hrafnix_files::data::decoration_background as file_bg;
use hrafnix_files::data::decoration_foreground as file_fg;
use hrafnix_files::data::graphics as file_graphics;
use hrafnix_files::data::metadata_general as file_metadata;
use hrafnix_files::data::metadata_gui as file_gui;
use hrafnix_files::data::source as file_source;
use hrafnix_files::data::translations as file_translations;
use hrafnix_keys::{ComponentKey, PortKey};
use hrafnix_shareable_string::{ShareableString, SharedStringStore};
use std::collections::BTreeMap;

/// A position in signed 32-bit drawing coordinates.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Position {
    /// Horizontal position.
    x: i32,
    /// Vertical position.
    y: i32,
}

impl Position {
    /// Creates a new position.
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Returns the horizontal coordinate.
    #[must_use]
    pub const fn x(self) -> i32 {
        self.x
    }

    /// Returns the vertical coordinate.
    #[must_use]
    pub const fn y(self) -> i32 {
        self.y
    }

    /// Sets the horizontal coordinate.
    pub const fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    /// Sets the vertical coordinate.
    pub const fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    /// Converts from a file snapshot position.
    #[must_use]
    pub(crate) const fn from_file(pos: file_graphics::Position) -> Self {
        Self {
            x: pos.x(),
            y: pos.y(),
        }
    }

    /// Converts into a file snapshot position.
    #[must_use]
    pub(crate) const fn to_file(self) -> file_graphics::Position {
        builder::position(self.x, self.y)
    }
}

/// Dimensions in unsigned 16-bit drawing coordinates.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Dimension {
    /// Width in drawing coordinates.
    width: u16,
    /// Height in drawing coordinates.
    height: u16,
}

impl Dimension {
    /// Creates new dimensions.
    #[must_use]
    pub const fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    /// Returns the width.
    #[must_use]
    pub const fn width(self) -> u16 {
        self.width
    }

    /// Returns the height.
    #[must_use]
    pub const fn height(self) -> u16 {
        self.height
    }

    /// Sets the width.
    pub const fn set_width(&mut self, width: u16) {
        self.width = width;
    }

    /// Sets the height.
    pub const fn set_height(&mut self, height: u16) {
        self.height = height;
    }

    /// Converts from a file snapshot dimension.
    #[must_use]
    pub(crate) const fn from_file(dim: file_graphics::Dimension) -> Self {
        Self {
            width: dim.width(),
            height: dim.height(),
        }
    }

    /// Converts into a file snapshot dimension.
    #[must_use]
    pub(crate) const fn to_file(self) -> file_graphics::Dimension {
        builder::dimension(self.width, self.height)
    }
}

/// Clockwise quarter-turn rotation about the center of unrotated drawing bounds.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum Rotation {
    /// No rotation.
    #[default]
    Degrees0 = 0,
    /// 90 degrees clockwise.
    Degrees90 = 90,
    /// 180 degrees clockwise.
    Degrees180 = 180,
    /// 270 degrees clockwise.
    Degrees270 = 270,
}

impl Rotation {
    /// Returns the rotation in degrees.
    #[must_use]
    pub const fn degrees(self) -> u16 {
        match self {
            Self::Degrees0 => 0,
            Self::Degrees90 => 90,
            Self::Degrees180 => 180,
            Self::Degrees270 => 270,
        }
    }

    /// Converts from a file snapshot rotation.
    #[must_use]
    pub(crate) const fn from_file(rotation: file_graphics::Rotation) -> Self {
        match rotation {
            file_graphics::Rotation::Degrees0 => Self::Degrees0,
            file_graphics::Rotation::Degrees90 => Self::Degrees90,
            file_graphics::Rotation::Degrees180 => Self::Degrees180,
            file_graphics::Rotation::Degrees270 => Self::Degrees270,
        }
    }

    /// Converts into a file snapshot rotation.
    #[must_use]
    pub(crate) const fn to_file(self) -> file_graphics::Rotation {
        match self {
            Self::Degrees0 => file_graphics::Rotation::Degrees0,
            Self::Degrees90 => file_graphics::Rotation::Degrees90,
            Self::Degrees180 => file_graphics::Rotation::Degrees180,
            Self::Degrees270 => file_graphics::Rotation::Degrees270,
        }
    }
}

/// Font size in hundredths of a drawing unit (`1234` represents `12.34`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct FontSize {
    /// Size in hundredths.
    size: u16,
}

impl FontSize {
    /// Creates a new font size in hundredths of a drawing unit.
    #[must_use]
    pub const fn new(size: u16) -> Self {
        Self { size }
    }

    /// Returns the raw size in hundredths.
    #[must_use]
    pub const fn size(self) -> u16 {
        self.size
    }

    /// Sets the raw size in hundredths.
    pub const fn set_size(&mut self, size: u16) {
        self.size = size;
    }

    /// Converts from a file snapshot font size.
    #[must_use]
    pub(crate) const fn from_file(font_size: file_graphics::FontSize) -> Self {
        Self {
            size: font_size.size(),
        }
    }

    /// Converts into a file snapshot font size.
    #[must_use]
    pub(crate) const fn to_file(self) -> file_graphics::FontSize {
        builder::font_size(self.size)
    }
}

/// Editable descriptive metadata for a model or user component.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Metadata {
    /// Name shown to users.
    display_name: ShareableString,
    /// Free-form description.
    description: ShareableString,
    /// Author of the design object.
    author: ShareableString,
    /// Major part of the minimum Hrafnix version required.
    minimum_version_major: u16,
    /// Minor part of the minimum Hrafnix version required.
    minimum_version_minor: u16,
}

impl Metadata {
    /// Creates new metadata.
    #[must_use]
    pub fn new(
        display_name: impl Into<ShareableString>,
        description: impl Into<ShareableString>,
        author: impl Into<ShareableString>,
        minimum_version_major: u16,
        minimum_version_minor: u16,
    ) -> Self {
        Self {
            display_name: display_name.into(),
            description: description.into(),
            author: author.into(),
            minimum_version_major,
            minimum_version_minor,
        }
    }

    /// Returns the display name.
    #[must_use]
    pub const fn display_name(&self) -> &ShareableString {
        &self.display_name
    }

    /// Sets the display name.
    pub fn set_display_name(&mut self, display_name: impl Into<ShareableString>) {
        self.display_name = display_name.into();
    }

    /// Returns the description.
    #[must_use]
    pub const fn description(&self) -> &ShareableString {
        &self.description
    }

    /// Sets the description.
    pub fn set_description(&mut self, description: impl Into<ShareableString>) {
        self.description = description.into();
    }

    /// Returns the author.
    #[must_use]
    pub const fn author(&self) -> &ShareableString {
        &self.author
    }

    /// Sets the author.
    pub fn set_author(&mut self, author: impl Into<ShareableString>) {
        self.author = author.into();
    }

    /// Returns the minimum Hrafnix major version.
    #[must_use]
    pub const fn minimum_version_major(&self) -> u16 {
        self.minimum_version_major
    }

    /// Sets the minimum Hrafnix major version.
    pub const fn set_minimum_version_major(&mut self, major: u16) {
        self.minimum_version_major = major;
    }

    /// Returns the minimum Hrafnix minor version.
    #[must_use]
    pub const fn minimum_version_minor(&self) -> u16 {
        self.minimum_version_minor
    }

    /// Sets the minimum Hrafnix minor version.
    pub const fn set_minimum_version_minor(&mut self, minor: u16) {
        self.minimum_version_minor = minor;
    }

    /// Converts from a file metadata snapshot.
    #[must_use]
    pub(crate) fn from_file(meta: &file_metadata::MetadataFile) -> Self {
        Self {
            display_name: meta.display_name().clone(),
            description: meta.description().clone(),
            author: meta.author().clone(),
            minimum_version_major: meta.minimum_version_major(),
            minimum_version_minor: meta.minimum_version_minor(),
        }
    }

    /// Converts into a file metadata snapshot.
    #[must_use]
    pub(crate) fn to_file(&self) -> file_metadata::MetadataFile {
        builder::MetadataFileBuilder::new(
            self.display_name.clone(),
            self.description.clone(),
            self.author.clone(),
            self.minimum_version_major,
            self.minimum_version_minor,
        )
        .build()
    }

    /// Interns this value's strings in `store` in place.
    pub fn launder(&mut self, store: &SharedStringStore) {
        self.display_name = store.launder(&self.display_name);
        self.description = store.launder(&self.description);
        self.author = store.launder(&self.author);
    }
}

/// Editable translation table for a model or user component.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Translations {
    /// Source language of the original text keys.
    language: ShareableString,
    /// Original text mapped to target language codes and translated strings.
    translations: BTreeMap<ShareableString, BTreeMap<ShareableString, ShareableString>>,
}

impl Translations {
    /// Creates an empty translation table for the given source language.
    #[must_use]
    pub fn new(language: impl Into<ShareableString>) -> Self {
        Self {
            language: language.into(),
            translations: BTreeMap::new(),
        }
    }

    /// Returns the source language.
    #[must_use]
    pub const fn language(&self) -> &ShareableString {
        &self.language
    }

    /// Sets the source language.
    pub fn set_language(&mut self, language: impl Into<ShareableString>) {
        self.language = language.into();
    }

    /// Returns the translation map.
    #[must_use]
    pub const fn translations(
        &self,
    ) -> &BTreeMap<ShareableString, BTreeMap<ShareableString, ShareableString>> {
        &self.translations
    }

    /// Returns a mutable reference to the translation map.
    pub const fn translations_mut(
        &mut self,
    ) -> &mut BTreeMap<ShareableString, BTreeMap<ShareableString, ShareableString>> {
        &mut self.translations
    }

    /// Converts from a file translations snapshot.
    #[must_use]
    pub(crate) fn from_file(translations: &file_translations::TranslationsFile) -> Self {
        Self {
            language: translations.language().clone(),
            translations: translations.translations().clone(),
        }
    }

    /// Converts into a file translations snapshot.
    #[must_use]
    pub(crate) fn to_file(&self) -> file_translations::TranslationsFile {
        builder::TranslationsFileBuilder::new(self.language.clone())
            .translations(self.translations.clone())
            .build()
    }

    /// Interns this value's strings in `store` in place.
    pub fn launder(&mut self, store: &SharedStringStore) {
        self.language = store.launder(&self.language);
        self.translations = std::mem::take(&mut self.translations)
            .into_iter()
            .map(|(text, translations)| {
                (
                    store.launder(&text),
                    translations
                        .into_iter()
                        .map(|(language, translation)| {
                            (store.launder(&language), store.launder(&translation))
                        })
                        .collect(),
                )
            })
            .collect();
    }
}

/// A nonnegative location in icon-local drawing coordinates.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct PortLocation {
    /// Horizontal coordinate.
    x: u16,
    /// Vertical coordinate.
    y: u16,
}

impl PortLocation {
    /// Creates a new port location.
    #[must_use]
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    /// Returns the horizontal coordinate.
    #[must_use]
    pub const fn x(self) -> u16 {
        self.x
    }

    /// Returns the vertical coordinate.
    #[must_use]
    pub const fn y(self) -> u16 {
        self.y
    }

    /// Converts from a file port location.
    #[must_use]
    pub(crate) const fn from_file(loc: file_gui::PortLocation) -> Self {
        Self {
            x: loc.x(),
            y: loc.y(),
        }
    }

    /// Converts into a file port location.
    #[must_use]
    pub(crate) const fn to_file(self) -> file_gui::PortLocation {
        builder::port_location(self.x, self.y)
    }
}

/// A port placed on a component's icon.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::struct_field_names)]
pub struct Port {
    /// Validated identifier of the port.
    id: PortKey,
    /// Role of the connection.
    port_type: PortType,
    /// Location within the icon's drawing dimensions.
    location: PortLocation,
}

impl Port {
    /// Creates a new port.
    #[must_use]
    pub const fn new(id: PortKey, port_type: PortType, location: PortLocation) -> Self {
        Self {
            id,
            port_type,
            location,
        }
    }

    /// Returns the port identifier.
    #[must_use]
    pub const fn id(&self) -> &PortKey {
        &self.id
    }

    /// Returns the port type.
    #[must_use]
    pub const fn port_type(&self) -> PortType {
        self.port_type
    }

    /// Returns the icon-local port location.
    #[must_use]
    pub const fn location(&self) -> PortLocation {
        self.location
    }

    /// Converts from a file port snapshot.
    #[must_use]
    pub(crate) fn from_file(port: &file_gui::Port) -> Self {
        Self {
            id: port.id().clone(),
            port_type: port.port_type(),
            location: PortLocation::from_file(port.location()),
        }
    }

    /// Converts into a file port snapshot.
    #[must_use]
    pub(crate) fn to_file(&self) -> file_gui::Port {
        builder::port(self.id.clone(), self.port_type, self.location.to_file())
    }

    /// Interns this value's strings in `store` in place.
    pub fn launder(&mut self, store: &SharedStringStore) {
        self.id.launder(store);
    }
}

/// Editable GUI metadata for a component or placed component instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuiMetadata {
    /// Unrotated dimensions in drawing coordinates.
    dimensions: Dimension,
    /// Clockwise quarter-turn rotation about the center of the icon bounds.
    rotation: Rotation,
    /// Icon path.
    icon: Utf8PathBuf,
    /// Ports placed within the icon's drawing dimensions.
    ports: Vec<Port>,
}

impl GuiMetadata {
    /// Creates new GUI metadata.
    #[must_use]
    pub const fn new(
        dimensions: Dimension,
        rotation: Rotation,
        icon: Utf8PathBuf,
        ports: Vec<Port>,
    ) -> Self {
        Self {
            dimensions,
            rotation,
            icon,
            ports,
        }
    }

    /// Returns the unrotated dimensions.
    #[must_use]
    pub const fn dimensions(&self) -> Dimension {
        self.dimensions
    }

    /// Sets the unrotated dimensions.
    pub const fn set_dimensions(&mut self, dimensions: Dimension) {
        self.dimensions = dimensions;
    }

    /// Returns the clockwise rotation.
    #[must_use]
    pub const fn rotation(&self) -> Rotation {
        self.rotation
    }

    /// Sets the clockwise rotation.
    pub const fn set_rotation(&mut self, rotation: Rotation) {
        self.rotation = rotation;
    }

    /// Returns the icon path.
    #[must_use]
    pub fn icon(&self) -> &Utf8Path {
        self.icon.as_path()
    }

    /// Sets the icon path.
    pub fn set_icon(&mut self, icon: Utf8PathBuf) {
        self.icon = icon;
    }

    /// Returns the ports.
    #[must_use]
    pub fn ports(&self) -> &[Port] {
        &self.ports
    }

    /// Returns a mutable reference to the ports list.
    pub const fn ports_mut(&mut self) -> &mut Vec<Port> {
        &mut self.ports
    }

    /// Converts from a file GUI metadata snapshot.
    #[must_use]
    pub(crate) fn from_file(gui: &file_gui::GuiMetadata) -> Self {
        Self {
            dimensions: Dimension::from_file(gui.dimensions()),
            rotation: Rotation::from_file(gui.rotation()),
            icon: gui.icon().to_path_buf(),
            ports: gui.ports().iter().map(Port::from_file).collect(),
        }
    }

    /// Converts into a file GUI metadata snapshot.
    #[must_use]
    pub(crate) fn to_file(&self) -> file_gui::GuiMetadata {
        builder::GuiMetadataBuilder::new(self.dimensions.to_file(), self.icon.clone())
            .rotation(self.rotation.to_file())
            .ports(self.ports.iter().map(Port::to_file).collect())
            .build()
    }

    /// Interns this value's strings in `store` in place.
    pub fn launder(&mut self, store: &SharedStringStore) {
        for port in &mut self.ports {
            port.launder(store);
        }
    }
}

/// Semantic package version with major, minor, and patch numbers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PackageVersion {
    /// Major version component.
    major: u16,
    /// Minor version component.
    minor: u16,
    /// Patch version component.
    patch: u16,
}

impl PackageVersion {
    /// Creates a new semantic package version.
    #[must_use]
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the major version component.
    #[must_use]
    pub const fn major(self) -> u16 {
        self.major
    }

    /// Returns the minor version component.
    #[must_use]
    pub const fn minor(self) -> u16 {
        self.minor
    }

    /// Returns the patch version component.
    #[must_use]
    pub const fn patch(self) -> u16 {
        self.patch
    }

    /// Converts from a file package version.
    #[must_use]
    pub(crate) const fn from_file(version: file_source::PackageVersion) -> Self {
        Self {
            major: version.major(),
            minor: version.minor(),
            patch: version.patch(),
        }
    }

    /// Converts into a file package version.
    #[must_use]
    pub(crate) const fn to_file(self) -> file_source::PackageVersion {
        file_source::PackageVersion::new(self.major, self.minor, self.patch)
    }
}

/// Unresolved reference to a specific package version.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PackageReference {
    /// Package name.
    name: ShareableString,
    /// Package version.
    version: PackageVersion,
}

impl PackageReference {
    /// Creates a new package reference.
    #[must_use]
    pub fn new(name: impl Into<ShareableString>, version: PackageVersion) -> Self {
        Self {
            name: name.into(),
            version,
        }
    }

    /// Returns the package name.
    #[must_use]
    pub const fn name(&self) -> &ShareableString {
        &self.name
    }

    /// Returns the package version.
    #[must_use]
    pub const fn version(&self) -> PackageVersion {
        self.version
    }

    /// Converts from a file package reference.
    #[must_use]
    pub(crate) fn from_file(package: &file_source::PackageReference) -> Self {
        Self {
            name: package.name().clone(),
            version: PackageVersion::from_file(package.version()),
        }
    }

    /// Converts into a file package reference.
    #[must_use]
    pub(crate) fn to_file(&self) -> file_source::PackageReference {
        file_source::PackageReference::new(self.name.clone(), self.version.to_file())
    }

    /// Interns this value's strings in `store` in place.
    pub fn launder(&mut self, store: &SharedStringStore) {
        self.name = store.launder(&self.name);
    }
}

/// Provenance of a component definition or instance.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ComponentSource {
    /// Component stored in a `.hxc` file.
    File {
        /// Optional package provenance.
        package: Option<PackageReference>,
        /// Stored file path.
        path: Utf8PathBuf,
    },
    /// Component supplied by the built-in registry.
    Builtin {
        /// Built-in component name.
        name: ShareableString,
        /// Built-in component version.
        version: u32,
    },
}

impl ComponentSource {
    /// Converts from a file component source.
    #[must_use]
    pub(crate) fn from_file(source: &file_source::ComponentSource) -> Self {
        match source {
            file_source::ComponentSource::File { package, path } => Self::File {
                package: package.as_ref().map(PackageReference::from_file),
                path: path.clone(),
            },
            file_source::ComponentSource::Builtin { name, version } => Self::Builtin {
                name: name.clone(),
                version: *version,
            },
        }
    }

    /// Converts into a file component source.
    #[must_use]
    pub(crate) fn to_file(&self) -> file_source::ComponentSource {
        match self {
            Self::File { package, path } => file_source::ComponentSource::File {
                package: package.as_ref().map(PackageReference::to_file),
                path: path.clone(),
            },
            Self::Builtin { name, version } => file_source::ComponentSource::Builtin {
                name: name.clone(),
                version: *version,
            },
        }
    }

    /// Interns this value's strings in `store` in place.
    pub fn launder(&mut self, store: &SharedStringStore) {
        match self {
            Self::File { package, .. } => {
                if let Some(pkg) = package {
                    pkg.launder(store);
                }
            }
            Self::Builtin { name, .. } => {
                *name = store.launder(&*name);
            }
        }
    }
}

/// Intermediate points along a connection's drawing path.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ConnectionLine {
    /// Ordered midpoints in drawing coordinates.
    midpoints: Vec<Position>,
}

impl ConnectionLine {
    /// Creates a new connection line.
    #[must_use]
    pub const fn new(midpoints: Vec<Position>) -> Self {
        Self { midpoints }
    }

    /// Returns the ordered midpoints.
    #[must_use]
    pub fn midpoints(&self) -> &[Position] {
        &self.midpoints
    }

    /// Returns a mutable reference to the ordered midpoints.
    pub const fn midpoints_mut(&mut self) -> &mut Vec<Position> {
        &mut self.midpoints
    }

    /// Converts from a file connection line.
    #[must_use]
    pub(crate) fn from_file(line: &file_connection::ConnectionLine) -> Self {
        Self {
            midpoints: line
                .midpoints()
                .iter()
                .copied()
                .map(Position::from_file)
                .collect(),
        }
    }

    /// Converts into a file connection line.
    #[must_use]
    pub(crate) fn to_file(&self) -> file_connection::ConnectionLine {
        builder::connection_line(
            self.midpoints
                .iter()
                .copied()
                .map(Position::to_file)
                .collect(),
        )
    }
}

/// Directed connection between two component ports.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Connection {
    /// Source component identifier.
    from_component: ComponentKey,
    /// Source port identifier.
    from_port: PortKey,
    /// Destination component identifier.
    to_component: ComponentKey,
    /// Destination port identifier.
    to_port: PortKey,
    /// Drawing path between the connected ports.
    line: ConnectionLine,
}

impl Connection {
    /// Creates a new directed connection.
    #[must_use]
    pub const fn new(
        from_component: ComponentKey,
        from_port: PortKey,
        to_component: ComponentKey,
        to_port: PortKey,
        line: ConnectionLine,
    ) -> Self {
        Self {
            from_component,
            from_port,
            to_component,
            to_port,
            line,
        }
    }

    /// Returns the source component identifier.
    #[must_use]
    pub const fn from_component(&self) -> &ComponentKey {
        &self.from_component
    }

    /// Returns the source port identifier.
    #[must_use]
    pub const fn from_port(&self) -> &PortKey {
        &self.from_port
    }

    /// Returns the destination component identifier.
    #[must_use]
    pub const fn to_component(&self) -> &ComponentKey {
        &self.to_component
    }

    /// Returns the destination port identifier.
    #[must_use]
    pub const fn to_port(&self) -> &PortKey {
        &self.to_port
    }

    /// Returns the drawing path.
    #[must_use]
    pub const fn line(&self) -> &ConnectionLine {
        &self.line
    }

    /// Returns a mutable reference to the drawing path.
    pub const fn line_mut(&mut self) -> &mut ConnectionLine {
        &mut self.line
    }

    /// Converts from a file connection snapshot.
    #[must_use]
    pub(crate) fn from_file(conn: &file_connection::Connection) -> Self {
        Self {
            from_component: conn.from_component().clone(),
            from_port: conn.from_port().clone(),
            to_component: conn.to_component().clone(),
            to_port: conn.to_port().clone(),
            line: ConnectionLine::from_file(conn.line()),
        }
    }

    /// Converts into a file connection snapshot.
    #[must_use]
    pub(crate) fn to_file(&self) -> file_connection::Connection {
        builder::connection(
            self.from_component.clone(),
            self.from_port.clone(),
            self.to_component.clone(),
            self.to_port.clone(),
            self.line.to_file(),
        )
    }

    /// Interns this value's strings in `store` in place.
    pub fn launder(&mut self, store: &SharedStringStore) {
        self.from_component.launder(store);
        self.from_port.launder(store);
        self.to_component.launder(store);
        self.to_port.launder(store);
    }
}

/// Shapes allowed in a background layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackgroundDecorationKind {
    /// A rectangular outline.
    Frame,
}

impl BackgroundDecorationKind {
    /// Converts from a file background decoration kind.
    #[must_use]
    pub(crate) const fn from_file(kind: &file_bg::BackgroundDecorationKind) -> Self {
        match kind {
            file_bg::BackgroundDecorationKind::Frame => Self::Frame,
        }
    }

    /// Converts into a file background decoration kind.
    #[must_use]
    pub(crate) const fn to_file(&self) -> file_bg::BackgroundDecorationKind {
        match self {
            Self::Frame => file_bg::BackgroundDecorationKind::Frame,
        }
    }
}

/// Editable shape drawn behind a model's or component's contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackgroundDecoration {
    /// Decoration identifier.
    id: ShareableString,
    /// Position in drawing coordinates.
    position: Position,
    /// Dimensions in drawing coordinates.
    dimensions: Dimension,
    /// Decoration shape.
    kind: BackgroundDecorationKind,
}

impl BackgroundDecoration {
    /// Creates a new background decoration.
    #[must_use]
    pub fn new(
        id: impl Into<ShareableString>,
        position: Position,
        dimensions: Dimension,
        kind: BackgroundDecorationKind,
    ) -> Self {
        Self {
            id: id.into(),
            position,
            dimensions,
            kind,
        }
    }

    /// Returns the decoration identifier.
    #[must_use]
    pub const fn id(&self) -> &ShareableString {
        &self.id
    }

    /// Returns the position.
    #[must_use]
    pub const fn position(&self) -> Position {
        self.position
    }

    /// Returns the dimensions.
    #[must_use]
    pub const fn dimensions(&self) -> Dimension {
        self.dimensions
    }

    /// Returns the decoration shape.
    #[must_use]
    pub const fn kind(&self) -> &BackgroundDecorationKind {
        &self.kind
    }

    /// Converts from a file background decoration snapshot.
    #[must_use]
    pub(crate) fn from_file(dec: &file_bg::BackgroundDecoration) -> Self {
        Self {
            id: dec.id().clone(),
            position: Position::from_file(dec.position()),
            dimensions: Dimension::from_file(dec.dimensions()),
            kind: BackgroundDecorationKind::from_file(dec.kind()),
        }
    }

    /// Converts into a file background decoration snapshot.
    #[must_use]
    pub(crate) fn to_file(&self) -> file_bg::BackgroundDecoration {
        builder::background_decoration(
            self.id.clone(),
            self.position.to_file(),
            self.dimensions.to_file(),
            self.kind.to_file(),
        )
    }

    /// Interns this value's strings in `store` in place.
    pub fn launder(&mut self, store: &SharedStringStore) {
        self.id = store.launder(&self.id);
    }
}

/// How an image fits within its decoration bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFit {
    /// Preserve aspect ratio and show the whole image within bounds.
    Contain,
    /// Preserve aspect ratio and cover the bounds, cropping if necessary.
    Cover,
    /// Stretch the image to fill the bounds.
    Fill,
}

impl ImageFit {
    /// Converts from a file image fit mode.
    #[must_use]
    pub(crate) const fn from_file(fit: file_fg::ImageFit) -> Self {
        match fit {
            file_fg::ImageFit::Contain => Self::Contain,
            file_fg::ImageFit::Cover => Self::Cover,
            file_fg::ImageFit::Fill => Self::Fill,
        }
    }

    /// Converts into a file image fit mode.
    #[must_use]
    pub(crate) const fn to_file(self) -> file_fg::ImageFit {
        match self {
            Self::Contain => file_fg::ImageFit::Contain,
            Self::Cover => file_fg::ImageFit::Cover,
            Self::Fill => file_fg::ImageFit::Fill,
        }
    }
}

/// Editable text content for a foreground decoration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextDecoration {
    /// Displayed text.
    text: ShareableString,
    /// Font size in hundredths of a drawing unit.
    font_size: FontSize,
}

impl TextDecoration {
    /// Creates new text decoration content.
    #[must_use]
    pub fn new(text: impl Into<ShareableString>, font_size: FontSize) -> Self {
        Self {
            text: text.into(),
            font_size,
        }
    }

    /// Returns the displayed text.
    #[must_use]
    pub const fn text(&self) -> &ShareableString {
        &self.text
    }

    /// Returns the font size.
    #[must_use]
    pub const fn font_size(&self) -> FontSize {
        self.font_size
    }

    /// Converts from a file text decoration.
    #[must_use]
    pub(crate) fn from_file(dec: &file_fg::TextDecoration) -> Self {
        Self {
            text: dec.text().clone(),
            font_size: FontSize::from_file(dec.font_size()),
        }
    }

    /// Converts into a file text decoration.
    #[must_use]
    pub(crate) fn to_file(&self) -> file_fg::TextDecoration {
        builder::text_decoration(self.text.clone(), self.font_size.to_file())
    }

    /// Interns this value's strings in `store` in place.
    pub fn launder(&mut self, store: &SharedStringStore) {
        self.text = store.launder(&self.text);
    }
}

/// Editable image content for a foreground decoration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageDecoration {
    /// Image path.
    path: Utf8PathBuf,
    /// Image fit mode.
    fit: ImageFit,
}

impl ImageDecoration {
    /// Creates new image decoration content.
    #[must_use]
    pub const fn new(path: Utf8PathBuf, fit: ImageFit) -> Self {
        Self { path, fit }
    }

    /// Returns the image path.
    #[must_use]
    pub fn path(&self) -> &Utf8Path {
        self.path.as_path()
    }

    /// Returns the fit mode.
    #[must_use]
    pub const fn fit(&self) -> ImageFit {
        self.fit
    }

    /// Converts from a file image decoration.
    #[must_use]
    pub(crate) fn from_file(dec: &file_fg::ImageDecoration) -> Self {
        Self {
            path: dec.path().to_path_buf(),
            fit: ImageFit::from_file(dec.fit()),
        }
    }

    /// Converts into a file image decoration.
    #[must_use]
    pub(crate) fn to_file(&self) -> file_fg::ImageDecoration {
        builder::image_decoration(self.path.clone(), self.fit.to_file())
    }
}

/// Content allowed in a foreground decoration layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForegroundDecorationKind {
    /// Text content.
    Text(TextDecoration),
    /// Image content.
    Image(ImageDecoration),
}

impl ForegroundDecorationKind {
    /// Converts from a file foreground decoration kind.
    #[must_use]
    pub(crate) fn from_file(kind: &file_fg::ForegroundDecorationKind) -> Self {
        match kind {
            file_fg::ForegroundDecorationKind::Text(text) => {
                Self::Text(TextDecoration::from_file(text))
            }
            file_fg::ForegroundDecorationKind::Image(img) => {
                Self::Image(ImageDecoration::from_file(img))
            }
        }
    }

    /// Converts into a file foreground decoration kind.
    #[must_use]
    pub(crate) fn to_file(&self) -> file_fg::ForegroundDecorationKind {
        match self {
            Self::Text(text) => file_fg::ForegroundDecorationKind::Text(text.to_file()),
            Self::Image(img) => file_fg::ForegroundDecorationKind::Image(img.to_file()),
        }
    }

    /// Interns this value's strings in `store` in place.
    pub fn launder(&mut self, store: &SharedStringStore) {
        if let Self::Text(text) = self {
            text.launder(store);
        }
    }
}

/// Editable text or image drawn in front of a model's or component's contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForegroundDecoration {
    /// Decoration identifier.
    id: ShareableString,
    /// Position in drawing coordinates.
    position: Position,
    /// Dimensions in drawing coordinates.
    dimensions: Dimension,
    /// Decoration content.
    kind: ForegroundDecorationKind,
}

impl ForegroundDecoration {
    /// Creates a new foreground decoration.
    #[must_use]
    pub fn new(
        id: impl Into<ShareableString>,
        position: Position,
        dimensions: Dimension,
        kind: ForegroundDecorationKind,
    ) -> Self {
        Self {
            id: id.into(),
            position,
            dimensions,
            kind,
        }
    }

    /// Returns the decoration identifier.
    #[must_use]
    pub const fn id(&self) -> &ShareableString {
        &self.id
    }

    /// Returns the position.
    #[must_use]
    pub const fn position(&self) -> Position {
        self.position
    }

    /// Returns the dimensions.
    #[must_use]
    pub const fn dimensions(&self) -> Dimension {
        self.dimensions
    }

    /// Returns the decoration content.
    #[must_use]
    pub const fn kind(&self) -> &ForegroundDecorationKind {
        &self.kind
    }

    /// Converts from a file foreground decoration snapshot.
    #[must_use]
    pub(crate) fn from_file(dec: &file_fg::ForegroundDecoration) -> Self {
        Self {
            id: dec.id().clone(),
            position: Position::from_file(dec.position()),
            dimensions: Dimension::from_file(dec.dimensions()),
            kind: ForegroundDecorationKind::from_file(dec.kind()),
        }
    }

    /// Converts into a file foreground decoration snapshot.
    #[must_use]
    pub(crate) fn to_file(&self) -> file_fg::ForegroundDecoration {
        builder::foreground_decoration(
            self.id.clone(),
            self.position.to_file(),
            self.dimensions.to_file(),
            self.kind.to_file(),
        )
    }

    /// Interns this value's strings in `store` in place.
    pub fn launder(&mut self, store: &SharedStringStore) {
        self.id = store.launder(&self.id);
        self.kind.launder(store);
    }
}
