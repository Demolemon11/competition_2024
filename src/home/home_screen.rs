use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io;
use std::net::TcpStream;
use std::path::PathBuf;

use makepad_widgets::*;
use reqwest::blocking;
use tinify::sync::Tinify;

use super::picture_dirs::PictureDirsWidgetRefExt;

const TEST_URL: &str = "https://cn.bing.com/th?id=OHR.AssiniboineTS_ZH-CN9936042562_1920x1080.jpg";
const DIR: &str = "/Pictures/Tiny";

live_design! {
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    use crate::home::picture_dirs::PictureDirs;
    use crate::shared::LogNotice;

    API_CHECK_OK = dep("crate://self/resources/icons/api_check_ok.svg")
    API_CHECK_ERR = dep("crate://self/resources/icons/api_check_err.svg")
    NO_INTERNET = dep("crate://self/resources/icons/no_internet.svg")
    PROCESSING = dep("crate://self/resources/icons/processing.svg")



    pub HomeScreen = {{HomeScreen}} {
        visible: true
        padding: {top: 10., bottom: 10.}

        dock = <DockMinimal> {
            width: Fill,
            height: Fill,
            padding: 0,
            spacing: 0,

            root = Splitter {
                axis: Horizontal,
                align: FromA(300.0),
                a: t1
                b: t2
            }

            t1 = Tab {
                kind: left
            }

            t2 = Tab {
                kind: right
            }


            left = <View> {
                show_bg: true
                draw_bg: {
                    color: #CFCF9F
                }
                flow: Down
                align: {x: 0.5}
                spacing: 15

                <View> {
                    height: 40
                    spacing: 11.5
                    upload_folder = <Button> {
                        width: Fill, height: 40
                        text: "Upload Folder"
                        draw_text: {
                            color: #55AF55
                            text_style: {
                                font_size: 13.
                            }
                        }
                    }
                    upload_picture = <Button> {
                        width: Fill, height: 40
                        text: "Upload Pictures"
                        draw_text: {
                            color: #55AF55
                            text_style: {
                                font_size: 13.
                            }
                        }
                    }
                    clean_path = <Button> {
                        width: Fill, height: 40
                        text: "Clean Pictures"
                        draw_text: {
                            color: #55AF55
                            text_style: {
                                font_size: 13.
                            }
                        }
                    }
                }
                picture_dirs = <PictureDirs> {}
            }
            right = <View> {
                show_bg: true
                draw_bg: {
                    color: #9FCFCF
                }
                flow: Down
                spacing: 20
                align: {x: 0.5, y: 0.5}
                padding: {bottom: 30}

                api_key = <View> {
                    height: 190
                    spacing: 10
                    text_input = <TextInput> {
                        width: Fill
                        empty_message: "Type your API key here"
                        draw_text: {
                            color: #333333
                            text_style: {
                                font_size: 18.
                            }
                        }
                    }
                    button = <Button> {
                        width: 120
                        text: "Check"
                        draw_text: {
                            color: #0
                            text_style: {
                                font_size: 14
                            }
                        }
                    }
                }
                <View> {
                    flow: Overlay
                    align: {x: 0.5, y: 0.5}

                    no_internet = <View> {
                        visible: false
                        flow: Down
                        align: {x: 0.5, y: 0.5}
                        spacing: 15

                        <Icon> {
                            draw_icon: {
                                svg_file: (NO_INTERNET),
                                color: #333333
                            }
                            icon_walk: {width: 90}
                        }
                        <LogNotice> {
                            text: "Please check your internet."
                        }
                    }
                    api_check_ok = <View> {
                        align: {x: 0.5, y: 0.5}
                        visible: false
                        flow: Down
                        spacing: 15

                        <Icon> {
                            draw_icon: {
                                svg_file: (API_CHECK_OK),
                                color: #00DF00
                            }
                            icon_walk: {width: 100}
                        }

                        <LogNotice> {
                            text: "Enjoy your converting!"
                        }
                    }
                    api_check_err = <View> {
                        align: {x: 0.5, y: 0.5}
                        visible: false
                        flow: Down
                        spacing: 15

                        <Icon> {
                            draw_icon: {
                                svg_file: (API_CHECK_ERR),
                                color: #DF0000
                            }
                            icon_walk: {width: 70 }
                        }

                        <LogNotice> {
                            text: "Incorrect API key."
                        }
                    }
                    processing = <View> {
                        align: {x: 0.5, y: 0.5}
                        visible: false
                        flow: Down
                        spacing: 15

                        <Icon> {
                            draw_icon: {
                                svg_file: (PROCESSING),
                                color: #FFFFFF
                            }
                            icon_walk: {width: 70 }
                        }

                        <LogNotice> {
                            text: "Processing..."
                        }
                    }

                }
                convert_button = <View> {
                    visible: false
                    align: {x: 0.5, y: 1.}
                    button = <Button> {
                        text: "Convert"
                        draw_text: {
                            color: #FFFFFF
                            text_style: {
                                font_size: 30.
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct HomeScreen {
    #[deref]
    view: View,
    #[rust]
    api_key: String,
    #[rust]
    result_dir: String,
}

impl Widget for HomeScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if let Event::Signal = event {
            self.view(id!(convert_button))
                .set_visible_and_redraw(cx, true);
            self.view(id!(processing)).set_visible_and_redraw(cx, false);

            let dir_list = self.portal_list(id!(picture_dirs));
            dir_list.as_picture_dirs().clean_dirs();
            dir_list.redraw(cx);
        }

        self.match_event(cx, event);
        self.view.handle_event(cx, event, scope)
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl MatchEvent for HomeScreen {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.button(id!(api_key.button)).clicked(actions) {
            let api_key = self.text_input(id!(api_key.text_input)).text();

            if TcpStream::connect("8.8.8.8:53").is_err() {
                self.view(id!(no_internet)).set_visible_and_redraw(cx, true);
                self.view(id!(api_check_ok))
                    .set_visible_and_redraw(cx, false);
                self.view(id!(api_check_err))
                    .set_visible_and_redraw(cx, false);
                return;
            } else {
                self.view(id!(no_internet))
                    .set_visible_and_redraw(cx, false);
            }

            if !api_key.is_empty() {
                self.api_key = self.text_input(id!(api_key.text_input)).text()
            } else {
                return;
            }

            let mut test_picture =
                File::create(format!("{}/{}", self.result_dir, "test.jpg")).unwrap();

            let response = blocking::get(TEST_URL).unwrap();
            let mut content = io::Cursor::new(response.bytes().unwrap());

            io::copy(&mut content, &mut test_picture).unwrap();

            if Tinify::new()
                .set_key(&self.api_key)
                .get_client()
                .unwrap()
                .from_file(format!("{}/test.jpg", self.result_dir))
                .is_ok()
            {
                self.view(id!(api_check_err))
                    .set_visible_and_redraw(cx, false);
                self.view(id!(api_check_ok))
                    .set_visible_and_redraw(cx, true);
                self.view(id!(convert_button))
                    .set_visible_and_redraw(cx, true);
                self.view(id!(api_key)).set_visible_and_redraw(cx, false)
            } else {
                self.view(id!(api_check_err))
                    .set_visible_and_redraw(cx, true);
                self.view(id!(api_check_ok))
                    .set_visible_and_redraw(cx, false);
                self.view(id!(convert_button))
                    .set_visible_and_redraw(cx, false);
                return;
            }
        }

        let dir_list = self.portal_list(id!(picture_dirs));

        if self.button(id!(upload_folder)).clicked(actions) {
            if let Some(dir) = rfd::FileDialog::new().pick_folder() {
                dir_list
                    .as_picture_dirs()
                    .add_dirs(collect_images_path(&dir).into_iter().collect::<Vec<_>>());
                dir_list.redraw(cx)
            }
        }

        if self.button(id!(upload_picture)).clicked(actions) {
            if let Some(dirs) = rfd::FileDialog::new()
                .add_filter("pictures", &["png", "jpg", "jpeg", "JPG", "PNG", "JPEG"])
                .pick_files()
            {
                dir_list.as_picture_dirs().add_dirs(dirs);
                dir_list.redraw(cx)
            }
        }

        if self.button(id!(clean_path)).clicked(actions) {
            dir_list.as_picture_dirs().clean_dirs();
            dir_list.redraw(cx)
        }

        if self.button(id!(convert_button.button)).clicked(actions) {
            self.view(id!(convert_button))
                .set_visible_and_redraw(cx, false);

            self.view(id!(api_check_ok))
                .set_visible_and_redraw(cx, false);

            self.view(id!(processing)).set_visible_and_redraw(cx, true);

            let picture_dirs = self
                .portal_list(id!(picture_dirs))
                .as_picture_dirs()
                .get_dirs();

            log!("picture dirs: {:?}", picture_dirs);

            let api_key = self.api_key.clone();
            let result_dir = self.result_dir.clone();

            std::thread::spawn(move || {
                for dir in picture_dirs.into_iter() {
                    Tinify::new()
                        .set_key(&api_key)
                        .get_client()
                        .unwrap()
                        .from_file(&dir)
                        .unwrap()
                        .to_file(format!(
                            "{}/{}",
                            &result_dir,
                            dir.file_name().unwrap().to_str().unwrap()
                        ))
                        .unwrap();
                }
                makepad_platform::SignalToUI::set_ui_signal();
            });
        }
    }
    fn handle_startup(&mut self, _: &mut Cx) {
        let home_path = home::home_dir().unwrap();
        let home_dir = home_path.to_str().unwrap().to_string();
        let result_dir = format!("{}{}", home_dir, DIR);
        self.result_dir = result_dir;

        if !PathBuf::from(&self.result_dir).exists() && fs::create_dir(&self.result_dir).is_err() {
            panic!("Permission denied")
        }
    }
}

#[inline(always)]
fn collect_images_path(path: &PathBuf) -> HashSet<PathBuf> {
    let iter1 = match fs::read_dir(path) {
        Ok(read_dir) => read_dir
            .map(|entry| entry.unwrap().path())
            .filter(|item| {
                item.is_file()
                    && (item.to_str().unwrap().to_lowercase().ends_with(".jpg")
                        || item.to_str().unwrap().to_lowercase().ends_with(".png"))
            })
            .collect(),
        Err(_) => Vec::new(),
    };

    let iter2 = match fs::read_dir(path) {
        Ok(read_dir) => read_dir
            .map(|entry| entry.unwrap().path())
            .filter(|item| item.is_dir())
            .flat_map(|item| collect_images_path(&item).into_iter())
            .collect(),
        Err(_) => Vec::new(),
    };
    iter1.into_iter().chain(iter2).collect()
}
