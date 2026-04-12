use std::process::exit;

const HELP_ADVICE: &str = "This command is ill-formed, please use '--help'.";

pub fn leave_with_help_screen() {
    print!(
        r"Help:
Usage:
Use setup '--help' to see this screen again.
Use setup 'new' 'MyNewProject' to create a new project.
Use setup 'mod' 'MyNewModule' to create a new module.
Use setup 'run' to run your project.

Note: 
This program needs to be in project root.
                "
    );

    exit(0);
}

pub fn leave_with_advice() {
    println!("{}", HELP_ADVICE);

    exit(0);
}

pub struct UserRequest {
    instruction: String,
    name: Option<String>,
}

impl UserRequest {
    pub fn new() -> Self {
        let mut instruction = None;
        let mut name = None;

        for (i, arg) in
            std::env::args().enumerate()
        {
            if i == 1 {
                instruction = Some(arg)
            } else if i == 2 {
                name = Some(arg)
            }
        }

        match (instruction, name) {
            (Some(i), Some(n)) => Self {
                instruction: i,
                name: Some(n),
            },
            (Some(i), None) => Self {
                instruction: i,
                name: None,
            },
            _ => {
                eprintln!("no command provided");

                exit(0)
            }
        }
    }

    pub fn chose(
        &self,
        new_proj: fn(&str),
        new_mod: fn(&str),
        run_proj: fn(),
    ) {
        match (
            self.instruction.as_str(),
            &self.name,
        ) {
            ("run", None) => run_proj(),
            ("run", Some(_)) => {
                self.leave_with_no_args_advice()
            }
            ("--help", None) => {
                leave_with_help_screen()
            }
            ("--help", Some(_)) => {
                self.leave_with_no_args_advice()
            }
            ("mod", Some(name)) => new_mod(name),
            ("new", Some(name)) => new_proj(name),
            _ => leave_with_advice(),
        }
    }

    fn leave_with_no_args_advice(&self) {
        let command_name =
            self.instruction.as_str();

        panic!(
            "after '{}' there should be nothing, try just 'setup {}'",
            command_name, command_name
        )
    }
}
