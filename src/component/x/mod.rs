use crate::element::Element;

// TODO: Move this to a feature on Chizel ..
use crate as slate;

// TODO: Move this into the Element derive ..
use alloc::boxed::Box;

pub mod layout {
    use super::*;
    
    use alloc::string::String;

    #[derive(Element, Default, Clone, Hash, Debug)]
    pub struct Container;
    
    pub type Div = Container;
    pub type Section = Container;
    pub type Main = Container;
    pub type Header = Container;
    pub type Footer = Container;
    pub type Sidebar = Container;
    
    impl Container {
        pub fn with_alt<S: Into<String>>(self, _value: S) -> Self {
            self // etc.
        }
    }
}

pub mod content {
    use super::*;
    
    use alloc::string::String;
    
    #[derive(Default, Clone, Hash, Debug)]
    pub struct TextBlock {
        text: String,
    }
    
    impl TextBlock {
        pub fn with_text<S: Into<String>>(mut self, value: S) -> Self {
            self.text = value.into();
            self // etc.
        }
    }
    
    impl Element for TextBlock {
        fn content(&self) -> Option<&str> {
            Some(self.text.as_ref())
        }
    }
}

pub mod input {
    use crate::element::DrawFn;

    use super::*;

    use alloc::string::String;

    #[derive(Element, Default, Clone, Hash, Debug)]
    pub struct Label;
    
    impl Label {
        pub fn with_text(self, _text: &str) -> Self {
            self // etc.
        }
    }
    
    #[derive(Element, Default, Clone, Hash, Debug)]
    pub struct TextInput;

    impl TextInput {
        pub fn with_value(self, _value: &str) -> Self {
            self // etc.
        }
    }
    
    #[derive(Default, Clone, Hash, Debug)]
    // #[render(self.draw)]
    pub struct Button(String);
    
    impl Button {
        pub fn with_value(mut self, value: &str) -> Self {
            self.0 = String::from(value);
            self // etc.
        }
    }
    
    impl Element for Button {
        fn draw(&self) -> DrawFn {
            // chizel::uix! {
            //     ^self {
            //         BackgroundColor::hex("#000000"),
            //     }
                
            //     .label {
            //         ContentColor::hex("#000000"),
            //     }
                
            //     #[class(label)]
            //     <Label text="TODO" />
            // }
            |scaffold| {
                Ok(())
            }
        }
    }
}
