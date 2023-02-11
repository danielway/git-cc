use std::{
    env,
    io::{stdout, Write},
    process::exit,
};

use git::{commit, has_staged_changes};
use tty_form::{
    control::{Control, SelectInput, StaticText, TextInput},
    dependency::{Action, DependencyId, Evaluation},
    device::StdinDevice,
    step::{CompoundStep, KeyValueStep, Step, TextBlockStep, YesNoStep},
    Error, Form, Result,
};
use tty_interface::Interface;

mod git;

fn main() {
    execute().expect("should commit successfully");
}

fn execute() -> Result<()> {
    if !has_staged_changes()? {
        eprintln!("There are no staged changes to commit.");
        exit(1);
    }

    let args: Vec<String> = env::args().collect();
    let quick_mode = args.len() > 1 && args[1].to_lowercase().trim() == "-q";

    let mut form = Form::new();

    if quick_mode {
        add_summary(&mut form, None);
    } else {
        let (breaking_step, breaking_change) = add_breaking();
        add_summary(&mut form, Some(breaking_change));
        add_description(&mut form);
        breaking_step.add_to(&mut form);
        add_trailers(&mut form);
    }

    let mut stdout = stdout();
    let mut stdin = StdinDevice;
    let mut interface = Interface::new_relative(&mut stdout)?;

    let result = form.execute(&mut interface, &mut stdin);
    interface.exit()?;

    match result {
        Ok(message) => {
            let output = commit(&message)?;
            std::io::stdout().write_all(&output.stdout)?;
            std::io::stderr().write_all(&output.stderr)?;
        }
        Err(Error::Canceled) => {}
        Err(err) => eprintln!("An unexpected error occurred: {:?}", err),
    }

    Ok(())
}

fn add_summary(form: &mut Form, breaking_change: Option<DependencyId>) {
    let mut commit_summary = CompoundStep::new();
    commit_summary.set_max_line_length(80);

    SelectInput::new(
        "Select the commit type.",
        vec![
            ("feat", "implemented a new feature"),
            ("bug", "fixed existing behavior"),
            ("docs", "added documentation"),
            ("chore", "non-source changes"),
        ],
    )
    .add_to(&mut commit_summary);

    let mut scope_input = TextInput::new("Enter the commit's scope.", true);
    let empty_scope = scope_input.set_evaluation(Evaluation::IsEmpty);

    let mut opening_paren = StaticText::new("(");
    opening_paren.set_dependency(empty_scope, Action::Hide);
    opening_paren.add_to(&mut commit_summary);

    scope_input.add_to(&mut commit_summary);

    let mut closing_paren = StaticText::new(")");
    closing_paren.set_dependency(empty_scope, Action::Hide);
    closing_paren.add_to(&mut commit_summary);

    if let Some(breaking_change) = breaking_change {
        let mut breaking_bang = StaticText::new("!");
        breaking_bang.set_dependency(breaking_change, Action::Show);
        breaking_bang.add_to(&mut commit_summary);
    }

    StaticText::new(": ").add_to(&mut commit_summary);

    TextInput::new("Enter the commit's description.", true).add_to(&mut commit_summary);

    commit_summary.add_to(form);
}

fn add_description(form: &mut Form) {
    let mut long_description = TextBlockStep::new("Enter a long-form commit description.");
    long_description.set_margins(Some(1), Some(1));
    long_description.set_max_line_length(100);
    long_description.add_to(form);
}

fn add_breaking() -> (YesNoStep, DependencyId) {
    let mut breaking_step = YesNoStep::new(
        "Is this commit a breaking change?",
        "Enter a description of the breaking change.",
        "BREAKING CHANGE",
    );

    let breaking_change = breaking_step.set_evaluation(Evaluation::Equal("Yes".to_string()));

    (breaking_step, breaking_change)
}

fn add_trailers(form: &mut Form) {
    KeyValueStep::new("Enter any key-value trailers, such as tickets.").add_to(form);
}
