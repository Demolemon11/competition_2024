use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    use crate::home::home_screen::HomeScreen;
    use crate::settings::settings_screen::SettingsScreen;
    use crate::dock::dock_screen::DockScreen;

    App = {{App}} {
        ui: <Window> {
            body = {
                flow: Overlay
                <View> {
                    width: Fill, height: Fill

                    dock = <DockScreen> {}
                    pages = <View> {
                        flow: Overlay
                        width: Fill, height: Fill
                        home = <HomeScreen> {}
                        settings = <SettingsScreen> {}
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}
app_main!(App);

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty())
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        self.ui
            .radio_button_set(ids!(dock.home_tab, dock.settings_tab))
            .selected_to_visible(cx, &self.ui, actions, ids!(pages.home, pages.settings));
        self.ui
            .radio_button_set(ids!(
                pages.settings.settings_sidebar.appearance,
                pages.settings.settings_sidebar.notifications,
                pages.settings.settings_sidebar.preferences,
                pages.settings.settings_sidebar.about
            ))
            .selected_to_visible(
                cx,
                &self.ui,
                actions,
                ids!(
                    pages.settings.settings_content.appearance,
                    pages.settings.settings_content.notifications,
                    pages.settings.settings_content.preferences,
                    pages.settings.settings_content.about
                ),
            );
    }
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        crate::shared::live_design(cx);
        crate::dock::live_design(cx);
        crate::home::live_design(cx);
        crate::settings::live_design(cx)
    }
}
