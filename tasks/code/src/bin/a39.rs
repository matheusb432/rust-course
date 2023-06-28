// Topic: Channels
//
// Summary:
//   Using the existing code, create a program that simulates an internet-of-things
//   remote control light bulb. The color of the light can be changed remotely.
//   Use threads and channels to communicate what color the light bulb should display.
//
// Requirements:
// * Create a separate thread representing the light bulb
// * Use a channel to communicate with the thread
// * Display a color change message using the println! macro
// * The light bulb must also be able to turn on and off
//   * Display whether the light is on or off on each color change
// * Turn off the light when disconnecting from it
//
// Notes:
// * Remember to add `crossbeam-channel` to your Cargo.toml file
// * Use the `colored` crate if you want to get fancy and display actual colors
// * The docs.rs site can be used to read documentation for third-party crates
// * Disconnection can be accomplished by dropping the sender, or
//   by telling the thread to self-terminate
// * Use `cargo test --bin a39` to test your program to ensure all cases are covered

use crossbeam_channel::{unbounded, Receiver};
use std::thread::{self, JoinHandle};

enum LightMsg {
    // Add additional variants needed to complete the exercise
    ChangeColor(u8, u8, u8),
    Toggle(LightStatus),
    Disconnect,
}

#[derive(Debug)]
enum LightStatus {
    On,
    Off,
}

fn spawn_light_thread(receiver: Receiver<LightMsg>) -> JoinHandle<LightStatus> {
    let light_thread = thread::spawn(move || {
        let mut status = LightStatus::Off;
        loop {
            match receiver.recv() {
                Ok(msg) => match msg {
                    LightMsg::Toggle(st) => {
                        println!("Light: Switching light to {st:?}");
                        status = st;
                    }
                    LightMsg::ChangeColor(r, g, b) => {
                        println!(
                            "Light: Switching color to rgb({r},{g},{b})\t| Light is {status:?}"
                        );
                    }
                    LightMsg::Disconnect => {
                        println!("Light: disconnecting...");
                        status = LightStatus::Off;
                        break;
                    }
                },
                Err(e) => {
                    status = LightStatus::Off;
                    println!("Light: disconnecting due to err: {e:?}");
                    break;
                }
            }
        }
        status
    });

    light_thread
}

fn main() {
    let (s, r) = unbounded();
    let light = spawn_light_thread(r);

    s.send(LightMsg::ChangeColor(200, 50, 0)).unwrap();
    s.send(LightMsg::Toggle(LightStatus::On)).unwrap();
    s.send(LightMsg::ChangeColor(250, 50, 50)).unwrap();
    s.send(LightMsg::ChangeColor(100, 0, 0)).unwrap();
    s.send(LightMsg::Toggle(LightStatus::Off)).unwrap();
    s.send(LightMsg::Toggle(LightStatus::On)).unwrap();
    s.send(LightMsg::Disconnect).unwrap();

    let light = light.join().unwrap();
    println!("Main: light status is {light:?}");
}

#[cfg(test)]
mod test {
    use super::*;
    use crossbeam_channel::unbounded;

    #[test]
    fn light_off_when_disconnect() {
        let (s, r) = unbounded();

        let light = spawn_light_thread(r);
        s.send(LightMsg::Disconnect).expect("channel disconnected");

        let light_status = light.join().expect("failed to join light thread");

        if let LightStatus::On = light_status {
            panic!("light should be off after disconnection");
        }
    }

    #[test]
    fn light_off_when_dropped() {
        let (s, r) = unbounded();

        let light = spawn_light_thread(r);
        drop(s);

        let light_status = light.join().expect("failed to join light thread");

        if let LightStatus::On = light_status {
            panic!("light should be off after dropping sender");
        }
    }
}
