use enigo::{Enigo, KeyboardControllable};
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    loop {
        // 创建 Enigo 实例
        let mut enigo = Enigo::new();

        // 获取需要按下的键
        print!("请输入需要按下的键（例如 A): ");
        io::stdout().flush().unwrap();
        let mut key_input = String::new();
        io::stdin().read_line(&mut key_input).expect("无法读取输入");

        // 检查输入是否为空
        if key_input.trim().is_empty() {
            println!("无效的输入，请重新输入。");
            continue;
        }

        // 提取输入的第一个字符
        let key_to_press = enigo::Key::Layout(key_input.trim().chars().next().unwrap());

        // 获取按键持续时间
        print!("请输入按键持续时间（秒）: ");
        io::stdout().flush().unwrap();
        let mut duration_input = String::new();
        io::stdin().read_line(&mut duration_input).expect("无法读取输入");

        // 检查输入是否为空或无效
        let press_duration: u64 = match duration_input.trim().parse() {
            Ok(duration) => duration,
            Err(_) => {
                println!("无效的输入，请重新输入。");
                continue;
            }
        };

        // 获取按键间隔
        print!("请输入按键间隔时间（毫秒）: ");
        io::stdout().flush().unwrap();
        let mut interval_input = String::new();
        io::stdin().read_line(&mut interval_input).expect("无法读取输入");

        // 检查输入是否为空或无效
        let interval: u64 = match interval_input.trim().parse() {
            Ok(interval) => interval,
            Err(_) => {
                println!("无效的输入，请重新输入。");
                continue;
            }
        };

        // 执行按键循环
        let start_time = std::time::Instant::now();
        while start_time.elapsed().as_secs() < press_duration {
            // 模拟按下和释放键
            enigo.key_down(key_to_press.clone());
            enigo.key_up(key_to_press.clone());

            // 间隔一段时间再次按键
            sleep(Duration::from_millis(interval));
        }

        // 显示运行完成
        println!("运行完成");

        // // 询问是否继续下一轮
        // println!("是否继续下一轮？(y/n): ");

        // io::stdout().flush().unwrap();
        // let mut continue_input = String::new();
        // io::stdin().read_line(&mut continue_input).expect("无法读取输入");

        // // 如果用户输入不是 'y'，则退出循环
        // if continue_input.trim() != "y" {
        //     break;
        // }
        break;
    }
}