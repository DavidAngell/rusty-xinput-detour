use super::mutable_xinput_state::{MutableXInputState};
use std::time::{SystemTime, Duration};

pub struct ScheduledFunction {
  duration: std::time::Duration, 
  func: Box<dyn Fn(&MutableXInputState) -> ()>,
}

impl ScheduledFunction {
    pub fn new(duration: Duration, func: Box<dyn Fn(&MutableXInputState) -> ()>) -> Self {
      Self {
        duration, 
        func,
      }
    }
}

impl ::std::fmt::Debug for ScheduledFunction {
  fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
    write!(f, "ScheduledFunction: Box<dyn Fn(&MutableXInputState) -> ()>")
  }
}

#[derive(PartialEq, Debug)]
pub enum ScheduledFunctionState {
  Ongoing,
  Completed,
}

#[derive(Debug)]
pub struct ScheduledFunctionStack {
  next_end: SystemTime,
  state: ScheduledFunctionState,
  functions: Vec<ScheduledFunction>,
}

impl ScheduledFunctionStack {
  pub fn new(functions: Vec<ScheduledFunction>) -> Self {
    Self {
      next_end: SystemTime::now() + functions[0].duration,
      functions,
      state: ScheduledFunctionState::Ongoing
    }
  }

  pub fn poll(&mut self, controller_state: &MutableXInputState) -> ScheduledFunctionState {
    // This check might not actually be needed but im too lazy to write tests so better safe than sorry
    if self.functions.is_empty() { 
      self.state = ScheduledFunctionState::Completed;
      return ScheduledFunctionState::Completed; 
    }

    if SystemTime::now() < self.next_end {
      (self.functions[0].func)(controller_state);
      ScheduledFunctionState::Ongoing
    } else {
      self.functions.remove(0);

      if self.functions.is_empty() {
        self.state = ScheduledFunctionState::Completed;
        ScheduledFunctionState::Completed
      } else {
        self.next_end += self.functions[0].duration;
        ScheduledFunctionState::Ongoing
      }
    }
  }
}

unsafe impl Send for ScheduledFunctionStack {}
unsafe impl Sync for ScheduledFunctionStack {}

// a macro to make it easier to create a ScheduledFunctionStack
// format:
//   [time to rerun function for in ms] => [function to run],
//
// example usage:
//    scheduled_function_stack! {
//      2000 => |controller_state| { controller_state.set_west_button(ButtonState::UP) },
//      2000 => |controller_state| { controller_state.set_west_button(ButtonState::DOWN) },
//      2000 => |controller_state| { controller_state.set_west_button(ButtonState::UP) },
//    }
#[macro_export]
macro_rules! scheduled_function_stack {
  ($($time_ms:literal => $func:expr),+ $(,)+) => {
    ScheduledFunctionStack::new(vec![
      $(ScheduledFunction::new(::std::time::Duration::from_millis($time_ms), Box::new($func))),+
    ])
  };
}