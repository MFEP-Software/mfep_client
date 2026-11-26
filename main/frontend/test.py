import tkinter as tk
from pathlib import Path
from tkinter import messagebox
import mfep_client

BASE_DIR = Path(__file__).resolve().parents[2]
CONFIG_PATH = str(BASE_DIR / "conf" / "config.yaml")

def load_settings():
    try:
        ip, port = mfep_client.get_network_config(CONFIG_PATH)
        ip_entry.delete(0, tk.END)
        ip_entry.insert(0, ip)
        port_entry.delete(0, tk.END)
        port_entry.insert(0, str(port))
    except RuntimeError as e:
        messagebox.showerror("Ladefehler", str(e))

def save_settings():
    new_ip = ip_entry.get().strip()
    port_text = port_entry.get().strip()

    if not port_text.isdigit():
        messagebox.showwarning("Ungültige Eingabe", "Der Port muss eine Zahl sein (z. B. 8080).")
        return

    try:
        msg = mfep_client.set_network_config(CONFIG_PATH, new_ip, int(port_text))
        status_label.config(text=msg, fg="green")
    except RuntimeError as e:
        messagebox.showerror("Speicherfehler", str(e))

def send_cmd():
    cmd = cmd_entry.get().strip()
    if not cmd:
        return
    try:
        msg = mfep_client.send_command(CONFIG_PATH, cmd)
        status_label.config(text=msg, fg="green")
    except RuntimeError as e:
        status_label.config(text=str(e), fg="red")

root = tk.Tk()
root.title("test")
root.geometry("360x280")
root.padx = 15

# netconf
settings_frame = tk.LabelFrame(root, text="Verbindungseinstellungen", padx=10, pady=10)
settings_frame.pack(fill="x", padx=15, pady=10)

tk.Label(settings_frame, text="IP-Adresse:").grid(row=0, column=0, sticky="w", pady=3)
ip_entry = tk.Entry(settings_frame, width=22)
ip_entry.grid(row=0, column=1, padx=5, pady=3)

tk.Label(settings_frame, text="Port:").grid(row=1, column=0, sticky="w", pady=3)
port_entry = tk.Entry(settings_frame, width=22)
port_entry.grid(row=1, column=1, padx=5, pady=3)

save_btn = tk.Button(settings_frame, text="Speichern", command=save_settings)
save_btn.grid(row=2, column=1, sticky="e", pady=5)

# send
cmd_frame = tk.LabelFrame(root, text="Steuerung", padx=10, pady=10)
cmd_frame.pack(fill="x", padx=15, pady=5)

cmd_entry = tk.Entry(cmd_frame, width=22)
cmd_entry.pack(side="left", padx=(0, 5))

send_btn = tk.Button(cmd_frame, text="Senden", command=send_cmd)
send_btn.pack(side="left")

# status
status_label = tk.Label(root, text="Bereit", fg="gray")
status_label.pack(pady=10)

# load
load_settings()
root.mainloop()
