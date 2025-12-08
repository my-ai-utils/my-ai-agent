use crate::*;

#[derive(Debug, Clone, Copy)]
pub enum LlmModel {
    Gpt4o(Gpt4Settings),
    Gpt4oMini(Gpt4Settings),
    Gpt5(Gpt5Settings),
    Gpt5Mini(Gpt5Settings),
    Gpt5Nano(Gpt5Settings),
    Qwen3_30bA3b(QwenSettings),
    ZaiGlm4_5(ZaiSettings),
    ZaiGlm4_5Air(ZaiSettings),
    ZaiGlm4_5X(ZaiSettings),
    ZaiGlm4_6(ZaiSettings),
    FireworksZaiGlm4_6(ZaiSettings),
}

impl LlmModel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Gpt4o(_) => "gpt-4o",
            Self::Gpt4oMini(_) => "gpt-4o-mini",
            Self::Gpt5(_) => "gpt-5",
            Self::Gpt5Mini(_) => "gpt-5-mini",
            Self::Gpt5Nano(_) => "gpt-5-nano",
            Self::Qwen3_30bA3b(_) => "Qwen/Qwen3-30B-A3B",
            Self::ZaiGlm4_5(_) => "glm-4.5",
            Self::ZaiGlm4_5Air(_) => "glm-4.5-air",
            Self::ZaiGlm4_5X(_) => "glm-4.5-x",
            Self::ZaiGlm4_6(_) => "glm-4.6",
            Self::FireworksZaiGlm4_6(_) => "accounts/fireworks/models/glm-4p6",
        }
    }

    pub fn is_gpt_5(&self) -> bool {
        match self {
            LlmModel::Gpt5(_) => true,
            LlmModel::Gpt5Mini(_) => true,
            LlmModel::Gpt5Nano(_) => true,
            _ => false,
        }
    }
    pub fn is_qwen3(&self) -> bool {
        match self {
            LlmModel::Qwen3_30bA3b(_) => true,
            _ => false,
        }
    }

    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }
    pub fn as_settings(&self) -> SettingsMode {
        match self {
            LlmModel::Gpt4o(settings) => SettingsMode::Gpt4(*settings),
            LlmModel::Gpt4oMini(settings) => SettingsMode::Gpt4(*settings),
            LlmModel::Gpt5(gpt5_settings) => SettingsMode::Gpt5(*gpt5_settings),
            LlmModel::Gpt5Mini(gpt5_settings) => SettingsMode::Gpt5(*gpt5_settings),
            LlmModel::Gpt5Nano(gpt5_settings) => SettingsMode::Gpt5(*gpt5_settings),
            LlmModel::Qwen3_30bA3b(settings) => SettingsMode::Qwen(*settings),
            LlmModel::ZaiGlm4_5(settings) => SettingsMode::Zai(*settings),
            LlmModel::ZaiGlm4_5Air(settings) => SettingsMode::Zai(*settings),
            LlmModel::ZaiGlm4_5X(settings) => SettingsMode::Zai(*settings),
            LlmModel::ZaiGlm4_6(settings) => SettingsMode::Zai(*settings),
            LlmModel::FireworksZaiGlm4_6(settings) => SettingsMode::Zai(*settings),
        }
    }

    pub fn is_qwen_think(&self) -> Option<bool> {
        match self.as_settings() {
            SettingsMode::Gpt4(_) => None,
            SettingsMode::Gpt5(_) => None,
            SettingsMode::Qwen(qwen_settings) => Some(qwen_settings.think),
            SettingsMode::Zai(_) => None,
        }
    }
}

pub enum SettingsMode {
    Gpt4(Gpt4Settings),
    Gpt5(Gpt5Settings),
    Qwen(QwenSettings),
    Zai(ZaiSettings),
}
