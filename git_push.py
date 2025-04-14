import subprocess

try:
    subprocess.run(["git", "add", "."], cwd="C:/Kairo")
    subprocess.run(["git", "commit", "-m", "Manual sync trigger"], cwd="C:/Kairo")
    subprocess.run(["git", "push", "origin", "main"], cwd="C:/Kairo")
    print("🌀 Mantis: Manual push complete.")
except Exception as e:
    print("⚠️ Push failed:", e)
