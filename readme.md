# Pull

A very, very simple program that will download from a url and print the result to stdout. If a second argument is provided, it will instead write to a file.

Does not use any crates (even reqwest), just raw tcp sockets. Communicates over HTTP1.0, does not support HTTPS.

Purpose is to copy this over to a machine (e.g. using base64) when the machine doesnt have curl, wget etc. Note most machines have some programming language like node, python or perl to download files so this is for really rare cases.

By default, release builds are ~200kb or less on windows and linux. After building with `--release`, run UPX on the result to chop off another 100kb+: `upx --best --lzma target/release/pull`

It can be made smaller by altering how libstd is used, but at that point it might as well be written in C.