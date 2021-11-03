# Introduction

This document will explain how to build and deploy Rust application for a RasperryPi (ARM architecture)

<br/>

# Steps
## 1. Creating a Rust app (we laready hae one so we skip this step)
## 2. Craeting a deployment script
## 3. Cross-compiling the application

<br/>

# Creating a Rust Application
Since we already have one so we're skipping this step

<br/>

# Creating a deployment script
Since, it's a multistep process, and we'll be iterating over our application and each iteration involves at a minimum compiling on the development machine and running on the target board. So, it is useful to create an script with necessary commands to avoid typing multiple commands each time.

We'll create a text file in the root of our project and name it after it's purpose `deploy` (without any extension).

```bash
#!/bin/bash

set -o errexit
set -o nounset
set -o xtrace

cargo build --release
```

After that we can just run to execute file:
```
$ ./deploy
```

If you encounter with some execution related permissions error then make the script file executable with:

```
$ chmod +x ./deploy
```
Now you can run the script successfully.
<br/>
<br/>
At this point we have compiled our app in release mode. We can inspect this compiled binary with the following command: 
```
$ file ./target/release/actix_mongo_app
```
- `actix_mongo_app`: is the application name

which prints out something like
```
target/release/actix_mongo_app: ELF 64-bit LSB shared object, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, BuildID[sha1]=7d6ea5fc91dcf91b45d4cc5b2a5f180206487961, for GNU/Linux 3.2.0, with debug_info, not stripped
```

If you read this carefully, you'll find out that it is a 64-bit binary for x86_64 architecture. Let's try running this on Pi and see what happens

```bash
#!/bin/bash

set -o errexit
set -o nounset
set -o xtrace

readonly TARGET_HOST=pi@raspberrypi.local
readonly TARGET_PATH=/home/pi/actix_mongo_app
readonly SOURCE_PATH=./target/release/actix_mongo_app

cargo build --release
rsync ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}
ssh -t ${TARGET_HOST} ${TARGET_PATH}
```

Let me describe the elements of the above scripts before moving further
- `pi`: Username on raspberry pi
- `raspberrypi.local`: Hostname of raspberry pi
- `actix_mongo_app`: Our application name

