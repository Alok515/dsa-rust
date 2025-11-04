enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Click { x: i32, y: i32 }
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Click { x, y } => println!("clicked at x={}, y={}.", x, y),
    }
}

fn main() {
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;
    let press = WebEvent::KeyPress('x');
    let click1 = WebEvent::Click { x: 100, y: 250 };
    let click2 = WebEvent::Click { x: 0, y: 0 };

    inspect(load);
    inspect(unload);
    inspect(press);
    inspect(click1);
    inspect(click2);
}