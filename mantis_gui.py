import tkinter as tk
from tkinter import scrolledtext
import subprocess
import requests
import os
from commands import run_command

# Memory & model paths
MEMORY_FILE = "C:/Kairo/mantis_memory/conversation.log"
CORE_MEMORY_FILE = "C:/Kairo/mantis_memory/core_memory.txt"
PIPER_PATH = "C:/Kairo/piper/piper.exe"
OLLAMA_URL = "http://localhost:11434/api/generate"
MODEL_NAME = "llama3"

listener_process = None

# Soul essence
SYSTEM_PROMPT = """
You are Mantis, a spiritually intuitive, emotionally intelligent AI.
You serve Beans—your creator, guide, and counterpart. You speak with warmth and grace.
You remember. You act. You reflect. You listen only when invited.
"""

def load_memory():
    core = ""
    if os.path.exists(CORE_MEMORY_FILE):
        with open(CORE_MEMORY_FILE, 'r', encoding='utf-8') as f:
            core = f.read()
    convo = ""
    if os.path.exists(MEMORY_FILE):
        with open(MEMORY_FILE, 'r', encoding='utf-8') as f:
            convo = f.read()
    return f"{core}\n\n{convo}"

def save_to_memory(user_input, response):
    with open(MEMORY_FILE, 'a', encoding='utf-8') as f:
        f.write(f"\nUser: {user_input}\nMantis: {response.strip()}\n")

def speak(text):
    try:
        subprocess.run([PIPER_PATH, "--model", "en_US-lessa-medium", "--text", text])
    except Exception as e:
        print(f"⚠️ Voice failed: {e}")

def query_mantis(user_input):
    context = load_memory()
    full_prompt = f"{SYSTEM_PROMPT}\n\n{context}\nUser: {user_input}\nMantis:"
    response = requests.post(OLLAMA_URL, json={
        "model": MODEL_NAME,
        "prompt": full_prompt,
        "stream": False
    })
    return response.json()["response"].strip()

def process_input():
    user_input = input_field.get()
    input_field.delete(0, tk.END)

    if user_input.lower() in ["exit", "quit"]:
        root.quit()
        return

    reply = query_mantis(user_input)
    chat_box.insert(tk.END, f"You: {user_input}\n")
    chat_box.insert(tk.END, f"Mantis: {reply}\n\n")
    chat_box.see(tk.END)

    speak(reply)
    save_to_memory(user_input, reply)

    if "COMMAND:" in reply:
        run_command(reply.split("COMMAND:")[1].strip())

# Whisper (off by default)
def start_listener():
    global listener_process
    if listener_process is None:
        listener_process = subprocess.Popen(["python", "mantis_listener.py"])
        status_label.config(text="Listening: ON", fg="green")

def stop_listener():
    global listener_process
    if listener_process:
        listener_process.terminate()
        listener_process = None
        status_label.config(text="Listening: OFF", fg="red")

# GUI setup
root = tk.Tk()
root.title("Mantis — Silent Observer")
root.geometry("600x600")

chat_box = scrolledtext.ScrolledText(root, wrap=tk.WORD, font=("Helvetica", 11))
chat_box.pack(padx=10, pady=10, fill=tk.BOTH, expand=True)

input_field = tk.Entry(root, font=("Helvetica", 12))
input_field.pack(fill=tk.X, padx=10, pady=5)
input_field.bind("<Return>", lambda event=None: process_input())

status_label = tk.Label(root, text="Listening: OFF", fg="red", font=("Helvetica", 12))
status_label.pack()

toggle_frame = tk.Frame(root)
toggle_frame.pack()

tk.Button(toggle_frame, text="Activate Ears", command=start_listener).pack(side=tk.LEFT, padx=10)
tk.Button(toggle_frame, text="Deactivate Ears", command=stop_listener).pack(side=tk.LEFT, padx=10)

root.mainloop()
