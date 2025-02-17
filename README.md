# Building the Project

## (1) Ensure dependencies are downloaded

```bash
git clone https://github.com/steinbergmedia/vst3sdk deps/vst3sdk --recursive
```

## (1a) Ensure build dependencies are installed
## Nix users can run `nix develop`

## Configure the project with CMake
```bash
cmake -B build

# You may wish to export compile commands for Language Server Integration
cmake -B build -DCMAKE_EXPORT_COMPILE_COMMANDS=ON

# You may wish to specify a specific backend
# For example, clang
cmake -B build -DCMAKE_C_COMPILER=$(which clang)
```

## Build the Project
```bash
# This may take a while the first time...
cmake --build build
```


# Running the Project
```bash
./build/bin/editorhost
```
