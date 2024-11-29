use makepad_widgets::*;
use std::{collections::HashSet, path::PathBuf};

live_design! {
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    use crate::shared::PictureDir;
    use crate::shared::CountNotice;

    pub PictureDirs = {{PictureDirs}} {
        flow: Down
        width: Fill, height: Fill

        <PortalList> {
            picture_dir = <PictureDir> {
                text: ""
            }
            count_notice = <CountNotice> {
                text: ""
            }
            empty = <View> {
                height: 12
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct PictureDirs {
    #[deref]
    view: View,
    #[rust]
    picture_dirs: HashSet<PathBuf>,
}

impl Widget for PictureDirs {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope)
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let range = self.picture_dirs.len();
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, range + 1);

                let mut picture_dirs = self.picture_dirs.clone().into_iter().collect::<Vec<_>>();

                while let Some(item_id) = list.next_visible_item(cx) {
                    let item = if let Some(t) = picture_dirs.pop() {
                        let picture_dir = list.item(cx, item_id, live_id!(picture_dir));
                        picture_dir
                            .as_label()
                            .set_text_and_redraw(cx, t.to_str().unwrap());
                        picture_dir
                    } else if item_id == range {
                        list.item(cx, item_id, live_id!(empty))
                    } else if item_id == range + 1 {
                        let count_notice = list.item(cx, item_id, live_id!(count_notice));
                        count_notice
                            .as_label()
                            .set_text_and_redraw(cx, &format!("{} total pictures", range));
                        count_notice
                    } else {
                        continue;
                    };
                    item.draw_all(cx, scope)
                }
            }
        }
        DrawStep::done()
    }
}

impl PictureDirsRef {
    pub fn add_dirs(&mut self, picture_dirs: Vec<PathBuf>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.picture_dirs = picture_dirs
                .into_iter()
                .chain(inner.picture_dirs.clone())
                .collect::<_>();
        }
    }
    pub fn get_dirs(&self) -> Vec<PathBuf> {
        if let Some(inner) = self.borrow() {
            inner.picture_dirs.clone().into_iter().collect::<_>()
        } else {
            Vec::new()
        }
    }
    pub fn clean_dirs(&mut self) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.picture_dirs.clear();
        }
    }
}
