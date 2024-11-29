use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    Label1 = <Label> {
        draw_text: {
            color: #000000
            text_style: {
                font_size: 12.2
            }
        }
    }
    Label2 = <Label> {
        draw_text: {
            color: #EFEFEF
            text_style: {
                font_size: 14
            }
        }
    }

    pub DockButton = <RadioButton> {
        width: Fill, height: 80

        draw_radio: {
            radio_type: Tab
        }

        draw_icon: {
            color: #FFAAFF
        }

        icon_walk: {width: 35, height: Fit}
    }
    pub SettingsSidebarButton = <RadioButton> {
        draw_text: {
            color: #FFFFFF
            text_style: {
                font_size: 14.
            }
        }
    }
    pub PictureDir = <Label1> {}
    pub CountNotice = <Label1> {}
    pub LogNotice = <Label2> {}
}
