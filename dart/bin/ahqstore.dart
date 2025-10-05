import 'dart:ffi';
import 'dart:io';

import 'package:ffi/ffi.dart';
import 'package:http/http.dart';

const String version = "0.15.0";

String getUserHomeDirectory() {
  if (Platform.isLinux || Platform.isMacOS) {
    return Platform.environment['HOME'] ??
        (throw StateError('HOME environment variable not found.'));
  } else if (Platform.isWindows) {
    return Platform.environment['USERPROFILE'] ??
        (throw StateError('USERPROFILE environment variable not found.'));
  } else {
    throw UnsupportedError(
      'Getting user home directory is not supported on ${Platform.operatingSystem}.',
    );
  }
}

(String, String) getPrefixSuffix() {
  if (Platform.isWindows) {
    return ("", ".dll");
  } else if (Platform.isMacOS) {
    return ("lib", ".dylib");
  }

  // Assume UNIX
  return ("lib", ".so");
}

String getTargetTuple() {
  if (Platform.isWindows) {
    switch (Abi.current()) {
      case Abi.windowsArm64:
        return "aarch64-pc-windows-msvc";
      case Abi.windowsX64:
        return "x86_64-pc-windows-msvc";
      case Abi.windowsIA32:
        return "i686-pc-windows-msvc";
      default:
        throw UnsupportedError("Unsupported ABI");
    }
  } else if (Platform.isMacOS) {
    switch (Abi.current()) {
      case Abi.macosArm64:
        return "aarch64-apple-darwin";
      case Abi.macosX64:
        return "x86_64-apple-darwin";
      default:
        throw UnsupportedError("Unsupported ABI");
    }
  } else if (Platform.isLinux) {
    switch (Abi.current()) {
      case Abi.linuxX64:
        return "x86_64-unknown-linux-gnu";
      case Abi.linuxIA32:
        return "i686-unknown-linux-gnu";
      case Abi.linuxArm64:
        return "aarch64-unknown-linux-gnu";
      case Abi.linuxArm:
        return "armv7-unknown-linux-gnueabihf";
      default:
        throw UnsupportedError("Unsupported ABI");
    }
  }

  throw UnsupportedError("Unsupported ABI");
}

String getDylib() {
  var (prefix, suffix) = getPrefixSuffix();
  String home = getUserHomeDirectory();

  Directory dir = Directory("$home/ahqstore-dart");

  if (!dir.existsSync()) {
    dir.createSync(recursive: true);
  }

  String dlibPath = "$home/ahqstore-dart/${prefix}ahqstore_cli_rs$suffix";

  File dylib = File(dlibPath);

  if (!dylib.existsSync()) {
    download(dlibPath);
  }

  return dlibPath;
}

Future<void> download(String path) async {
  File dlib = File(path);

  try {
    dlib.deleteSync();
  } catch (e) {
    // We ignore this
  }

  dlib.createSync();

  var (prefix, suffix) = getPrefixSuffix();

  var url = Uri.parse(
    "https://github.com/ahqstore/cli/releases/download/$version/${prefix}ahqstore_cli_rs-${getTargetTuple()}$suffix",
  );

  Response resp = await get(url, headers: {"user-agent": "AHQ Store Client"});

  dlib.writeAsBytesSync(resp.bodyBytes);
}

typedef GetVerNative = Pointer<Utf8> Function();
typedef InitArgs = Void Function();
typedef AddArg = Void Function(Pointer<Utf8> ptr);
typedef Entrypoint = Void Function(Bool ci);

Future<void> main(List<String> arguments) async {
  String dylib = getDylib();

  DynamicLibrary? library;

  try {
    library = DynamicLibrary.open(dylib);

    var versionFn = library
        .lookup<NativeFunction<GetVerNative>>("get_ver")
        .asFunction<GetVerNative>();

    var data = versionFn().toDartString();

    if (data != version) {
      library.close();
      throw Error();
    }
  } catch (e) {
    download(dylib);
    library = DynamicLibrary.open(dylib);
  }

  var initFn = library
      .lookup<NativeFunction<InitArgs>>("init_args")
      .asFunction<void Function()>();
  initFn();

  var addArg = library
      .lookup<NativeFunction<AddArg>>("add_arg")
      .asFunction<void Function(Pointer<Utf8>)>();

  for (var arg in arguments) {
    var allocated = arg.toNativeUtf8();

    addArg(allocated);

    calloc.free(allocated);
  }

  var start = library
      .lookup<NativeFunction<Entrypoint>>("node_entrypoint")
      .asFunction<void Function(bool)>();

  start(String.fromEnvironment("CI", defaultValue: "false") == "true");
}
