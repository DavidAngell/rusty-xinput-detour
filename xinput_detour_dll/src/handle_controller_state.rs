use winapi::um::xinput::*;
use winapi::shared::minwindef::DWORD;
use super::mutable_xinput_state::*;
use super::function_scheduler::*;
use retour::static_detour;

// set up the detour for the XInputGetState function
static_detour! {
  pub static XInputGetStateHook: unsafe extern "system" fn(DWORD, *mut XINPUT_STATE) -> DWORD;
}

// a user-defined function that modifies the controller state based on the current state
// some examples are included below
pub fn handle_controller_state(controller_state: &MutableXInputState, scheduled_functions: &mut Vec<ScheduledFunctionStack>) {
  // Example 1: 
  // When the B button is pressed, press the X button
  if controller_state.east_button() {
    controller_state.set_west_button(ButtonState::DOWN);
  }

  // Example 2:
  // When the D-Pad Up button is pressed, press the A button for
  // 2 seconds, then release it for 2 seconds, then repeat
  if controller_state.arrow_up() {
    scheduled_functions.push(
      scheduled_function_stack!(
        2000 => |cs| { cs.set_south_button(ButtonState::DOWN) },
        2000 => |cs| { cs.set_south_button(ButtonState::UP) },
        2000 => |cs| { cs.set_south_button(ButtonState::DOWN) },
        2000 => |cs| { cs.set_south_button(ButtonState::UP) },
        2000 => |cs| { cs.set_south_button(ButtonState::DOWN) },
        2000 => |cs| { cs.set_south_button(ButtonState::UP) },
      )
    );
  }

  // Example 3:
  // When the D-Pad Down button is pressed, interupt the D-Pad Down button,
  // the press the right trigger for 2 seconds, then release it for 2 seconds,
  // then repeat
  if controller_state.arrow_down() {
    // interupt the D-Pad Down button
    controller_state.set_arrow_down(ButtonState::UP);

    // schedule the right trigger to be pressed and released
    scheduled_functions.push(
      scheduled_function_stack!(
        2000 => |cs| { cs.set_right_trigger(u8::MAX) },
        2000 => |cs| { cs.set_right_trigger(u8::MIN) },
        2000 => |cs| { cs.set_right_trigger(u8::MAX) },
        2000 => |cs| { cs.set_right_trigger(u8::MIN) },
        2000 => |cs| { cs.set_right_trigger(u8::MAX) },
        2000 => |cs| { cs.set_right_trigger(u8::MIN) },
      )
    );
  }

  // Example 4:
  // Swap the left and right sticks
  let left_stick = controller_state.left_stick_raw();
  let right_stick = controller_state.right_stick_raw();
  controller_state.set_left_stick_raw(right_stick);
  controller_state.set_right_stick_raw(left_stick);

}