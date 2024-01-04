use enigo::{Enigo, KeyboardControllable};

fn main() {
    // 创建 Enigo 实例
    let mut enigo = Enigo::new();

    // 模拟按下和释放 'A' 键
    enigo.key_down(enigo::Key::Layout('A'));
    enigo.key_up(enigo::Key::Layout('A'));

    // 模拟按下和释放 'B' 键，带有一秒的延迟
    enigo.key_down(enigo::Key::Layout('B'));
    std::thread::sleep(std::time::Duration::from_secs(1));
    enigo.key_up(enigo::Key::Layout('B'));
}
