pub use crate::prelude::*;

#[derive(Debug)]
pub struct InputBuffer {
  pub motions: Vec<u8>,
  pub player_id: u8,
  pub command_priority: u8,
  pub command_duration: u8,
  pub command_type: Option<CommandType>
}

impl InputBuffer {
  pub fn new(player_id: u8) -> Self {
    InputBuffer {
      motions: Vec::new(),
      player_id,
      command_priority: 0,
      command_duration: 0,
      command_type: None
    }
  }

  pub fn update(&mut self, motion_input_reader: &mut EventReader<MotionEvent>) {
    self.tick();
    for event in motion_input_reader.iter() {
      if event.player_id == self.player_id {
        self.motions.push(event.motion);
      };
    };
    let (motion_string, command_input) = self.extract_special_motions();

    let mut cm_input = String::new();
    if let Some(sp) = command_input {
      write!(cm_input,"{:?}", sp).unwrap();
    } else {
      write!(cm_input," ").unwrap();
    };

    println!("{:?} : {:?}",motion_string,cm_input);
  }

  fn tick(&mut self) {
    if self.motions.len() > 20 {
      self.motions.remove(0);
    }

    if self.command_duration > 0 {
      self.command_duration -= 1;
    }

    if self.command_duration == 0 {
      self.command_type = None;
    }
  }

  fn motion_to_string(self: &Self) -> String {
    let mut motions_string = String::new();
    for motion in self.motions.iter() {
      write!(motions_string,"{:?}",motion).unwrap();
    }
    return motions_string;
  }

  fn extract_special_motions(self: &mut Self) -> (String,Option<CommandType>) {
    let motion_string = self.motion_to_string();
    let mut priority: u8 = self.command_priority;
    let mut current_command: Option<CommandType> = None;

    for command_motion in MOTIONS.iter() {
      if command_motion.regular_expression.is_match(&motion_string[..]) {
        if command_motion.priority > priority {
          priority = command_motion.priority;
          current_command = Some(command_motion.command.clone());
        }
      }
    }

    if let Some(c) = current_command {
      self.command_type = Some(c.clone());
      self.command_duration = 5;
    }

    return (motion_string, self.command_type.clone());
  }

}
