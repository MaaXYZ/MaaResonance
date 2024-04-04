import toml
import os


def get_version_name():
    with open('tauri/Cargo.toml') as f:
        data = toml.load(f)
        return (data['package']['version'], data['package']['name'])


def build_single(name, version, arch, merge_conf):
    config_name = merge_conf.split(".")[0].split("\\")[-1]
    os.system(f"pnpm tauri build --config {merge_conf}")

    target_name = f"tauri/target/release/bundle/nsis/{name}_{version}_{arch}_{config_name}-setup.exe"
    if os.path.exists(target_name):
        os.remove(target_name)
    os.rename(f"tauri/target/release/bundle/nsis/{name}_{version}_{arch}-setup.exe",
              f"tauri/target/release/bundle/nsis/{name}_{version}_{arch}_{config_name}-setup.exe")


def build():
    version, name = get_version_name()
    arch = "x64"  # For now we only support x64 for simplicity
    configs = os.scandir("config/windows")
    for config in configs:
        build_single(name, version, arch, config.path)


if __name__ == "__main__":
    build()
