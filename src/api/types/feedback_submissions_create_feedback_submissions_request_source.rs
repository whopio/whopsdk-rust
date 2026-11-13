pub use crate::prelude::*;

/// What kind of submission this is. Use api_report_feedback for an issue or api_ask_question for an unanswered question. The other values are reserved for Whop's own clients: the hosted MCP server, the Whop CLI, and Whop AI chat.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateFeedbackSubmissionsRequestSource {
    McpReportFeedback,
    McpAskQuestion,
    CliReportFeedback,
    CliAskQuestion,
    AiChatReportFeedback,
    AiChatAskQuestion,
    ApiReportFeedback,
    ApiAskQuestion,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateFeedbackSubmissionsRequestSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::McpReportFeedback => serializer.serialize_str("mcp_report_feedback"),
            Self::McpAskQuestion => serializer.serialize_str("mcp_ask_question"),
            Self::CliReportFeedback => serializer.serialize_str("cli_report_feedback"),
            Self::CliAskQuestion => serializer.serialize_str("cli_ask_question"),
            Self::AiChatReportFeedback => serializer.serialize_str("ai_chat_report_feedback"),
            Self::AiChatAskQuestion => serializer.serialize_str("ai_chat_ask_question"),
            Self::ApiReportFeedback => serializer.serialize_str("api_report_feedback"),
            Self::ApiAskQuestion => serializer.serialize_str("api_ask_question"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateFeedbackSubmissionsRequestSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "mcp_report_feedback" => Ok(Self::McpReportFeedback),
            "mcp_ask_question" => Ok(Self::McpAskQuestion),
            "cli_report_feedback" => Ok(Self::CliReportFeedback),
            "cli_ask_question" => Ok(Self::CliAskQuestion),
            "ai_chat_report_feedback" => Ok(Self::AiChatReportFeedback),
            "ai_chat_ask_question" => Ok(Self::AiChatAskQuestion),
            "api_report_feedback" => Ok(Self::ApiReportFeedback),
            "api_ask_question" => Ok(Self::ApiAskQuestion),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateFeedbackSubmissionsRequestSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::McpReportFeedback => write!(f, "mcp_report_feedback"),
            Self::McpAskQuestion => write!(f, "mcp_ask_question"),
            Self::CliReportFeedback => write!(f, "cli_report_feedback"),
            Self::CliAskQuestion => write!(f, "cli_ask_question"),
            Self::AiChatReportFeedback => write!(f, "ai_chat_report_feedback"),
            Self::AiChatAskQuestion => write!(f, "ai_chat_ask_question"),
            Self::ApiReportFeedback => write!(f, "api_report_feedback"),
            Self::ApiAskQuestion => write!(f, "api_ask_question"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
