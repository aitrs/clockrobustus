# ClockRobustus : A Super lightweight clock widget

## Build

Most of the things in this cargo workspace can be build using the cargo build --release command   
However, to build the front-end part, you'll have to run cargo tauri build (don't forget to install
the Tauri framework before).

## Run

ClockRobustus has a client/server architecture, with the two bounded by a lib holding every shared code the 
two have in common.   

The clockrobustus executable (in target/debug or target/release) holds the graphics, while the clockrobustusd daemon
is basically a sort of time server. Both have to be running to work, and they communicate through zmq (see the
libclockrobustus documentation for more details, and possible environment tweaks to suit your needs).

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


