# Building the Project

## (1) Clone the repository 
```bash
git clone git@github.com/shakesbeare --recursive
```

## (2) Ensure system dependencies are installed
One or many of these may already be installed in a gui environment
```bash
# Ubuntu
sudo apt install libx11-dev pkg-config libfreetype6-dev libxcb-util-dev libxcb-cursor-dev libxcb-keysyms1-dev libxcb-xkb-dev libxkbcommon-dev libxkbcommon-x11-dev libcairo2-dev libpango1.0-dev libgtkmm-3.0-dev libsqlite3-dev
```
- Nix users can run `nix develop`

## (3) Configure the project with CMake (Whenever CMakeLists.txt is modified)
```bash
cmake -B build

# You may wish to export compile commands for Language Server Integration
cmake -B build -DCMAKE_EXPORT_COMPILE_COMMANDS=ON

# You may wish to specify a specific backend
# For example, clang
cmake -B build -DCMAKE_C_COMPILER=$(which clang)
```

## (4) Build the Project
```bash
# This may take a while the first time...
cmake --build build
```

# Running the Project
```bash
./build/editorhost
```
