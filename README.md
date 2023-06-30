# ClockRobustus : A Super lightweight clock widget

## Build

Most of the things in this cargo workspace can be build using the cargo build --release command   
However, to build the front-end part, you'll have to run cargo tauri build (don't forget to install
the Tauri framework before).

### Dependencies

#### Linux

you will need pkgconfig. Pick one of the following lines to paste in your shell
```bash
sudo apt install pkg-config
sudo yum install pkg-config
sudo pacman -S pkg-config
```

you will also need libsoup 2.4
```bash
sudo apt install libsoup2.4-dev
sudo yum install libsoup
sudo pacman -S libsoup
```
   
Then you will need the rust toolchain
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
...and the tauri cli
```bash
cargo install tauri-cli
```
...and the node toolchain
```bash
sudo apt install node npm npx
```
(it is recommanded to use nvm to manage easily node versions)

#### Windows

Work in progress (but steps from the linux instructions are reproductible with some tweaking in windows powershell)

### Building

#### Time Server

First you need to build the time server

```bash
cargo build --release --package clockrobustusd
```

Then build the app
```bash
cd ./clockrobustus-app
npm i
cargo tauri build
cd ..
```

## Run

First make sure to run the time server. Le it run in it's on terminal so you can ctrl+c it when you're done testing
```bash
./target/release/clockrobustusd
```

Then run the front app 
```bash
./target/release/clockrobustus
```

It should work...

## Testing

The libclockrobustus project holds the most critical part of the software, hence you should check that everything is ok
by running cargo test in the lib folder.   
Note : Sometimes, env tests fails randomly. This will have to be fixed in the near future.

## Todo

Some features have not been implemented in the front-end for now. The server is able to distribute clock messages (intended
to provite numeric time and also angles for the heads) and alarm messages. Also, the libclockrobustus holds several handy
methods to store alarms in database. Sneeze and shutdown functionnality should be implemented for alarms (as they now "ring" for
only 30 seconds, and the only way to shut them down is to remove them).

In place of "ringing", the alarm blinks the face's color. So it would be a great improvement to add sound.

Also, the libclockrobustus holds a function named 'check_database_directory' in the 'lib.rs' file, that is not portable (it's unix-like only for now). Some improvements could be to handle this function better for portability.   

Most of the tests in the libclockrobustus come from the documentation examples, so it would be good to add more and more test, and also add a unified coverage process.

It is also crucial to add dockerfiles in the two executable projects and also configure CI/CD depending on the final devops platform.   


