use core::ops::Deref;
// use core::fmt::Display;
use core::fmt::Debug;

// use alloc::vec::Vec;
// use alloc::format;


use chalk::Color;
use chalk::Size2d;
use chalk::Unit;
use chalk::Rect;

use crate::style::StyleValue;
use crate::style::StyleValueRef;
// use crate::style::primitive::Weight;
use crate::style::primitive::Unit;
use crate::style::primitive::Size2d;
use crate::style::primitive::Rect;
use crate::style::primitive::Color;

/// TODO
#[derive(Default, Debug, PartialEq)]
pub struct Flex(bool);

impl Into<StyleValueRef> for Flex {
    /// TODO
    fn into(self) -> StyleValueRef {
        StyleValueRef::Flex(self)
    }
}

impl StyleValue for Flex {
    //..
}

/// TODO
#[derive(Default, Debug, PartialEq)]
pub enum FlexDirection {
    /// TODO
    Column,
    
    /// TODO
    #[default]
    Row,
}

impl Into<StyleValueRef> for FlexDirection {
    /// TODO
    fn into(self) -> StyleValueRef {
        StyleValueRef::FlexDirection(self)
    }
}

impl StyleValue for FlexDirection {
    //..
}

//--
/// TODO
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct FlexBasis(Unit<f32>);

#[automatically_derived]
impl FlexBasis {
    pub fn new(value: f32) -> Self {
        FlexBasis(Unit::Px(value))
    }
}

#[automatically_derived]
impl FlexBasis {
    pub fn unit(&self) -> &Unit {
        &self.0
    }
}

#[automatically_derived]
impl Into<Unit> for FlexBasis {
    fn into(self) -> Unit {
        self.0
    }
}

#[automatically_derived]
impl Deref for FlexBasis {
    type Target = Unit;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl Into<StyleValueRef> for FlexBasis {
    fn into(self) -> StyleValueRef {
        StyleValueRef::FlexBasis(self)
    }
}

#[automatically_derived]
impl StyleValue for FlexBasis {
    //..
}

//--
/// TODO
#[derive(Debug, PartialEq)]
pub struct FlexGrow(f32);

impl FlexGrow {
    /// TODO
    pub fn new(value: f32) -> Self {
        FlexGrow(value)
    }
}
    
impl Deref for FlexGrow {
    type Target = f32;
    
    /// TODO
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl Into<StyleValueRef> for FlexGrow {
    /// TODO
    fn into(self) -> StyleValueRef {
        StyleValueRef::FlexGrow(self)
    }
}

#[automatically_derived]
impl StyleValue for FlexGrow {
    //..
}

/// TODO
#[derive(Debug, PartialEq)]
pub struct FlexShrink(f32);

impl FlexShrink {
    /// TODO
    pub fn new(value: f32) -> Self {
        FlexShrink(value)
    }
}

#[automatically_derived]
impl Into<StyleValueRef> for FlexShrink {
    /// TODO
    fn into(self) -> StyleValueRef {
        StyleValueRef::FlexShrink(self)
    }
}

impl Deref for FlexShrink {
    type Target = f32;
    
    /// TODO
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl StyleValue for FlexShrink {
    //..
}

//--
/// TODO
#[derive(Unit, Default, Clone, Copy, Debug, PartialEq)]
pub struct Gap(Unit<f32>);

/// TODO
#[derive(Color, Default, Debug, PartialEq)]
pub struct BackgroundColor(Color);

/// TODO
#[derive(Rect, Default, Debug, PartialEq)]
pub struct Margin(Rect<f32>);

/// TODO
#[derive(Rect, Default, Debug, PartialEq)]
pub struct Padding(Rect<f32>);

/// TODO
#[derive(Size2d, Default, Debug, PartialEq)]
pub struct BoxSize(Size2d<f32>);

//--
/// TODO
#[derive(Unit, Default, Clone, Copy, Debug, PartialEq)]
pub struct Width(Unit<f32>);

//--
/// TODO
#[derive(Unit, Default, Clone, Copy, Debug, PartialEq)]
pub struct Height(Unit<f32>);

//--
/// TODO
#[derive(Unit, Default, Clone, Copy, Debug, PartialEq)]
pub struct MinWidth(Unit<f32>);

//--
/// TODO
#[derive(Unit, Default, Clone, Copy, Debug, PartialEq)]
pub struct MinHeight(Unit<f32>);

//--
/// TODO
#[derive(Unit, Default, Clone, Copy, Debug, PartialEq)]
pub struct MaxWidth(Unit<f32>);

//--
/// TODO
#[derive(Unit, Default, Clone, Copy, Debug, PartialEq)]
pub struct MaxHeight(Unit<f32>);

//--
/// TODO
#[derive(Rect, Default, Debug, PartialEq)]
pub struct BorderWeight(Rect<f32>);

/// TODO
#[derive(Color, Default, Debug, PartialEq)]
pub struct BorderColor(Color);
