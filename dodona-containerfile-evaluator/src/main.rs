mod config;

use std::{
    fs::{self, File},
    io,
    path::PathBuf,
};

use clap::*;
use dockerfile_parser::*;
use dodona::{Command, Message, Status, StatusEnum};

use self::config::Config;

fn main() -> io::Result<()> {
    let matches = command!()
        .name(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            arg!(-c --config <path> "Sets a custom config file")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(<Containerfile> "Containerfile to operate on")
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let config_path = matches.get_one::<PathBuf>("config").expect("config is required");
    let config_content = fs::read_to_string(config_path)?;
    let config: Config = serde_json::from_str(&config_content).expect("invalid config");

    let containerfile_path = matches.get_one::<PathBuf>("Containerfile").expect("containerfile is required");
    let containerfile = File::open(containerfile_path)?;

    analyze(containerfile, config);

    Ok(())
}

fn analyze(containerfile: File, config: Config) {
    let parsed = Dockerfile::from_reader(containerfile).expect("");
    if let Some(stage) = parsed.iter_stages().last() {
        if let Some(user) = config.user {
            test_misc_instruction(&stage, "USER", &user);
        }

        if let Some(workdir) = config.workdir {
            test_misc_instruction(&stage, "WORKDIR", &workdir);
        }
    }
}

fn test_misc_instruction(stage: &Stage, name: &str, argument: &str) {
    cmd(Command::StartContext { description: None });
    cmd(Command::StartTestcase {
        description: Message::String(format!("{} instruction", name)),
    });
    cmd(Command::StartTest {
        expected: argument.to_owned(),
        format: None,
        description: None,
        channel: None,
    });
    if let Some(content) = get_content_from_misc_instruction(&stage.instructions, name) {
        let content = content.trim();
        let status = if content != argument {
            Status {
                r#enum: StatusEnum::Wrong,
                human: format!("{} doesn't match requested \"{}\"", name, argument),
            }
        } else {
            Status {
                r#enum: StatusEnum::Correct,
                human: "Correct".to_owned(),
            }
        };
        cmd(Command::CloseTest {
            generated: content.to_owned(),
            accepted: None,
            status,
        });
    } else {
        cmd(Command::CloseTest {
            generated: "".to_owned(),
            accepted: None,
            status: Status {
                r#enum: StatusEnum::Wrong,
                human: format!("{} instruction not found", name),
            },
        });
    }
    cmd(Command::CloseTestcase { accepted: None });
    cmd(Command::CloseContext { accepted: None });
}

fn get_content_from_misc_instruction(instructions: &[&Instruction], name: &str) -> Option<String> {
    if let Some(instruction) = instructions
        .iter()
        .filter_map(|x| x.as_misc())
        .filter(|x| name.eq_ignore_ascii_case(&x.instruction.content))
        .last()
    {
        return Some(
            instruction
                .arguments
                .components
                .iter()
                .filter_map(|x| match x {
                    BreakableStringComponent::String(s) => Some(s.content.as_str()),
                    _ => None,
                })
                .collect(),
        );
    }

    None
}

fn cmd(cmd: Command) {
    println!("{}", serde_json::to_string(&cmd).unwrap())
}
