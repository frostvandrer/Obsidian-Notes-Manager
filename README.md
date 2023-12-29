# Obsidian-Notes-Manager

This is a simple tool written in `Rust` which creates a simple notes structure when starting a new machine on [Hack The Box](https://www.hackthebox.com/) or [Vulnlab](https://www.vulnlab.com/).

The current structure is:

```
<MACHINE DIRECTORY>./00 - Credentials
<MACHINE DIRECTORY>./10 - Nmap
<MACHINE DIRECTORY>./20 - Enumeration
<MACHINE DIRECTORY>./30 - User
<MACHINE DIRECTORY>./40 - Root
```

## How to install

```
git clone https://github.com/frostvandrer/Obsidian-Notes-Manager.git
cd Obsidian-Notes-Manager\notes_manager
cargo build --release
```

## Usage

```
Usage: notes_manager.exe <COMMAND> <LAB> <LEVEL> <MACHINE_NAME>

Arguments:
  <COMMAND>       Command (only supporting "new" for now)
  <LAB>           HTB/VL
  <LEVEL>         Level (easy, medium, hard, insane)
  <MACHINE_NAME>  Machine name

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## How to run

Example:

```
C:\<PATH TO BINARY>\notes_manager.exe new htb easy Test
C:\<PATH TO OBSIDIAN VAULTS\Vaults\Hacking\Labs\HackTheBox\Boxes\Easy\Test
        ./00 - Credentials
        ./10 - Nmap
        ./20 - Enumeration
        ./30 - User
        ./40 - Root
```

Since it can be painful to go to the release directory every time you want to run the tool, you can create a bat file and add it to a directory that is in the PATH:

```batch
"C:\<PATH TO REPO>\Obsidian-Notes-Manager\notes_manager\target\release\notes_manager.exe" %1 %2 %3 %4
```

Name it e.g. `labs`, then just:

```powershell
labs new htb easy Test
```

## TODO
- [x] Add support for Vulnlab
- [ ] Add credentials template functionality