# Building the Project

## (1) Ensure dependencies are downloaded (One-time only)

```bash
git clone https://github.com/steinbergmedia/vst3sdk deps/vst3sdk --recursive
```

## (1a) Ensure build dependencies are installed (One-time only)
```bash
sudo apt install libx11-dev pkgconfig
```
- Nix users can run `nix develop`

## (2) Configure the project with CMake (Whenever CMakeLists.txt is modified)
```bash
cmake -B build

# You may wish to export compile commands for Language Server Integration
cmake -B build -DCMAKE_EXPORT_COMPILE_COMMANDS=ON

# You may wish to specify a specific backend
# For example, clang
cmake -B build -DCMAKE_C_COMPILER=$(which clang)
```

## (3) Build the Project
```bash
# This may take a while the first time...
cmake --build build
```


# Running the Project
```bash
./build/bin/editorhost
```
