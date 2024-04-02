# Download dependencies for the project based on the host triplet
import platform
import os
import sys
import requests
from tqdm import tqdm
import zipfile

# Dependency versions
maafw_version = "v1.7.0-alpha.2"

def copy_file_recursively(src, dst, system):
    if os.path.isdir(src):
        if not os.path.exists(dst):
            os.makedirs(dst)
        for item in os.listdir(src):
            s = os.path.join(src, item)
            d = os.path.join(dst, item)
            copy_file_recursively(s, d, system)
    else:
        if system == "windows":
            os.system("copy " + src + " " + dst)
        else:
            os.system("cp " + src + " " + dst)

def download(url: str, fname: str, chunk_size=1024):
    resp = requests.get(url, stream=True)
    total = int(resp.headers.get('content-length', 0))
    with open(fname, 'wb') as file, tqdm(
        desc=fname,
        total=total,
        unit='iB',
        unit_scale=True,
        unit_divisor=1024,
    ) as bar:
        for data in resp.iter_content(chunk_size=chunk_size):
            size = file.write(data)
            bar.update(size)

def figure_triplet():
    supported_triplets = {
        "windows": ["x86_64","aarch64"],
        "linux": ["x86_64","aarch64"],
        # may add macos support in the future
    }

    host_system = platform.system().lower()
    host_arch = platform.machine().lower()

    if host_arch == "amd64":
        host_arch = "x86_64"

    if host_system not in supported_triplets:
        print("Error: Unsupported host system: ", host_system)
        sys.exit(1)

    if host_arch not in supported_triplets[host_system]:
        print("Error: Unsupported host architecture:, ", host_arch)
        sys.exit(1)

    return host_system, host_arch

def setup_deps_dir():
    deps_dir = "deps"
    if not os.path.exists(deps_dir):
        os.makedirs(deps_dir)

    return deps_dir

def setup_maafw_dynamic_libs(host_system, fw_dir):
    needed_libs = [
        "fastdeploy_ppocr_maa",
        "MaaAdbControlUnit",
        "MaaFramework",
        "MaaToolkit",
        "MaaUtils",
        "onnxruntime_maa.dll",
        "opencv_world_maa.dll"
    ]
    lib_dir = os.path.join(fw_dir, "bin")
    copy_file_recursively(lib_dir, os.path.join("tauri"), host_system)

def setup_maafw(host_system, host_arch, dst, force=False):
    print("Setting up MAA Framework")
    print("Downloading MAA Framework " + maafw_version)
    maafw_zip = os.path.join(dst, "maafw.zip")
    if os.path.exists(maafw_zip) and not force:
        print("MAA Framework already downloaded")
    else:
        maafw_base_url = "https://github.com/MaaAssistantArknights/MaaFramework/releases/download/"
        triplet_string = ("win" if host_system == "windows" else host_system) + "-" + host_arch
        maafw_url = maafw_base_url + maafw_version + "/MAA-" + triplet_string + "-" + maafw_version + ".zip"
        print("Downloading from " + maafw_url)
        download(maafw_url, maafw_zip)

    print("Extracting MAA Framework")
    extracted_path = os.path.join(dst, "maafw")
    with zipfile.ZipFile(maafw_zip, 'r') as zip_ref:
        zip_ref.extractall(extracted_path)

    print("Copying MAA Framework dynamic libraries")
    setup_maafw_dynamic_libs(host_system, extracted_path)


def main():
    # if -f is passed, force download
    force = False
    if len(sys.argv) > 1:
        if sys.argv[1] == "-f":
            force = True
    host_system, host_arch = figure_triplet()
    print("Setting up dependencies for " + host_system + "-" + host_arch)
    triplet_dir = setup_deps_dir()
    setup_maafw(host_system, host_arch, triplet_dir,force)

if __name__ == "__main__":
    main()