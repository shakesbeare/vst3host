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

### For Unix-like Systems
```bash
cmake -B build
```

### For Windows with MSVC
In the Developer Powershell
```powershell
cmake -B build -G "Ninja"
```

## (4) Build the Project
```bash
# For windows, must in Developer Powershell
cmake --build build
```

# Running the Project
### Linux and MacOS
```bash
./build/editorhost
```

### Windows
```powershell
.\build\Debug\editorhost

```
