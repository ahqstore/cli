from cffi import FFI
from pathlib import Path
from requests import get

import importlib.metadata

import os
import platform
import sys

ffi = FFI()

ffi.cdef("""
  char* get_ver();
  void init_args();
  void add_arg(char*);
  void node_entrypoint(bool);
""")

ver = importlib.metadata.version("ahqstore-cli")

def rust_target():
  os = platform.system().lower()
  arch = platform.machine().lower()

  if os == "windows":
    if arch == "i686":
      return "i686-pc-windows-msvc"
    elif arch in ("x86_64", "amd64"):
      return "x86_64-pc-windows-msvc"
    elif arch == "aarch64":
      return "aarch64-pc-windows-msvc"
  elif os == "darwin":
    if arch in ("x86_64", "amd64"):
      return "x86_64-apple-darwin"
    elif arch == "aarch64":
      return "aarch64-apple-darwin"
  elif os == "linux":
    if arch == "i686":
      return "i686-unknown-linux-gnu"
    elif arch in ("x86_64", "amd64"):
      return "x86_64-unknown-linux-gnu"
    elif arch == "armv7l":
      return "armv7-unknown-linux-gnueabihf"
    elif arch == "aarch64":
      return "aarch64-unknown-linux-gnu"
  
  # Error out for unsupported systems
  print(f"Error: Unsupported platform: {os} {arch}")
  sys.exit(1)

def get_prefix_suffix():
  os = platform.system().lower()
  prefix = ""
  suffix = ""
  
  if os == "windows":
    suffix = ".dll"
  elif os == "darwin":
    prefix = "lib"
    suffix = ".dylib"
  elif os == "linux":
    prefix = "lib"
    suffix = ".so"
  else:
    # Default to a generic UNIX-like system
    prefix = "lib"
    suffix = ".so"

  return (prefix, suffix)

def dlib():
  (prefix, suffix) = get_prefix_suffix()

  return f"{prefix}ahqstore_cli_rs{suffix}"

dylib = Path.home().joinpath("ahqstore-py")

if not dylib.exists():
  dylib.mkdir()

dylib = dylib.joinpath(dlib())

def dwnl():
  (prefix, suffix) = get_prefix_suffix()

  return f"https://github.com/ahqstore/cli/releases/download/{ver}/{prefix}ahqstore_cli_rs-{rust_target()}{suffix}"

def dwn():
  url = dwnl()

  resp = get(url)

  resp.raise_for_status()

  with open(dylib, "wb") as f:
    f.write(resp.content)

def main():
  C = None

  try:
    C = ffi.dlopen(str(dylib))

    ver_ptr = C.get_ver()

    version = ffi.string(ver_ptr)

    if not version == bytes(ver, "utf-8"):
      raise BufferError("This was an error comapring them")
  except:
    dwn()
    C = ffi.dlopen(str(dylib))

  C.init_args()

  for item in sys.argv[1:]:
    arg = ffi.new("char[]", bytes(item, "ascii"))

    C.add_arg(arg)

  C.node_entrypoint(
    os.environ.get("CI") == "true"
  )