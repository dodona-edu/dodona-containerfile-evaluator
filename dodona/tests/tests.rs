use dodona::*;

fn example() -> Vec<Command> {
    let mut example = Vec::new();
    example.push(Command::StartJudgement);
    example.push(Command::AppendMessage {
        message: Message::String("will be added to the judgement".to_owned()),
    });
    example.push(Command::AnnotateCode {
        row: 3,
        column: Some(4),
        text: "some info on the fourth line, fifth column of the source".to_owned(),
        external_url: None,
        r#type: None,
        rows: None,
        columns: None,
    });
    example.push(Command::StartTab {
        title: "Tab One".to_owned(),
        hidden: None,
        permission: None,
    });
    example.push(Command::StartContext { description: None });
    example.push(Command::StartTestcase {
        description: Message::String("case 1".to_owned()),
    });
    example.push(Command::StartTest {
        expected: "SOMETHING".to_owned(),
        format: None,
        description: None,
        channel: None,
    });
    example.push(Command::AppendMessage {
        message: Message::String("some more info about the test".to_owned()),
    });
    example.push(Command::CloseTest {
        generated: "SOMETHING".to_owned(),
        accepted: None,
        status: Status {
            r#enum: StatusEnum::Correct,
            human: "Correct".to_owned(),
        },
    });
    example.push(Command::CloseTestcase { accepted: None });
    example.push(Command::CloseContext { accepted: None });
    example.push(Command::StartContext { description: None });
    example.push(Command::StartTestcase {
        description: Message::String("case 2".to_owned()),
    });
    example.push(Command::StartTest {
        expected: "SOMETHING".to_owned(),
        format: None,
        description: None,
        channel: None,
    });
    example.push(Command::CloseTest {
        generated: "ELSE".to_owned(),
        accepted: None,
        status: Status {
            r#enum: StatusEnum::Wrong,
            human: "Wrong".to_owned(),
        },
    });
    example.push(Command::CloseTestcase { accepted: None });
    example.push(Command::CloseContext { accepted: None });
    example.push(Command::CloseTab { badge_count: None });
    example.push(Command::CloseJudgement {
        accepted: None,
        status: None,
    });
    example
}

#[test]
fn deserialize_example() {
    let result: Vec<Command> = serde_json::from_str(include_str!("example.json")).unwrap();
    assert_eq!(result, example());
}

#[test]
fn roundtrip_example() {
    let example = example();
    let result: Vec<Command> =
        serde_json::from_str(&serde_json::to_string(&example).unwrap()).unwrap();
    assert_eq!(result, example);
}
