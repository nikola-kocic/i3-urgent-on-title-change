extern crate i3ipc;
use i3ipc::event::Event;
use i3ipc::event::inner::WindowChange;
use i3ipc::{I3EventListener, Subscription};
use i3ipc::reply::Node;

extern crate xcb;
extern crate xcb_util;
use xcb_util::icccm;

pub fn set_urgent(window_id: i32) {
    let (xcb_conn, _screen_num) = xcb::Connection::connect(None).unwrap();
    let hints = icccm::WmHints::empty().is_urgent().build();
    let window: xcb::Window = window_id as xcb::Window;
    let cookie: xcb::VoidCookie = icccm::set_wm_hints_checked(&xcb_conn, window, &hints);
    cookie.request_check().unwrap();
}

fn on_title_change(node: Node) {
    if node.focused || node.urgent {
        return;
    }

    if let Some(win_id) = node.window {
        let name = node.name.unwrap_or_else(|| String::from("N/A"));
        println!("Setting urgent to: {}, {}", win_id, name);
        set_urgent(win_id);
    }
}

fn main() {
    let mut listener = I3EventListener::connect().unwrap();
    let result = listener.subscribe(&[Subscription::Window]).unwrap();
    assert!(result.success);
    let event_iterator = listener.listen();
    for event in event_iterator {
        if let Ok(Event::WindowEvent(info)) = event {
            if let WindowChange::Title = info.change {
                on_title_change(info.container);
            }
        }
    }
}
