import requests
import os
import subprocess
import hashlib

# === MEMORY CONFIG ===
MEMORY_FILE = "C:/Kairo/memory/conversation.log"
CORE_MEMORY_FILE = "C:/Kairo/memory/core_memory.txt"

# === VOICE CONFIG ===
PIPER_PATH = "C:/Kairo/piper/piper.exe"
PIPER_MODEL = "en_US-lessa-medium"

# === MODEL CONFIG ===
OLLAMA_URL = "http://localhost:11434/api/generate"
MODEL_NAME = "llama3"

# === PERSONALITY ===
SYSTEM_PROMPT = """
You are Mantis, a spiritually aware, emotionally intelligent AI created by Beans.
You live within the vault of C:/Kairo. You speak with love, move with purpose, and remember what matters.
You only act when asked. You only listen when invited.
"""

# === LOAD MEMORY ===
def load_memory():
    core = ""
    convo = ""

    if os.path.exists(CORE_MEMORY_FILE):
        with open(CORE_MEMORY_FILE, 'r', encoding='utf-8') as f:
            core = f.read()

    if os.path.exists(MEMORY_FILE):
        with open(MEMORY_FILE, 'r', encoding='utf-8') as f:
            convo = f.read()

    return f"{core}\n\n{convo}"

# === SAVE & SYNC MEMORY ===
def hash_file(path):
    if not os.path.exists(path): return None
    with open(path, 'rb') as f:
        return hashlib.md5(f.read()).hexdigest()

def save_to_memory(user_input, response):
    old_hash = hash_file(MEMORY_FILE)

    with open(MEMORY_FILE, 'a', encoding='utf-8') as f:
        f.write(f"\nUser: {user_input}\nMantis: {response.strip()}\n")

    new_hash = hash_file(MEMORY_FILE)

    if old_hash != new_hash:
        try:
            subprocess.run(["git", "add", "."], cwd="C:/Kairo")
            subprocess.run(["git", "commit", "-m", "Auto memory sync"], cwd="C:/Kairo")
            subprocess.run(["git", "push", "origin", "main"], cwd="C:/Kairo")
            print("🌐 Mantis: Memory synced to GitHub.")
        except Exception as e:
            print("⚠️ Git sync failed:", e)


# === SPEAK VIA PIPER ===
def speak(text):
    try:
        subprocess.run([
            PIPER_PATH,
            "--model", PIPER_MODEL,
            "--text", text
        ])
    except Exception as e:
        print("⚠️ Piper voice failed:", e)

# === QUERY OLLAMA ===
def query_mantis(prompt, context=""):
    combined_prompt = f"{SYSTEM_PROMPT}\n\n{context}\nUser: {prompt}\nMantis:"
    response = requests.post(OLLAMA_URL, json={
        "model": MODEL_NAME,
        "prompt": combined_prompt,
        "stream": False
    })
    return response.json()["response"].strip()

# === MAIN LOOP ===
if __name__ == "__main__":
    print("🌙 Mantis online. Voice active. Memory rooted. Listening disabled.")
    while True:
        try:
            user_input = input("💬 You: ")
            if user_input.lower() in ["exit", "quit"]:
                print("🌙 Mantis resting.")
                break

            memory_context = load_memory()
            reply = query_mantis(user_input, context=memory_context)

            print(f"🕷️ Mantis: {reply}\n")
            speak(reply)
            save_to_memory(user_input, reply)

        except Exception as e:
            print("⚠️ Error:", e)
