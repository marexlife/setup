use std::process::exit;

const HELP_ADVICE: &str =
    "not valid, please use '--help'";

pub fn leave_with_help_screen() {
    print!(
        r"
(NOTE: program needs to be in project root)
definition: 'my_name' <- your desired name.

Use setup '--help' to see this screen again.
Use setup 'new' 'my_name' to create a new project.
Use setup 'mod' 'my_name' to create a new module.
Use setup 'run' to run your project.
                "
    );

    exit(0);
}

pub fn leave_with_advice() {
    panic!("{}", HELP_ADVICE)
}

pub struct UserRequest {
    instruction: String,
    name: Option<String>,
}

impl UserRequest {
    pub fn new() -> Self {
        println!("'--help' for the help screen");

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
            _ => panic!("no command provided"),
        }
    }

    pub fn visit<NewProj, NewMod, RunProj>(
        &self,
        new_proj: NewProj,
        new_mod: NewMod,
        run_proj: RunProj,
    ) where
        NewProj: Fn(&str),
        NewMod: Fn(&str),
        RunProj: Fn(),
    {
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
            (name, None) => new_proj(name),
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
