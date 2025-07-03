//// Make colored console logs when app is working and something is needed to be mentioned.
//// All logs stored in `.cache` system folder.

import gleam/io
import simplifile

const esc = "\u{1b}"

const log_file_path = "~/.cache/timetracker/server.log"

const color_end = "[0m"

type LogColor {
  Green
  Yellow
  Red
}

fn get_color_code(color: LogColor) -> String {
  case color {
    Green -> "[32m"
    Yellow -> "[33m"
    Red -> "[31m"
  }
}

/// Green log in the console
pub fn info(message: String) {
  write_logs(message)
  io.println(
    esc <> get_color_code(Green) <> "INFO: " <> message <> esc <> color_end,
  )
  Nil
}

/// Yellow log in the console
pub fn warn(message: String) {
  write_logs(message)
  io.println(
    esc <> get_color_code(Yellow) <> "WARN: " <> message <> esc <> color_end,
  )
  Nil
}

/// Yellow log in the console
pub fn error(message: String) {
  write_logs(message)
  io.println(
    esc <> get_color_code(Red) <> "ERROR: " <> message <> esc <> color_end,
  )
  Nil
}

/// Writing logs to the .cache folder without color signs
fn write_logs(message: String) {
  let message = message <> "\n"
  let _ = simplifile.write_bits(to: log_file_path, bits: <<message:utf8>>)
  Nil
}
