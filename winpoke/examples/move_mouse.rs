use winpoke::window::msg::input::{Input, MouseMessage};

fn main() {
    let (x, y) = winpoke::monitor::MonitorInfo::primary_monitor()
        .unwrap()
        .map_to_virtual_screen(0, 0);
    
    Input::send_seq(&vec![
        Input::Mouse(MouseMessage::MoveTo(x, y)),
        Input::Mouse(MouseMessage::RightClick),
        Input::Mouse(MouseMessage::Move(10, 10)),
        Input::Mouse(MouseMessage::LeftClick),
    ])
    .unwrap();
}
