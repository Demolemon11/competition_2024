use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub SettingsContent = <View> {
        flow: Overlay
        width: Fill, height: Fill
        align: {x: 0.5, y: 0.5}
        spacing: 10
        appearance = <View> {
            visible: false
            width: Fill, height: Fill
            align: {x: 0.5, y: 0.5}
            <Label> {
                text: "Appearance"
                draw_text: {
                    color: #00FFAA
                    text_style: {
                        font_size: 40.
                    }
                }
            }
        }
        notifications = <View> {
            visible: false
            width: Fill, height: Fill
            align: {x: 0.5, y: 0.5}
            <Label> {
                text: "Notifications"
                draw_text: {
                    color: #00FFAA
                    text_style: {
                        font_size: 40.
                    }
                }
            }
        }
        preferences = <View> {
            flow: Down
            visible: false
            width: Fill, height: Fill
            align: {x: 0.5}
            spacing: 200
            <Label> {
                text: "Preferences"
                draw_text: {
                    color: #00FFAA
                    text_style: {
                        font_size: 40.
                    }
                }
            }
            <View> {
                align: {x: 0.5}
                width: Fill, height: Fill
                spacing: 10
                <Label> {
                    text: "Choose Your Language"
                    draw_text: {
                        color: #99FF99
                        text_style: {
                            font_size: 19
                        }
                    }
                }
                <DropDown> {
                    labels: ["C", "C++", "Julia", "Zig", "Rust", "ObjC"]
                    values: [A, B, C, D, E, F]
                    draw_text: {
                        text_style: {
                            font_size: 20
                        }
                    }
                }
            }
        }
        about = <View> {
            visible: true
            flow: Down
            height: Fill
            align: {x: 0.5}
            padding: {top: 70}
            spacing: 60

            <Label> {
                text: "Last Update: 2024_1128"
                draw_text: {
                    text_style: {
                        font_size: 17
                    }
                    color: #000000
                }
            }
            <Label> {
                text: "Copyright Protection"
                draw_text: {
                    text_style: <THEME_FONT_BOLD>{
                        font_size: 13
                    }
                    color: #9FEFEF
                }
            }
        }
    }
}
