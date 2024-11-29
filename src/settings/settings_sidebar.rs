use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    use crate::shared::SettingsSidebarButton;

    pub SettingsSidebar = <View> {
        flow: Down
        width: 127, height: Fill
        padding: {top: 70}
        spacing: 150
        show_bg: true
        draw_bg: {
            color: #A1BFA1
        }

        appearance = <SettingsSidebarButton> {
            text: "Appearance"
        }
        notifications = <SettingsSidebarButton> {
            text: "Notifications"
        }
        preferences = <SettingsSidebarButton> {
            text: "Preferences"
        }
        about = <SettingsSidebarButton> {
            animator: {selected = {default: on}}
            text: "About"
        }
    }
}
