#[derive(Debug)]
#[allow(dead_code)]
struct Point {
    x: u64,
    y: u64,
}

enum Message {
    // ウィンドウのサイズを変更するためのバリアント
    Resize { width: u32, height: u32 },
    
    // 位置を移動するためのバリアント
    Move(Point),
    
    // メッセージをエコーするためのバリアント
    Echo(String),
    
    // 色を変更するためのバリアント
    ChangeColor(u8, u8, u8),
    
    // プログラムを終了するためのバリアント
    Quit,
}

#[derive(Debug)]
struct State {
    width: u64,
    height: u64,
    position: Point,
    message: String,
    // RGB color composed of red, green and blue.
    color: (u8, u8, u8),
    quit: bool,
}

impl State {
    fn resize(&mut self, width: u64, height: u64) {
        self.width = width;
        self.height = height;
    }

    fn move_position(&mut self, point: Point) {
        self.position = point;
    }

    fn echo(&mut self, s: String) {
        self.message = s;
    }

    fn change_color(&mut self, red: u8, green: u8, blue: u8) {
        self.color = (red, green, blue);
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn process(&mut self, _message: Message) {
        match _message {
            Message::Resize { width, height } => self.resize(width as u64, height as u64),
            Message::Move(point) => self.move_position(point),
            Message::Echo(s) => self.echo(s),
            Message::ChangeColor(r, g, b) => self.change_color(r, g, b),
            Message::Quit => self.quit(),
        }
    }
}

fn main() {
    // You can optionally experiment here.
    let mut state = State {
        width: 0,
        height: 0,
        position: Point { x: 0, y: 0 },
        message: String::from("Initial message"),
        color: (0, 0, 0),
        quit: false,
    };

    println!("Initial State: {:?}", state);

    state.process(Message::Resize { width: 800, height: 600 });
    println!("After Resize: {:?}", state);

    state.process(Message::Move(Point { x: 100, y: 150 }));
    println!("After Move: {:?}", state);

    state.process(Message::Echo(String::from("New message")));
    println!("After Echo: {:?}", state);

    state.process(Message::ChangeColor(128, 128, 128));
    println!("After ChangeColor: {:?}", state);

    state.process(Message::Quit);
    println!("After Quit: {:?}", state);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            width: 0,
            height: 0,
            position: Point { x: 0, y: 0 },
            message: String::from("hello world"),
            color: (0, 0, 0),
            quit: false,
        };

        state.process(Message::Resize {
            width: 10,
            height: 30,
        });
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Echo(String::from("Hello world!")));
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Quit);

        assert_eq!(state.width, 10);
        assert_eq!(state.height, 30);
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.message, "Hello world!");
        assert_eq!(state.color, (255, 0, 255));
        assert!(state.quit);
    }
}
