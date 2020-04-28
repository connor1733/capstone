# Capstone
## Things TODO

### Stage 1 Chrome Exploit

- [x] Get stage 1 exploit working on chrome browser on x86-64 linux machine (only works on the javascript shell as of now) **difficulty -> easyish**
- [ ] Compile Chrome for ARM
- [ ] Compile d8 for ARM and test exploit on d8 shell
- [ ] Get stage 1 exploit working on chrome browser on actual arm phone (shell then browser) **difficulty  -> medium**

### Stage 2 Kernel Exploit

- [X] Find vulnerable kernel version for CVE-2019-2181 **difficulty -> easyish**
- [X] Compile it for arm android  **difficulty -> easy**
- [X]  Get it on the phone **difficulty -> mediumish?**
- [ ] Write/take exploit to get it working from adb shell **difficulty -> easy/medium/hard** <- depends on what you want
- [ ] Take working exploit and get it working from chrome **difficulty -> medium/hard**

### Android Implant

- [X] Set up a dummy native binary for android
- [X] Implement/Find library to do stego for the database
- [X] Ninja stuff
- [X] Post it to twitter

### C2 Server
- [X] Listen for phone and send exploit file
- [X] Listen again and send implant
- [ ] Pull stego-hidden data from Twitter API
- [ ] Undo the stego, leaving raw WhatsApp database
- [X] Parse WhatsApp database for messages
- [ ] Move C2 infrastructure off of localhost to an accessible IP address, craft link to the C2

