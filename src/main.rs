use std::{
    io::{stdout, Write},
    process::Command,
};

use tty_form::{
    control::{Control, SelectInput, StaticText, TextInput},
    dependency::{Action, Evaluation},
    device::StdinDevice,
    step::{CompoundStep, Step, TextBlockStep, YesNoStep},
    Form,
};
use tty_interface::Interface;

fn main() {
    let mut form = Form::new();

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

    let mut opening_paren = StaticText::new("(");
    let mut closing_paren = StaticText::new(")");

    let mut scope_input = TextInput::new("Enter the commit's scope.", true);

    let empty_scope = scope_input.set_evaluation(Evaluation::IsEmpty);
    opening_paren.set_dependency(empty_scope, Action::Hide);
    closing_paren.set_dependency(empty_scope, Action::Hide);

    let mut breaking_bang = StaticText::new("!");
    let colon = StaticText::new(": ");

    let description = TextInput::new("Enter the commit's description.", true);

    let mut long_description = TextBlockStep::new("Enter a long-form commit description.");
    long_description.set_margins(Some(1), Some(1));
    long_description.set_max_line_length(100);

    let mut breaking_step = YesNoStep::new("Is this commit a breaking change?", "BREAKING CHANGE");

    let breaking_change = breaking_step.set_evaluation(Evaluation::Equals("Yes".to_string()));
    breaking_bang.set_dependency(breaking_change, Action::Show);

    opening_paren.add_to(&mut commit_summary);
    scope_input.add_to(&mut commit_summary);
    closing_paren.add_to(&mut commit_summary);
    breaking_bang.add_to(&mut commit_summary);
    colon.add_to(&mut commit_summary);
    description.add_to(&mut commit_summary);
    commit_summary.add_to(&mut form);
    long_description.add_to(&mut form);
    breaking_step.add_to(&mut form);

    let mut stdout = stdout();
    let mut stdin = StdinDevice;

    let mut interface = Interface::new(&mut stdout).unwrap();
    let message = form.execute(&mut interface, &mut stdin).unwrap();

    interface.exit().unwrap();

    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(message)
        .output()
        .expect("failed to execute process");

    println!("status: {}", output.status);
    std::io::stdout().write_all(&output.stdout).unwrap();
    std::io::stderr().write_all(&output.stderr).unwrap();
}
