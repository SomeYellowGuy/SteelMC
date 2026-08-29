use text_components::TextComponent;
use crate::command::brigadier::CommandSyntaxError;
use crate::command::execution::{CommandSource, SteelCommandContext};

#[derive(Debug, Clone)]
pub struct CommandResponseTracker<'a, E> {
    total_value: i32,

    only_element: Option<&'a E>,
    element_count: i32,
    only_non_zero_element: Option<&'a E>,
    non_zero_element_count: i32
}

impl<E> Default for CommandResponseTracker<'_, E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, E> CommandResponseTracker<'a, E> {
    pub fn new() -> CommandResponseTracker<'a, E> {
        CommandResponseTracker {
            total_value: 0,
            only_element: None,
            element_count: 0,
            only_non_zero_element: None,
            non_zero_element_count: 0
        }
    }

    pub fn track_with_value(&mut self, element: &'a E, value: i32) {
        self.total_value += value;
        self.element_count += 1;
        self.only_element = if self.element_count == 1 {
            Some(element)
        } else {
            None
        };

        if value != 0 {
            self.non_zero_element_count += 1;
            self.only_non_zero_element = if self.non_zero_element_count == 1 {
                Some(element)
            } else {
                None
            };
        }
    }

    pub fn track_with_bool(&mut self, element: &'a E, value: bool) {
        self.track_with_value(element, value as i32);
    }

    pub fn track(&mut self, element: &'a E) {
        self.track_with_value(element, 1);
    }

    pub fn total_value(&self) -> i32 {
        self.total_value
    }

    fn first_element(&self, element_type: ElementType) -> Option<&'a E> {
        match element_type {
            ElementType::Any => self.only_element,
            ElementType::NonZero => self.only_non_zero_element
        }
    }

    fn element_count(&self, element_type: ElementType) -> i32 {
        match element_type {
            ElementType::Any => self.element_count,
            ElementType::NonZero => self.non_zero_element_count
        }
    }

    pub fn dispatch<R, Args>(&self, element_type: ElementType, dispatch: Dispatch<R, &'a E, Args>, args: Args) -> R {
        let first_element = self.first_element(element_type);
        if let Some(first_element) = first_element {
            (dispatch.on_single)(first_element, self.total_value, args)
        } else {
            (dispatch.on_multiple)(self.element_count(element_type), self.total_value, args)
        }
    }

    pub fn send_feedback<Args: Clone>(
        &self,
        context: &SteelCommandContext<CommandSource>,
        broadcast: bool,
        element_type: ElementType,
        messages: Messages<&'a E, Args>,
        args: Args
    ) -> Result<i32, CommandSyntaxError> {
        messages.throw_if_zero(self.element_count(element_type), args.clone())?;
        context.source().send_success(&*self.dispatch(element_type, messages.on_success, args), broadcast);
        Ok(self.total_value)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ElementType {
    Any,
    NonZero
}

pub type ErrorHandler<Args> = fn(Args) -> CommandSyntaxError;
pub type SingleHandler<R, E, Args> = fn(E, i32, Args) -> R;
pub type MultipleHandler<R, Args> = fn(i32, i32, Args) -> R;

pub struct Dispatch<R, E, Args> {
    on_single: SingleHandler<R, E, Args>,
    on_multiple: MultipleHandler<R, Args>,
}

pub struct Messages<E, Args> {
    on_zero: Option<ErrorHandler<Args>>,
    on_success: Dispatch<Box<TextComponent>, E, Args>
}

impl<E, Args> Messages<E, Args> {

    pub fn new(
        on_zero: Option<ErrorHandler<Args>>,
        on_single: SingleHandler<Box<TextComponent>, E, Args>,
        on_multiple: MultipleHandler<Box<TextComponent>, Args>
    ) -> Self {
        Self {
            on_zero,
            on_success: Dispatch {
                on_single, on_multiple
            }
        }
    }

    pub fn with_no_error_handler(
        on_single: SingleHandler<Box<TextComponent>, E, Args>,
        on_multiple: MultipleHandler<Box<TextComponent>, Args>
    ) -> Self {
        Self::new(None, on_single, on_multiple)
    }

    pub fn throw_if_zero(&self, value: i32, args: Args) -> Result<(), CommandSyntaxError> {
        if let Some(on_zero) = self.on_zero && value == 0 {
            Err(on_zero(args))
        } else {
            Ok(())
        }
    }
}