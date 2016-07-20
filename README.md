# page-mon
Periodically check on your favorite web pages and alert you with new info

### Planning:
Probably going to write it in Rust, just for fun.

The gist of it is that every so often, it will visit a web page and record the html. If there are html differences between this one and the previous copy, it will diff them to try to discern the relevant changes, and then contact the user with that info. Ideally it will run as a daemon or service or something, and will be able to check different sites with different frequencies. Maybe there'll be functionality to log into a website or something, but that's probably beyond the scope of the project. Useful crates look like [hyper](https://crates.io/crates/hyper), [diff](https://crates.io/crates/diff), and [unix-daemonize](https://crates.io/crates/unix-daemonize); I've had an eye on [Pushjet](https://pushjet.io/) for a while, and I might try to use it for notifications.  
