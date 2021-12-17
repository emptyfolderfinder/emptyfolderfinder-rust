<h1 align="center">Empty-Folder-Finder-Rust</h1>

<p align="center">
  <img alt="Github top language" src="https://img.shields.io/github/languages/top/emptyfolderfinder/emptyfolderfinder-rust?color=56BEB8">

  <img alt="Repository size" src="https://img.shields.io/github/repo-size/emptyfolderfinder/emptyfolderfinder-rust?color=56BEB8">

  <img alt="License" src="https://img.shields.io/github/license/emptyfolderfinder/emptyfolderfinder-rust?color=56BEB8">
</p>

<hr>

<p align="center">
  <a href="#dart-about">About</a> &#xa0; | &#xa0; 
  <a href="#sparkles-features">Features</a> &#xa0; | &#xa0;
  <a href="#white_check_mark-requirements">Requirements</a> &#xa0; | &#xa0;
  <a href="#checkered_flag-starting">Starting</a> &#xa0; | &#xa0;
  <a href="#memo-license">License</a> &#xa0;
</p>

<br>

## :dart: About

This repository containes the implementation of an empty folder finder in
Rust.
It can be used to scan a folder on your hard drive for empty folders and
then delete them.

## :sparkles: Features

:heavy_check_mark: Find empty folders and list them\
:heavy_check_mark: Delete empty folders\
:heavy_check_mark: Also delete parent folder if directory would be empty

## :white_check_mark: Requirements

Before starting :checkered_flag:, you need to have 
[Git](https://git-scm.com) and [Rust](https://rust-lang.org) installed.

## :checkered_flag: Starting

```bash
# Clone this project
$ git clone https://github.com/emptyfolderfinder/emptyfolderfinder-rust

# Access
$ cd emptyfolderfinder-rust

# Build the project
$ cargo build

# Run the project
# PATH_TO_EXECUTABLE on linux should be ./emptyfolderfinder
# PATH_TO_EXECUTABLE on windows should be ./Debug/emptyfolderfinder.exe
$ ./target/debug/emptyfolderfinder [-d] <Path>

# -d: Delete the empty folders otherwise the empty directories will only
#     be printed to the console.
```

## :memo: License

This project is under license from MIT. For more details, see the [LICENSE](LICENSE) file.

Made with :heart: by <a href="https://github.com/philskat" target="_blank">philskat</a>

&#xa0;

<a href="#top">Back to top</a>
