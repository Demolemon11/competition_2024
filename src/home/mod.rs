use makepad_widgets::Cx;

pub mod home_screen;
pub mod picture_dirs;

pub fn live_design(cx: &mut Cx) {
    home_screen::live_design(cx);
    picture_dirs::live_design(cx);
}
