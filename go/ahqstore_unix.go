//go:build darwin || freebsd || linux || netbsd

package main

import (
	"syscall"

	"github.com/ebitengine/purego"
)

func openLibrary(name string) (uintptr, error) {
	return purego.Dlopen(name, purego.RTLD_NOW|purego.RTLD_GLOBAL)
}

func unloadLibrary(library uintptr) {
	purego.Dlclose(syscall.Handle(library))
}
