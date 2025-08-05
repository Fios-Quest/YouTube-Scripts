Job Tracker 12 - Errors... again
================================

Start
-----

Preamble:

- Building a job tracker
  - Using Rust and Dioxus
- I will monetizing it (in case you want to help so you're not surprised later)
- Regarding stream: Chatting to a friend:
  - streams don't do what I want
  - Wanted to show, here's how you work on a real project
  - Actually that's not very interesting so been doing things like testing etc in the background
  - Job Tracker is coming to an end (or at least I'm nearly out of interesting things to do) so whats next?

Latest changes:

- [ ] Logs open correctly (on mac and windows)
  - [ ] We are likely to do away with the JSON nonesense TBH, haven't quite decided yet
- [ ] Reorganised UI crate to be clearer
- [ ] Leaned more heavily into the Partials which are now working nicely, much cleaner code, easy to drop in
  - [ ] At some point I want to rethink the entirely though, move logic out of compoments, rethink forms
- [ ] Moved the application context to its own crate
- [ ] Allow timestamps to be read (if you have older data this will allow it to be read again)
  - [ ] We might be able to expand on how we parse string date times
- [ ] Changed keyboard shortcuts so they can be applied to anything
- [ ] Some style changes, not quite made it into main yet

What's wrong with what we have now?
-----------------------------------

- [ ] Writing to an ever expanding log file
- [ ] Log file can crash the app itself *facepalm*
- [ ] Logs are off to one side both given more importance than they need and too little
- [ ] A lot of errors simply aren't handled right now and just crash the app
- [ ] JSON is completely unreadable (because I turn it into json for the log then leave it as a string in the app)
- [ ] Some errors we can't do much about right now
- [ ] Right now I don't know if errors leak anything (does it matter?)

Work
----

- [ ] Set up a signal to recieve errors
- [ ] Create a component to listen for errors
- [ ] Create macros to simplify error handling
  - [ ] Warning / Error / Irrecoverable Error?
- [ ] When to clear the log file?
- [ ] Potentially make our own error type to help us decide what gets displayed when
