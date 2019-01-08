use read::{
    Abbreviations, Attribute, AttributeValue, CompilationUnitHeader, DebugAbbrev, DebugInfo,
    DebugLine, DebugStr, DebugTypes, LocationLists, RangeLists, Reader, Result, TypeUnitHeader,
};

/// All of the commonly used DWARF sections, and other common information.
#[derive(Debug, Default)]
pub struct Dwarf<R: Reader> {
    /// The endianity of bytes that are read.
    pub endian: R::Endian,

    /// The `.debug_abbrev` section.
    pub debug_abbrev: DebugAbbrev<R>,

    /// The `.debug_info` section.
    pub debug_info: DebugInfo<R>,

    /// The `.debug_line` section.
    pub debug_line: DebugLine<R>,

    /// The `.debug_str` section.
    pub debug_str: DebugStr<R>,

    /// The `.debug_str` section for a supplementary object file.
    pub debug_str_sup: DebugStr<R>,

    /// The `.debug_types` section.
    pub debug_types: DebugTypes<R>,

    /// The location lists in the `.debug_loc` and `.debug_loclists` sections.
    pub locations: LocationLists<R>,

    /// The range lists in the `.debug_ranges` and `.debug_rnglists` sections.
    pub ranges: RangeLists<R>,
}

impl<R: Reader> Dwarf<R> {
    /// Parse the abbreviations for a compilation unit.
    // TODO: provide caching of abbreviations
    #[inline]
    pub fn abbreviations(
        &self,
        unit: &CompilationUnitHeader<R, R::Offset>,
    ) -> Result<Abbreviations> {
        unit.abbreviations(&self.debug_abbrev)
    }

    /// Parse the abbreviations for a type unit.
    // TODO: provide caching of abbreviations
    #[inline]
    pub fn type_abbreviations(&self, unit: &TypeUnitHeader<R, R::Offset>) -> Result<Abbreviations> {
        unit.abbreviations(&self.debug_abbrev)
    }

    /// Try to return an attribute's value as a string slice.
    ///
    /// If the attribute's value is either an inline `DW_FORM_string` string,
    /// or a `DW_FORM_strp` reference to an offset into the `.debug_str`
    /// section, or a `DW_FORM_strp_sup` reference to an offset into a supplementary
    /// object file, return the attribute's string value as `Some`. Other attribute
    /// value forms are returned as `None`.
    pub fn attr_string(&self, attr: &Attribute<R>) -> Option<R> {
        match attr.value() {
            AttributeValue::String(ref string) => Some(string.clone()),
            AttributeValue::DebugStrRef(offset) => self.debug_str.get_str(offset).ok(),
            AttributeValue::DebugStrRefSup(offset) => self.debug_str_sup.get_str(offset).ok(),
            _ => None,
        }
    }
}
