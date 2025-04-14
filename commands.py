import subprocess
import os
import pyautogui
import webbrowser

def run_command(text):
    text = text.lower()

    if "open browser" in text:
        webbrowser.open("https://www.google.com")
    
    elif "close window" in text:
        pyautogui.hotkey('alt', 'f4')

    elif "open obs" in text:
        subprocess.Popen("C:/Program Files/obs-studio/bin/64bit/obs64.exe")

    elif "switch to overwatch" in text:
        subprocess.run([
            "powershell", 
            "-Command", 
            "Start-Process 'C:\\Program Files (x86)\\Overwatch\\Overwatch.exe'"
        ])

    elif "lock screen" in text:
        subprocess.run("rundll32.exe user32.dll,LockWorkStation")

    else:
        print(f"⚠️ No known command matched for: {text}")
