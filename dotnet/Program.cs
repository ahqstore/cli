using System.Reflection;
using System.Runtime.InteropServices;
using Microsoft.VisualBasic;
using ShellProgressBar;

struct PrefixSuffix
{
    public string prefix, suffix;

    public PrefixSuffix(string pre, string suf)
    {
        prefix = pre;
        suffix = suf;
    }

}

public class Program
{
    private static string GetArch()
    {
        switch (RuntimeInformation.ProcessArchitecture)
        {
            case Architecture.X86:
                return "i686";
            case Architecture.X64:
                return "x86_64";
            case Architecture.Arm:
                return "armv7";
            case Architecture.Arm64:
                return "aarch64";
            default:
                throw new Exception("Unsupported Platform");
        }
    }

    private static string GetTargetTriple()
    {
        var arch = GetArch();

        switch (Environment.OSVersion.Platform)
        {
            case PlatformID.Win32NT:
                return $"{arch}-pc-windows-msvc";
            case PlatformID.Unix:
                if (arch == "armv7")
                {
                    return "armv7-unknown-linux-gnueabihf";
                }

                return $"{arch}-unknown-linux-gnu";
            default:
                throw new Exception("We do not support this operating system.");
        }
    }

    static void Download(string path)
    {
        var a = GetPrefixSuffix();

        var version = Assembly.GetEntryAssembly()!
            .GetCustomAttribute<AssemblyInformationalVersionAttribute>()!
            .InformationalVersion;

        version = version.Split("+")[0];

        string url = $"https://github.com/ahqstore/cli/releases/download/{version}/{a.prefix}ahqstore_cli_rs-{GetTargetTriple()}{a.suffix}";

        using (var client = new HttpClient())
        {
            HttpResponseMessage response = client.GetAsync(url).Result;

            response.EnsureSuccessStatusCode();

            var length = response.Content.Headers.ContentLength;

            using ProgressBar pbar = new ProgressBar(100, "Downloading");

            // var file = File.Create(path);

            // stream.CopyToAsync(file);
            using (var stream = response.Content.ReadAsStreamAsync().Result)
            {
                var fileStream = File.Create(path);
                var buffer = new byte[81920]; // 80KB buffer
                long bytesRead = 0;
                int bytes;

                while ((bytes = stream.ReadAsync(buffer, 0, buffer.Length).Result) > 0)
                {
                    fileStream.Write(buffer, 0, bytes);
                    bytesRead += bytes;

                    pbar.AsProgress<float>().Report(
                        bytesRead / ((float)length!)
                    );

                    /// 10ms timeout
                    Thread.Sleep(10);
                }

                fileStream.Close();
            }
        }
    }

    private static PrefixSuffix GetPrefixSuffix()
    {
        switch (Environment.OSVersion.Platform)
        {
            case PlatformID.Win32NT:
                return new PrefixSuffix("", ".dll");
            case PlatformID.Unix:
                return new PrefixSuffix("lib", ".so");
            default:
                return new PrefixSuffix("lib", ".so");
        }
    }

    public static string GetDylibInfo()
    {
        var a = GetPrefixSuffix();
        var home = Environment.GetFolderPath(Environment.SpecialFolder.UserProfile);

        var dylibDir = $"{home}/ahqstore-dotnet";
        var dylib = $"{dylibDir}/{a.prefix}ahqstore_cli_rs{a.suffix}";


        if (!Path.Exists(dylib) || !Path.Exists(dylibDir))
        {
            try
            {
                FileSystem.MkDir(dylibDir);
            }
            catch (Exception)
            {

            }

            Download(dylib);
        }

        return dylib;
    }

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate IntPtr get_ver();

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate void init_args();

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate void add_arg(IntPtr str);

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate void node_entrypoint(bool ci);

    public static void Main(string[] args)
    {
        // This will auto download the binary too if needed
        string dylib = GetDylibInfo();

        nint? library;
        try
        {
            library = NativeLibrary.Load(dylib);
        }
        catch (Exception)
        {
            // Fix corruption
            Download(dylib);
            library = NativeLibrary.Load(dylib);
        }

        IntPtr functionPointer = NativeLibrary.GetExport((nint)library, "get_ver");
        get_ver get = Marshal.GetDelegateForFunctionPointer<get_ver>(functionPointer);
        var dll_ver = Marshal.PtrToStringAnsi(get());

        var version = Assembly.GetEntryAssembly()!
            .GetCustomAttribute<AssemblyInformationalVersionAttribute>()!
            .InformationalVersion;

        version = version.Split("+")[0];

        if (dll_ver != version)
        {
            NativeLibrary.Free((nint)library);

            Download(dylib);

            library = NativeLibrary.Load(dylib);
        }

        // Args init
        functionPointer = NativeLibrary.GetExport((nint)library, "init_args");
        init_args init = Marshal.GetDelegateForFunctionPointer<init_args>(functionPointer);
        init();

        // Push Args
        functionPointer = NativeLibrary.GetExport((nint)library, "add_arg");
        add_arg push = Marshal.GetDelegateForFunctionPointer<add_arg>(functionPointer);

        for (var i = 0; i < args.Length; i++)
        {
            string arg = args[i];

            nint ptr = Marshal.StringToHGlobalAnsi(arg);
            push(ptr);

            Marshal.FreeHGlobal(ptr);
        }

        functionPointer = NativeLibrary.GetExport((nint)library, "node_entrypoint");
        node_entrypoint start = Marshal.GetDelegateForFunctionPointer<node_entrypoint>(functionPointer);

        start(
            Environment.GetEnvironmentVariable("CI") == "true"
        );
    }
}
