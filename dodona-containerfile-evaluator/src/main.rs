mod config;

use core::fmt;
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

    if let Some(comments) = config.comments {
        cmd(Command::StartContext { description: None });
        cmd(Command::StartTestcase {
            description: Message::String("Comments".to_owned()),
        });
        for comment in comments {
            cmd(Command::StartTest {
                expected: comment.to_owned(),
                format: None,
                description: None,
                channel: None,
            });

            if parsed.comments.iter().any(|x| x.contains(&comment)) {
                cmd(Command::CloseTest {
                    generated: comment.to_owned(),
                    accepted: None,
                    status: Status {
                        r#enum: StatusEnum::Correct,
                        human: "Correct".to_owned(),
                    },
                });
            } else {
                cmd(Command::CloseTest {
                    generated: "".to_owned(),
                    accepted: None,
                    status: Status {
                        r#enum: StatusEnum::Wrong,
                        human: format!("No comment found containing \"{}\"", &comment),
                    },
                });
            };
        }

        cmd(Command::CloseTestcase { accepted: None });
        cmd(Command::CloseContext { accepted: None });
    }

    if let Some(stage) = parsed.iter_stages().last() {
        if let Some(base_image) = config.from {
            test_from_instruction(&stage, &base_image);
        }

        if let Some(user) = config.user {
            test_misc_instruction(&stage, "USER", &user);
        }

        if let Some(workdir) = config.workdir {
            test_misc_instruction(&stage, "WORKDIR", &workdir);
        }

        if let Some(expose) = config.expose {
            test_expose_instruction(&stage, &expose);
        }
    }
}

fn test_from_instruction(stage: &Stage, expected: &config::From) {
    cmd(Command::StartContext { description: Some(Message::String("FROM instruction".to_owned())) });

    if let Some(from) = stage.instructions.iter().filter_map(|x| x.as_from()).last() {
        cmd(Command::StartTestcase {
            description: Message::String("image".to_owned()),
        });
        cmp_test(&expected.image, Some(&from.image_parsed.image), "image");
        cmd(Command::CloseTestcase { accepted: None });

        if let Some(expected_tag) = &expected.tag {
            cmd(Command::StartTestcase {
                description: Message::String("tag".to_owned()),
            });
            cmp_test::<&str>(&expected_tag, from.image_parsed.tag.as_deref(), "image tag");
            cmd(Command::CloseTestcase { accepted: None });
        }
        if let Some(expected_hash) = &expected.hash {
            cmd(Command::StartTestcase {
                description: Message::String("digest".to_owned()),
            });
            cmp_test::<&str>(&expected_hash, from.image_parsed.hash.as_deref(), "image hash");
            cmd(Command::CloseTestcase { accepted: None });
        }
        cmd(Command::CloseContext { accepted: None });
    }
    else {
        cmd(Command::CloseContext { accepted: Some(false) });
    }
}

fn test_expose_instruction(stage: &Stage, expected: &[config::Port]) {
    cmd(Command::StartContext { description: Some(Message::String("EXPOSE instruction".to_owned())) });
    let expose_instructions: Vec<&ExposeInstruction> = stage.instructions.iter().filter_map(|x| x.as_expose()).collect();
    for port in expected {
        cmd(Command::StartTestcase {
            description: Message::String(format!("{}/{}", port.number, port.protocol.as_deref().unwrap_or("tcp"))),
        });
        cmd(Command::StartTest {
            expected: format!("{}", port.number),
            format: None,
            description: None,
            channel: None,
        });

        if let Some(found_port) = expose_instructions.iter().find_map(|x| x.vars.iter().find(|x| x.port.content == port.number)) {
            cmd(Command::CloseTest {
                generated: format!("{}", port.number),
                accepted: None,
                status: Status {
                    r#enum: StatusEnum::Correct,
                    human: "Correct".to_owned(),
                }
            });
            cmd(Command::CloseTestcase { accepted: None });

            cmd(Command::StartTestcase {
                description: Message::String("protocol".to_owned()),
            });
            cmp_test(port.protocol.as_deref().unwrap_or("tcp"), Some(found_port.protocol.as_ref().map_or("tcp", |x| &x.content)), "image tag");
            cmd(Command::CloseTestcase { accepted: None });

        }
        else {
            cmd(Command::CloseTest {
                generated: "".to_owned(),
                accepted: None,
                status: Status {
                    r#enum: StatusEnum::Wrong,
                    human: "expose instruction not found".to_owned(),
                },
            });
            cmd(Command::CloseTestcase { accepted: None });
        }
    }

    cmd(Command::CloseContext { accepted: None });
}

fn test_misc_instruction(stage: &Stage, name: &str, argument: &str) {
    cmd(Command::StartContext { description: None });
    cmd(Command::StartTestcase {
        description: Message::String(format!("{} instruction", name)),
    });

    cmp_test(
        argument,
        get_content_from_misc_instruction(&stage.instructions, name).as_deref().map(|x| x.trim()),
        name);

    cmd(Command::CloseTestcase { accepted: None });
    cmd(Command::CloseContext { accepted: None });
}

fn cmp_test<T: fmt::Display + PartialEq>(expected: T, value: Option<T>, name: &str) {
    cmd(Command::StartTest {
        expected: format!("{}", expected),
        format: None,
        description: None,
        channel: None,
    });
    if let Some(value) = value {
        let status = if expected == value {
            Status {
                r#enum: StatusEnum::Correct,
                human: "Correct".to_owned(),
            }
        } else {
            Status {
                r#enum: StatusEnum::Wrong,
                human: format!("{} doesn't match requested \"{}\"", name, expected),
            }
        };

        cmd(Command::CloseTest {
            generated: format!("{}", value),
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
