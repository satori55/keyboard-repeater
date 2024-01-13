use enigo::{Enigo, KeyboardControllable};
use std::thread;
use std::time::Duration;

fn main() {
    // 创建 Enigo 实例
    let mut enigo = Enigo::new();

    //waiting
    let duration = Duration::from_secs(3);
    thread::sleep(duration);

    // 模拟按下和释放 'A' 键
    enigo.key_down(enigo::Key::Layout('A'));
    enigo.key_up(enigo::Key::Layout('A'));

    // 模拟按下和释放 'B' 键，带有一秒的延迟
    enigo.key_down(enigo::Key::Layout('B'));
    std::thread::sleep(std::time::Duration::from_secs(1));
    enigo.key_up(enigo::Key::Layout('B'));
}
