# Capstone
## Things TODO

### Stage 1 Chrome Exploit

- [x] Get stage 1 exploit working on chrome browser on x86-64 linux machine (only works on the javascript shell as of now) **difficulty -> easyish**
- [X] Compile Chrome for ARM
- [X] Compile d8 for ARM and test exploit on d8 shell
- [X] Get stage 1 exploit working on chrome browser on actual arm phone (shell then browser) **difficulty  -> medium**

### Stage 2 Kernel Exploit

- [X] Find vulnerable kernel version for CVE-2019-2181 **difficulty -> easyish**
- [X] Compile it for arm android  **difficulty -> easy**
- [X]  Get it on the phone **difficulty -> mediumish?**
- [X] Write/take exploit to get it working from adb shell **difficulty -> easy/medium/hard** <- depends on what you want
- [X] Take working exploit and get it working from chrome **difficulty -> medium/hard**

### Android Implant

- [X] Set up a dummy native binary for android
- [X] Implement/Find library to do encrypt
- [X] Ninja stuff
- [X] Exfil data

### C2 Server
- [X] Listen for phone and send exploit file
- [X] Listen again and send implant
- [X] Pull encrypted data
- [X] Undo the encryption, leaving raw WhatsApp database
- [X] Parse WhatsApp database for messages
- [X] Move C2 infrastructure off of localhost to an accessible IP address, craft link to the C2

