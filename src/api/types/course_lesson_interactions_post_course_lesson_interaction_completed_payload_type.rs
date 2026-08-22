pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostCourseLessonInteractionCompletedPayloadType {
    #[serde(rename = "course_lesson_interaction.completed")]
    CourseLessonInteractionCompleted,
}
impl fmt::Display for PostCourseLessonInteractionCompletedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::CourseLessonInteractionCompleted => "course_lesson_interaction.completed",
        };
        write!(f, "{}", s)
    }
}
