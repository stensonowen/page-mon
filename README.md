# page-mon
Periodically check on your favorite web pages and alert you with new info

### Planning:
Probably going to write it in Rust, just for fun.

The gist of it is that every so often, it will visit a web page and record the html. If there are html differences between this one and the previous copy, it will diff them to try to discern the relevant changes, and then contact the user with that info. Ideally it will run as a daemon or service or something, and will be able to check different sites with different frequencies. Maybe there'll be functionality to log into a website or something, but that's probably beyond the scope of the project. Useful crates look like [hyper](https://crates.io/crates/hyper), [diff](https://crates.io/crates/diff), and [unix-daemonize](https://crates.io/crates/unix-daemonize); I've had an eye on [Pushjet](https://pushjet.io/) for a while, and I might try to use it for notifications.  

Edit: It might be cool to use [an actual parser crate](https://crates.io/crates/lalrpop) to parse a cron-like config file. Is a cron config file LR(1)? I didn't think so, but now I'm not sure. 
Unfortunately I don't think too much relevant functionality can be packed into the attribute grammar, but instead it would just have to generate a useful struct

### Parsing:
So I decided to use [lalrpop](https://crates.io/crates/lalrpop) to define a grammar for the config file that works very similarly to cron because it seemed interesting and powerful and fun to say "lalrpop". It was initially going to be pretty complicated, but I simplified it without removing much of the interesting bit by dropping certain special characters (`L`, `W`, `?`, and `#`) because I was afraid of getting caught down a rabbit hole. The new grammar is mostly based on the cron spec defined [here](http://linux.die.net/man/5/crontab), but is more flexible in a few areas (probably because adding flexibility is relatively pleasant when you're just defining an attribute grammar). It took longer to figure out lalrpop and construct the grammar/AST than it would have to just mimic cron functionality using string manipulations, but this way was more fun and the result is probably better. Here is the context-free grammar I'm using to parse the cron-like config file: 

```
    Line        →   Command  |  Comment
  † Comment     →   "#.*"
    Command     →   Time Url
    Time        →   Entry Entry Entry Entry_Month Entry_Day  |  Nickname
    Entry_Month →   Entry  |  MonthOfYear
    Entry_Day   →   Entry  |  DayOfWeek
  † Entry       →   Value (, Value)*
    Value       →   ContVal  |  ContVal Skip
    ContVal     →   "*"  |  Num  |  Num "-" Num
    Skip        →   "/" Num
  † Num         →   "[\d]+"
    Nickname    →   "@yearly"  |  "@annually"  |  "@monthly"  |  "@weekly"  |  "@hourly"  |  "@daily" 
  † Url         →   "https?://[^\d\s]+"
    MonthOfYear →   "JAN"  |  "FEB"  |  "MAR"  |  ...  |  "DEC"
    DayOfWeek   →   "SUN"  |  "MON"  |  "TUE"  |  ...  |  "SAT"
```
where a dagger(†) indicates that a feature of LALRPOP was used to simplify matters with regular expressions or macros (though it would be trivial to revise the grammar to exclude them.




