# Names
David O’Sullivan (davidOSUL), Drew Kersnar (dakersnar), Simon Lothschutz (simonlothschutz)

# Project concept
“Context Switch” is a focus application 
(examples of this include: https://heyfocus.com/, https://freedom.to/, https://github.com/SelfControlApp/selfcontrol). 
The end goal is to allow users to block out certain websites/apps in scheduled blocks of time (“contexts”) 
via whitelists/blacklists, and to provide a means to automatically switch between these contexts. 
For example, a student might want to spend 60 minutes working on their english paper, 
10 minutes relaxing, and 60 minutes working on a coding assignment. 
They would use context switch to set up a schedule for three contexts -- 
the first that blocks out all websites except dictionary.com and docs.google.com for 60 minutes, 
followed by a free block for 10 minutes, followed by a 60 minute block allowing only clion app, 
stackoverflow, and google.com (or alternatively, depending on who the user is, allowing for example, 
every website except facebook, reddit, and twitter). When they hit start, context switch will 
automatically switch between these blocks when time is appropriate. Ideally users would be able to 
save blocks (e.g. “writing” or “coding” and re-use them), and also would be able to interface with the 
application in different ways (i.e. a UI/terminal app, or via the app looking at, for instance, their icloud calendar). 

# What we hope to accomplish in this iteration 
- Working on Mac OS
- Allow for blocking of just websites (rather than both websites and apps)
- Allow for one interface -- via a text file
- Stretch goals:
  - Allow for saving of blocks for future use
  - notifications when a “context switch” occurs
  - Allow for additional interface via icloud iCal calendar
  - Get time from NTP server to prevent user from being able to disable by modifying the system time 

# Spec

The project will provide a backend with a developer API to allow for multiple front-end use-cases. In this way we can allow, for example, both icloud cal data, or a text file (and potentially in the future an actual UI) to make use of the same API. 


## Basic Backend API
note: API subject to change, and not necessarily fully complete. This is meant to provide an overview of what the flow for using the application might look like
 
### Scheduler -- manages blocks of scheduled blockages
```Rust 
/// creates a new Scheduler object
Scheduler::new() -> Self 

/// Block contains info on blacklist/whitelist, time scheduled, duration, etc. 
/// Will return an error if block overlaps with another block and would have an inconsistency 
/// (e.g. the first has a blacklist and the second has a whitelist). 
/// Returns uid for referencing later. 
Scheduler::addBlock(&self, b: Block) -> Result<int, Error> 

Scheduler::removeBlock(&self, id: int) -> Result<(), Error>

/// What websites should be blocked at the provided time according to the scheduler
Scheduler::getBlockList(t : currTime) -> &[Website] 
```
### Block -- contains blacklist/whitelist, time start, time end
```Rust
Block::from_blacklist(blacklist : &[Website], time_start: time, time_end : time) -> Self

Block::from_whitelist(whitelist : &[Website], time_start: time, time_end : time) -> Self

/// for stretch goal, Allows for saving of blocks for later
Block::serialize<T : Write>(writer : &mut T) 

Block::from_serialized<T: Read>(read : t) -> Result<Block, Error>
```

### Runner -- the object that actually works to block out websites given a schedule
```Rust
/// creates a new runner object. Takes ownership of scheduler
Runner::new(s : Scheduler, blocker: WebsiteBlocker) -> Self

/// Will block websites as needed at the appropriate time according to the blocks that reside within it. 
/// This most likely will be done by spawning a thread to periodically poll 
/// owned scheduler for the block list. Will fail if already running.
Runner::start(&self) -> Result<(), Error>

/// It is up to the respective front-end to decide if this should be allowable to users, but this is an exposed method regardless. WIll fail if not running. 
Runner::stop() -> Result<(), Error>

/// for stretch goal -- make use of something akin to the observer pattern to allow for notifications 
Runner::addObserver(o : &Observer) -> () 
```

### WebsiteBlocker trait
There are many different potential techniques for blocking websites, so this will be a trait, that we can implement.
```Rust

/// Object to handle blocking/unblocking of websites. 
/// In future iterations this would be a uniform API that can do different things depending on the OS. for now we are focused just on mac. 
WebsiteBlocker::new() -> Self

WebsiteBlocker::block(w : &Website) 

WebsiteBlocker::unblock(w : &Website)
```


## Potential Website Blocking Mechanisms
There are several ways we could do this. Two possible ones are:
1. Modify the hosts file (as described in https://www.geeksforgeeks.org/website-blocker-using-python/ 
    - This is the mechanism we will start with
2. Modify the packet firewall (as described here: https://blog.neilsabol.site/post/quickly-easily-adding-pf-packet-filter-firewall-rules-macos-osx/) 

Self control app for example does both of these


## Run from text file
There will be a standard text file format that user can pass into application and will make the appropriate API calls. As an example:
“Time: 3/14/2020 3:45-4:45; Blacklist: facebook.com, TIme: 3/14/2020 5:00-6:00; Whitelist: google.com”

## Icloud front-end (python) (for stretch goal)
We will use an existing python library: https://github.com/picklepete/pyicloud  to access icloud data. Ideally we would do this in rust, but it may not be time feasible.

Users will provide the name of the calendar tag to use to identify events. In the “notes” section of the event they will write “blacklist: [...]” or “whitelist: [...]”


