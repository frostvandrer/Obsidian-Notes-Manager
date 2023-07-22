# Obsidian-Notes-Manager

This is a simple tool written in `Rust` which creates a simple notes structure when starting a new machine on [Hack The Box](https://www.hackthebox.com/).

The current structure is:

```
<MACHINE DIRECTORY>./00 - Credentials
<MACHINE DIRECTORY>./10 - Nmap
<MACHINE DIRECTORY>./20 - Enumeration
<MACHINE DIRECTORY>./30 - Initial Foothold
<MACHINE DIRECTORY>./40 - Post-exploitation Enumeration
<MACHINE DIRECTORY>./50 - Privilege Escalation
```

## How to install

```powershell
git clone https://github.com/frostvandrer/Obsidian-Notes-Manager.git
cd notes_manager
cargo build --release
```

## How to run

Example:

```powershell
C:\<PATH TO BINARY>\notes_manager.exe new easy Test
C:\<PATH TO OBSIDIAN VAULTS\Vaults\Hacking\Labs\HackTheBox\Boxes\Easy\Test
        ./00 - Credentials
        ./10 - Nmap
        ./20 - Enumeration
        ./30 - Initial Foothold
        ./40 - Post-exploitation Enumeration
        ./50 - Privilege Escalation
```

Since it can be painfull to go to the release directory everytime you want to run the tool, you can create a bat file and add it to a directory that is in the PATH:

```batch
"C:\<PATH TO REPO>\Obsidian-Notes-Manager\notes_manager\target\release\notes_manager.exe" %1 %2 %3
```

Name it e.g. `htb`, then just:

```powershell
htb new easy Test
```

## TODO

- [] Add credentials template functionality