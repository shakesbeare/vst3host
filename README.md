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
- Open folder with Visual Studio
- Build editorhost.exe

### For Windows with clang (still requires Visual Studio):

If you aren't sure you need this, you probably don't.
```powershell
winget install llvm
cmake -B build -G "Ninja" # May need to start a new terminal session
```

## (4) Build the Project
```bash
# For windows, must be in a Developer Powershell session
# This may take a while the first time...
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
