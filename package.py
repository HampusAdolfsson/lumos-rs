import os
import shutil

def main():
    build_backend()
    copy_assets()
    package_tauri()
    print("\n\nThe tauri executable and installer can be found here:\ntauri-app/target/release/")

def build_backend():
    os.chdir("backend")
    os.system("cargo build --release")
    os.chdir("..")

def copy_assets():
    os.makedirs("tauri-app/src-tauri/backend", exist_ok=True)
    shutil.copyfile("backend/target/release/lumos-rs.exe", "tauri-app/src-tauri/backend/lumos-rs-x86_64-pc-windows-msvc.exe")

def package_tauri():
    os.chdir("tauri-app")
    os.system("npm run tauri build")
    os.chdir("..")

if __name__ == "__main__":
    main()
