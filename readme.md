# Pull

A very, very simple program that will download from a url and print the result to stdout. If a second argument is provided, it will instead write to a file.

By default, release builds are ~1mb on windows and linux (slightly larger on linux, ~1.3). After building with --release, run UPX on the result to chop off another 500kb+: `upx --best --lzma target/release/pull`