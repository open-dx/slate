use core::any::TypeId;
use core::fmt::Debug;
// use core::fmt::Display;

use alloc::vec::Vec;
// use alloc::format;

use enum_dispatch::enum_dispatch;

use chalk::StyleProperty;
// use tracing::Value;
// use chalk::Unit;

use crate::x::HashMap;

use crate::style::property::*;

//---
/// TODO
// #[derive(Debug)]
pub struct StyleSheet {
    /// TODO
    styles: HashMap<TypeId, Vec<StyleValueRef>>,
}

impl core::fmt::Debug for StyleSheet {
    /// TODO
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut struct_fmt = f.debug_struct("StyleSheet");
        
        for (type_id, style_values) in &self.styles {
            let type_name = StylePropertyName::get(type_id).unwrap_or("Unknown");
            struct_fmt.field(type_name, style_values);
        }
        
        struct_fmt.finish()
    }
}

impl StyleSheet {
    /// TODO
    pub fn new() -> Self {
        StyleSheet {
            styles: HashMap::new(),
        }
    }
}

impl StyleSheet {
    /// TODO
    pub fn get<P: StyleValue + 'static>(&self) -> Option<&Vec<StyleValueRef>> {
        let type_id = TypeId::of::<P>();
        self.styles.get(&type_id)
    }
    
    /// TODO
    pub fn styles(&self) -> &HashMap<TypeId, Vec<StyleValueRef>> {
        &self.styles
    }
}

impl StyleSheet {
    /// TODO
    pub fn push<P: StyleValue + 'static>(&mut self, value: P) {
        let type_id = TypeId::of::<P>();
        self.styles.entry(type_id).or_insert_with(Vec::new).push(value.into());
    }
    
    // Method to append styles from another StyleSheet
    pub fn append(&mut self, other: &mut StyleSheet) {
        for (type_id, mut values) in other.styles.drain() {
            self.styles.entry(type_id).or_insert_with(Vec::new).append(&mut values);
        }
    }
}

pub struct StylePropertyName(TypeId, &'static str);

static STYLE_PROPERTY_NAMES: [StylePropertyName; 16] = [
    StylePropertyName(TypeId::of::<Flex>(), "Flex"),
    StylePropertyName(TypeId::of::<FlexGrow>(), "FlexGrow"),
    StylePropertyName(TypeId::of::<FlexDirection>(), "FlexDirection"),
    StylePropertyName(TypeId::of::<BackgroundColor>(), "BackgroundColor"),
    StylePropertyName(TypeId::of::<Margin>(), "Margin"),
    StylePropertyName(TypeId::of::<Padding>(), "Padding"),
    StylePropertyName(TypeId::of::<BoxSize>(), "BoxSize"),
    StylePropertyName(TypeId::of::<Width>(), "Width"),
    StylePropertyName(TypeId::of::<Height>(), "Height"),
    StylePropertyName(TypeId::of::<MinWidth>(), "MinWidth"),
    StylePropertyName(TypeId::of::<MinHeight>(), "MinHeight"),
    StylePropertyName(TypeId::of::<MaxWidth>(), "MaxWidth"),
    StylePropertyName(TypeId::of::<MaxHeight>(), "MaxHeight"),
    StylePropertyName(TypeId::of::<Gap>(), "Gap"),
    StylePropertyName(TypeId::of::<BorderWeight>(), "BorderWeight"),
    StylePropertyName(TypeId::of::<BorderColor>(), "BorderColor"),
];

impl StylePropertyName {
    /// TODO
    pub fn get(type_id: &TypeId) -> Option<&'static str> {
        STYLE_PROPERTY_NAMES.iter()
            // Find the property that matches `type_id` ..
            .find(|StylePropertyName(id, _)| id == type_id)
            // Return the found property name.
            .map(|StylePropertyName(_, name)| *name)
    }
}

/// TODO
pub trait StyleValue: Debug + PartialEq + Into<StyleValueRef> {
    //..
}

/// Provides (faster?) dynamic dispatch for the StyleValue (via `enum_dispatch`).
/// 
/// Represents a handle to a StyleValue with a few extra features:
/// 1. TODO
#[derive(StyleProperty, PartialEq)]
#[enum_dispatch(StyleValue)]
pub enum StyleValueRef {
    Flex(Flex),
    FlexBasis(FlexBasis),
    FlexDirection(FlexDirection),
    FlexGrow(FlexGrow),
    FlexShrink(FlexShrink),
    Gap(Gap),
    BackgroundColor(BackgroundColor),
    Margin(Margin),
    Padding(Padding),
    BoxSize(BoxSize),
    Width(Width),
    Height(Height),
    MinWidth(MinWidth),
    MinHeight(MinHeight),
    MaxWidth(MaxWidth),
    MaxHeight(MaxHeight),
    BorderWeight(BorderWeight),
    BorderColor(BorderColor),
}

#[automatically_derived]
impl core::fmt::Debug for StyleValueRef {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            StyleValueRef::Flex(value) => write!(f, "{:?}", value),
            StyleValueRef::FlexBasis(value) => write!(f, "{:?}", value),
            StyleValueRef::FlexDirection(value) => write!(f, "{:?}", value),
            StyleValueRef::FlexGrow(value) => write!(f, "{:?}", value),
            StyleValueRef::FlexShrink(value) => write!(f, "{:?}", value),
            StyleValueRef::Gap(value) => write!(f, "{:?}", value),
            StyleValueRef::BackgroundColor(value) => write!(f, "{:?}", value),
            StyleValueRef::Margin(value) => write!(f, "{:?}", value),
            StyleValueRef::Padding(value) => write!(f, "{:?}", value),
            StyleValueRef::BoxSize(value) => write!(f, "{:?}", value),
            StyleValueRef::Width(value) => write!(f, "{:?}", value),
            StyleValueRef::Height(value) => write!(f, "{:?}", value),
            StyleValueRef::MinWidth(value) => write!(f, "{:?}", value),
            StyleValueRef::MinHeight(value) => write!(f, "{:?}", value),
            StyleValueRef::MaxWidth(value) => write!(f, "{:?}", value),
            StyleValueRef::MaxHeight(value) => write!(f, "{:?}", value),
            StyleValueRef::BorderWeight(value) => write!(f, "{:?}", value),
            StyleValueRef::BorderColor(value) => write!(f, "{:?}", value),
        }
    }
}
