use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    use crate::settings::settings_sidebar::SettingsSidebar;
    use crate::settings::settings_content::SettingsContent;

    pub SettingsScreen = <View> {
        visible: false
        flow: Right
        width: Fill, height: Fill

        settings_sidebar = <SettingsSidebar> {}
        settings_content = <SettingsContent> {}
    }
}
