# page-mon
Periodically check on web pages and convey any updates

### Planning:
Probably going to write it in Rust. It would not be hard to write something similar in Python.

The gist of it is that every so often, it will visit a web page and record the html. If there are html differences between the new one and the previous copy, it will diff them to try to discern the relevant changes, and then contact the user with that info. 
Ideally it will run as a daemon or service or something and will be able to check different sites with different frequencies. Maybe there'll be functionality to log into a website or something, but that's probably beyond the scope of the project. 
Useful crates look like [hyper](https://crates.io/crates/hyper), [diff](https://crates.io/crates/diff), and [unix-daemonize](https://crates.io/crates/unix-daemonize); I've also had an eye on [Pushjet](https://pushjet.io/) for a while, and I might try to use it for notifications.  

Edit: It might be cool to use [an actual parser crate](https://crates.io/crates/lalrpop) to parse a cron-like config file. Is a cron config file LR(1)? 


### Parsing:
I decided to use [lalrpop](https://crates.io/crates/lalrpop) to define a grammar for the config file that works very similarly to [crontab](http://linux.die.net/man/5/crontab) because it seemed interesting and powerful and fun to say "lalrpop". 
I decided against implementing [non-standard special character](https://en.wikipedia.org/wiki/Cron#Non-Standard_Characters) support (i.e. `L`,`W`,`?`,`#`) because it lowered my odds of ever completing this project and would probably make a mess of the code.
The new grammar is mostly based on the cron spec defined in the man page but is more flexible in a few areas (I don't believe cron uses an attribute grammar). 
It took longer to figure out lalrpop and construct the grammar/AST than it would have to just mimic cron functionality using string manipulations, but this way was more fun and the result is probably better. 
The attribute grammar is defined in [croncfg.lalrpop](src/croncfg.lalrpop) (which lalrpop will compile into croncfg.rs) and the AST is in [ast.rs](src/ast.rs). 
Here is the context-free grammar used to parse the cron-like config file, where a dagger (`†`) indicates that a feature of lalrpop was used to simplify otherwise verbose constructs via regular expressions or macros. 

```
    Line        →   Command  |  Comment
  † Comment     →   "#.*"
    Command     →   Time Url
    Time        →   Entry Entry Entry Entry_Month Entry_Day  |  Nickname
    Entry_Month →   Entry  |  MonthOfYear
    Entry_Day   →   Entry  |  DayOfWeek
  † Entry       →   Value ("," Value)*
    Value       →   ContVal  |  ContVal Skip
    ContVal     →   "*"  |  Num  |  Num "-" Num
    Skip        →   "/" Num
  † Num         →   "[\d]+"
    Nickname    →   "@yearly"  |  "@annually"  |  "@monthly"  |  "@weekly"  |  "@hourly"  |  "@daily" 
  † Url         →   "https?://[\S]+"
    MonthOfYear →   "JAN"  |  "FEB"  |  "MAR"  |  ...  |  "DEC"
    DayOfWeek   →   "SUN"  |  "MON"  |  "TUE"  |  ...  |  "SAT"
```





