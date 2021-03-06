# Godot steam launcher
An application for launching Godot from Steam without the "usage time" bug.

# Why?
I created this application because when you download and install Godot from Steam, The usage time will not be recorded so this application stays opened for letting Steam record the time.

# How to install

**Note**: This application is only made for Windows, I don't have any idea if this works on other platforms.

1. Download [Godot](https://store.steampowered.com/app/404790/Godot_Engine/) from Steam
2. Download the latest binary [from releases](https://github.com/ElCosmoXD/godot-steam-launcher/releases)
3. Paste the downloaded binary on the Godot executable folder (You can find it on Steam, Right click to the Godot app and press "Search local files")
4. Copy the original Godot executable name (could be something like godot.windows.opt.tools.64.exe, godot.windows.opt.tools.32.exe and so on)
5. Rename the original godot executable to: "godot.exe"
6. Rename the "godot-steam-launcher.exe" executable to the original godot executable name (As mentioned above it could be something like godot.windows.opt.tools.64.exe, godot.windows.opt.tools.32.exe and so on).
7. Launch godot from Steam

# Compiling

1. Clone the repository (using ``` git clone ``` or downloading the project directly from github) 
2. Install the rust tools (cargo, rustc, etc)
3. Run ``` cargo build ``` on the main folder of the repository
