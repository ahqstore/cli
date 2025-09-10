package main

import (
	"fmt"
	"io"
	"net/http"
	"os"
	"path"
	"runtime"

	"github.com/ebitengine/purego"
	"github.com/schollz/progressbar/v3"
)

const version = "0.10.9"

func main() {
	var bin = getBinary()

	var libahqstore, err = openLibrary(bin)

	if err != nil {
		Download(bin)
		libahqstore, err = openLibrary(bin)

		if err != nil {
			os.Exit(1)
		}
	}

	var get_ver func() string
	purego.RegisterLibFunc(&get_ver, libahqstore, "get_ver")

	if get_ver() != version {
		unloadLibrary(libahqstore)

		os.Remove(bin)

		Download(bin)
		libahqstore, err = openLibrary(bin)

		if err != nil {
			os.Exit(1)
		}
	}

	var init_args func()
	purego.RegisterLibFunc(&init_args, libahqstore, "init_args")

	init_args()

	var add_arg func(string)
	purego.RegisterLibFunc(&add_arg, libahqstore, "add_arg")

	for i := 1; i < len(os.Args); i++ {
		add_arg(os.Args[i])
	}

	var node_entrypoint func(bool)
	purego.RegisterLibFunc(&node_entrypoint, libahqstore, "node_entrypoint")

	node_entrypoint(
		os.Getenv("CI") == "true",
	)
}

func Download(file string) {
	var prefix, suffix = GetPrefixSuffix()
	var triple, _ = GetRustTargetTriple()

	var url = fmt.Sprintf("https://github.com/ahqstore/cli/releases/download/%s/%sahqstore_cli_rs-%s%s", version, prefix, triple, suffix)

	var req, _ = http.NewRequest("GET", url, nil)
	var resp, err = http.DefaultClient.Do(req)

	if err != nil {
		fmt.Println("Could not connect to download the required dynamic library")
		os.Exit(1)
	}
	defer resp.Body.Close()

	bar := progressbar.DefaultBytes(
		resp.ContentLength,
		"Downloading...",
	)

	defer bar.Finish()

	var f, _ = os.OpenFile(file, os.O_WRONLY|os.O_CREATE, os.ModePerm)
	defer f.Close()

	io.Copy(io.MultiWriter(f, bar), resp.Body)
}

func GetPrefixSuffix() (string, string) {
	switch runtime.GOOS {
	case "windows":
		return "", ".dll"
	case "darwin":
		return "lib", ".dylib"
	default:
		return "lib", ".so"
	}
}

func GetRustTargetTriple() (string, error) {
	// Use runtime.GOOS for the operating system and runtime.GOARCH for the architecture.
	switch runtime.GOOS {
	case "windows":
		switch runtime.GOARCH {
		case "386":
			return "i686-pc-windows-msvc", nil
		case "amd64":
			return "x86_64-pc-windows-msvc", nil
		case "arm64":
			return "aarch64-pc-windows-msvc", nil
		}
	case "darwin":
		switch runtime.GOARCH {
		case "amd64":
			return "x86_64-apple-darwin", nil
		case "arm64":
			return "aarch64-apple-darwin", nil
		}
	case "linux":
		switch runtime.GOARCH {
		case "386":
			return "i686-unknown-linux-gnu", nil
		case "amd64":
			return "x86_64-unknown-linux-gnu", nil
		case "arm":
			return "armv7-unknown-linux-gnueabihf", nil
		case "arm64":
			return "aarch64-unknown-linux-gnu", nil
		}
	}

	return "", fmt.Errorf("whoa there, this platform is not on the list: %s %s", runtime.GOOS, runtime.GOARCH)
}

func binName() string {
	var prefix, suffix = GetPrefixSuffix()

	return fmt.Sprintf("%sahqstore_cli_rs%s", prefix, suffix)
}

func getBinary() string {
	var dir, err = os.UserHomeDir()

	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	dir = path.Join(dir, "ahqstore-go")

	_, err = os.ReadDir(dir)

	if err != nil {
		os.MkdirAll(dir, os.ModePerm)
	}

	var file = path.Join(dir, binName())

	return file
}
