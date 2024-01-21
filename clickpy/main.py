import time
import pyautogui
import tkinter as tk
from tkinter import ttk
import threading


# 全局变量，用于标记是否终止程序
terminate_program = False
running_thread = None  # 用于跟踪运行的线程


def send_continuous_key(key: str, duration: int = 1, interval: float = 0.05) -> None:
    """
    模拟连续按键
    :param key: 要模拟的按键
    :param duration: 持续时间（秒）
    :param interval: 两次按键之间的时间间隔（秒）
    """
    global terminate_program
    end_time = time.time() + duration
    while time.time() < end_time and not terminate_program:
        pyautogui.press(key)
        time.sleep(interval)


# 后台运行程序的函数
def run_program() -> None:
    global terminate_program
    key = key_entry.get()
    duration = int(duration_entry.get())
    interval = float(interval_entry.get())
    terminate_program = False  # 重置终止标志
    send_continuous_key(key, duration=duration, interval=interval)


# 终止程序的函数
def terminate_program_func() -> None:
    global terminate_program
    terminate_program = True


# 创建主窗口
root = tk.Tk()
# root.iconphoto(True, tk.PhotoImage(file='./APP.png'))
root.title("键盘自动输入")

# 创建和布局GUI组件
key_label = ttk.Label(root, text="按键:")
key_label.grid(row=0, column=0, padx=10, pady=10, sticky="w")

key_entry = ttk.Entry(root)
key_entry.grid(row=0, column=1, padx=10, pady=10)

duration_label = ttk.Label(root, text="持续时间 (秒):")
duration_label.grid(row=1, column=0, padx=10, pady=10, sticky="w")

duration_entry = ttk.Entry(root)
duration_entry.grid(row=1, column=1, padx=10, pady=10)

interval_label = ttk.Label(root, text="间隔 (秒):")
interval_label.grid(row=2, column=0, padx=10, pady=10, sticky="w")

interval_entry = ttk.Entry(root)
interval_entry.grid(row=2, column=1, padx=10, pady=10)


# 添加开始按钮
def start_button_callback() -> None:
    global running_thread
    if running_thread and running_thread.is_alive():
        # 如果线程仍在运行，说明正在输入数字，不执行任何操作
        pass
    else:
        # 启动新线程运行程序
        running_thread = threading.Thread(target=run_program)
        running_thread.start()


start_button = ttk.Button(root, text="开始", command=start_button_callback)
start_button.grid(row=3, column=0, columnspan=2, pady=10)

# 添加终止按钮
terminate_button = ttk.Button(root, text="终止", command=terminate_program_func)
terminate_button.grid(row=4, column=0, columnspan=2, pady=10)

# 启动GUI主循环
root.mainloop()
