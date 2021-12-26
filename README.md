# tokio chat server

implementation of [chat server with async rust and tokio](https://www.youtube.com/watch?v=4DqP57BHaXI) w/ join/leave broadcasts added

### running

```
$ cargo run
   Compiling chat v0.1.0 (/home/clifton/code/rust/chat)
    Finished dev [unoptimized + debuginfo] target(s) in 0.81s
     Running `target/debug/chat`
[::1]:58500 has has entered the chat
[::1]:58502 has has entered the chat
[::1]:58504 has has entered the chat
[::1]:58504: bye
[::1]:58504: baz
[::1]:58500: boop
[::1]:58502: beep
[::1]:58500 has left the chat
[::1]:58506 has has entered the chat
[::1]:58506: foooo
[::1]:58506 has left the chat
[::1]:58502 has left the chat
[::1]:58504 has left the chat
```

### connecting

```
❯ telnet localhost 8000
Trying ::1...
Connected to localhost.
Escape character is '^]'.
[::1]:58504 has has entered the chat
[::1]:58504: bye
[::1]:58504: baz
[::1]:58500: boop
beep
[::1]:58500 has left the chat
[::1]:58506 has has entered the chat
[::1]:58506: foooo
[::1]:58506 has left the chat

goodbye
Connection closed by foreign host.
```
