use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "command")]
#[serde(rename_all = "kebab-case")]
#[serde(rename_all_fields = "camelCase")]
pub enum Command {
    StartJudgement,
    StartTab {
        title: String,
        hidden: Option<bool>,
        permission: Option<Permission>,
    },
    StartContext {
        description: Option<Message>,
    },
    StartTestcase {
        description: Message,
    },
    StartTest {
        expected: String,
        format: Option<TestFormat>,
        description: Option<Message>,
        channel: Option<String>,
    },
    AppendMessage {
        message: Message,
    },
    AnnotateCode {
        row: Index,
        column: Option<Index>,
        text: String,
        external_url: Option<String>,
        r#type: Option<Severity>,
        rows: Option<Index>,
        columns: Option<Index>,
    },
    EscalateStatus {
        status: Status,
    },
    CloseTest {
        generated: String,
        accepted: Option<bool>,
        status: Status,
    },
    CloseTestcase {
        accepted: Option<bool>,
    },
    CloseContext {
        accepted: Option<bool>,
    },
    CloseTab {
        badge_count: Option<BadgeCount>,
    },
    CloseJudgement {
        accepted: Option<bool>,
        status: Option<Status>,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Permission {
    Student,
    Staff,
    Zeus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(untagged)]
pub enum Message {
    String(String),
    Object {
        format: MessageFormat,
        description: Description,
        permission: Permission,
    },
}

type MessageFormat = String;
type Description = String;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum TestFormat {
    Text,
    CSV,
}

type Index = u16;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Status {
    pub r#enum: StatusEnum,
    pub human: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum StatusEnum {
    InternalError,
    CompilationError,
    MemoryLimitExceeded,
    TimeLimitXxceeded,
    OutputLimitExceeded,
    RuntimeError,
    #[serde(alias = "wrong answer")]
    Wrong,
    #[serde(alias = "correct answer")]
    Correct,
}

type BadgeCount = u16;
