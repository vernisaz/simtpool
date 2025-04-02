# Simple Thread Pool (simtpool)

## Purpose 

A thread pool to use in Rust applications

## Usage

Create a pool
```
extern crate simtpool;

let tpool = simtpool::ThreadPool::new(4); // size of the pool
```

Add a code for execution
```
tp.execute(move || {
    eprintln!{"request from {:?}", stream.peer_addr()}
    handle_connection(stream)
});
```
## Build
Checkout [simscript](https://github.com/vernisaz/simscript). If you did it in a directory sibling to the current,
then just type **rb**. You may need to edit **bee.7b** if you use a different directories structure.

## Microlibrary
This crate uses a consept of microlibray described in the [article](https://www.linkedin.com/pulse/micro-libraries-vs-mega-dmitriy-rogatkin-q6e6c).