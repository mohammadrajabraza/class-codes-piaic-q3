# Differentiate between Multi-threading and Async Programming in rust
## _Multithreading in Rust vs Async Programming in Rust_

# Multi-threaded programming: 
is an important area of Rust. It is not at all like multi-threaded programming in any other production language. Once your program compiles, it is free of data races. You can use threads in Rust whenever they make sense, just as you would decide to use a for loop. Channels and mutexes and arcs are not hard to get used to, and the language will catch whatever mistakes you make in the learning process.

# Asynchronous programming in Rust:
has its great importance due to its performance in certain cases. Operating system threads have certain kinds of overhead that make them unsuitable for certain sorts of work. On Linux, an operating system thread takes 15µs to start, and uses around 20KiB at minimum. Rust can start an asynchronous task fifty times faster than that, and it will use only as much memory as it needs to suspend itself at its await points. So for example, if your program is limited by the speed of task creation and per-task memory consumption, then you can go much farther on a given piece of hardware with Asynchronous code.

So in fact, asynchronous programming in Rust has been designed from the start to be used together with threads: you'll usually see a pool of threads all running asynchronous tasks. So an async task might migrate from one thread to another over the course of its execution, and will certainly run concurrently with other tasks. This is why crates like async-std and tokio include full suites of synchronization tools like:

- Channels
&
- Mutexes

## Example/Scenario

You are cooking in a restaurant. An order comes in for eggs and toast.
Synchronous: you cook the eggs, then you cook the toast.
Asynchronous, single threaded: you start the eggs cooking and set a timer. You start the toast cooking, and set a timer. While they are both cooking, you clean the kitchen. When the timers go off you take the eggs off the heat and the toast out of the toaster and serve them.
Asynchronous, multithreaded: you hire two more cooks, one to cook eggs and one to cook toast. Now you have the problem of coordinating the cooks so that they do not conflict with each other in the kitchen when sharing resources. And you have to pay them.

Threading is about workers; asynchrony is about tasks. In multithreaded workflows you assign tasks to workers. In asynchronous single-threaded workflows you have a graph of tasks where some tasks depend on the results of others; as each task completes it invokes the code that schedules the next task that can run, given the results of the just-completed task. But you (hopefully) only need one worker to perform all the tasks, not one worker per task.

## Conclusion

Asynchronous programming is a form of parallel programming that allows a unit of work to run separately from the primary application thread and then proceeds to notify the main thread if said work was executed successfully or not. The benefits of asynchronous code cannot be overemphasized. From better performance to enhanced responsiveness, asynchronous programming has in many ways improved the way we write code and consequently its quality.