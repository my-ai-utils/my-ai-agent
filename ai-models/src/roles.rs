pub const SYSTEM_ROLE: &'static str = "system";
pub const USER_ROLE: &'static str = "user";

pub const ASSISTANT_ROLE: &'static str = "assistant";

pub const TOOL_ROLE: &'static str = "tool";

pub trait MessageRole {
    fn get_role(&self) -> &str;

    fn is_system(&self) -> bool {
        self.get_role() == SYSTEM_ROLE
    }

    fn is_user(&self) -> bool {
        self.get_role() == USER_ROLE
    }

    fn is_assistant(&self) -> bool {
        self.get_role() == ASSISTANT_ROLE
    }

    fn is_tool(&self) -> bool {
        self.get_role() == TOOL_ROLE
    }
}
