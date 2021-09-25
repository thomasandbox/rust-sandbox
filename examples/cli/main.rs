use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};

fn main() {
  loop {
    // Print `>` as the prompt.
    print!("> ");
    // Need to explicitly flush this to ensure it prints before read_line.
    stdout().flush().unwrap();

    // A variable that has a user input.
    let mut input = String::new();
    // Blocks a process until a user presses the enter key,
    // then it writes the entire input (including the newline from pressing enter.) to the variable.
    stdin().read_line(&mut input).unwrap();

    // Split the user input by a pipe.
    let mut commands = input.trim().split(" | ").peekable();
    // A variable that has a previous command.
    let mut previous_command = None;

    // Continue this loop until all commands are done.
    while let Some(command) = commands.next() {
      // Split the user input by white spaces.
      let mut parts = command.trim().split_whitespace();
      // Bind only the command to the command variable.
      let command = parts.next().unwrap();
      // Bind other options to the args variable.
      let args = parts;

      match command {
        "cd" => {
          // A variable that is a new directory.
          let new_dir = args.peekable().peek().map_or("/", |x| *x);
          let root = Path::new(new_dir);
          // Move the new directory if the directory exist.
          if let Err(e) = env::set_current_dir(&root) {
            eprintln!("{}", e);
          }

          // cd is just moving a directory.
          previous_command = None;
        }
        "exit" => return,
        command => {
          // stdin, short for Standard Input, is the file handle that your process reads to get information from you.
          let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
            Stdio::from(output.stdout.unwrap())
          });

          // stdout,short for Standard Output, is your process writes conventional output to this file handle.
          let stdout = if commands.peek().is_some() {
            Stdio::piped()
          } else {
            Stdio::inherit()
          };

          // Execute a command.
          let output = Command::new(command)
            .args(args)
            .stdin(stdin)
            .stdout(stdout)
            .spawn();

          match output {
            Ok(output) => {
              // Put the executed command into the variable as previout_command.
              previous_command = Some(output);
            }
            Err(e) => {
              // Notice it that there is not a previous command.
              previous_command = None;
              eprintln!("{}", e)
            }
          };
        }
      }
    }

    if let Some(mut final_command) = previous_command {
      // Don't allow other commands until this one completes.
      final_command.wait();
    }
  }
}
