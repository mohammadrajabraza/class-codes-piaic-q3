# Multi-Theading
## Introduction

Multi-threading is an environment in which multiple independent (preferably- although dependent too with some extra effort) tasks can be rum simultaneously. In this working environment; different sections (or functions) of a main function is divided into multiple threads and those threads are processed parallel by multi-threaded CPU environment.

Multi-threading is used primiarly for CPU extensive scenarios.


## Pros

- Better utilization of the resources
- Time saving 

## Cons
- overhead in switching between threads
- overhead in sharing data between threads


## Example
Consider a scenario, a person has a cpu which supports multi-threading. He has to complete his scheduled work. He also wants to enjoy his time using music etc. He wants to perform both work simultaneously. Considering a non multi-threadin approach, he can either work or listen to music but not both. It can be comprehend by example 1 of Assignment. When work will be done only then person can listen to music in this case. But he wants to perform both tasks (work) simultaneously. For this, multi-threading approach can be used. Example 2 shows how to perfrom both work simultaneosuly using multi-threadin approach. Both threads needs 1 sec to complete. By using the time stamping, one can see that using multi-threading both threads are completed in 1 sec rather than 2 sec as seen in example 1. As each thread requires 1 sec to complete and we have two threads; so commulatively needs 2 secs but using multi-threading approach our work is done in 1 sec and both threads work simulateneously. This efficeint utilization of resource sharing is an edge over sequential coding / tasking.

# Async
## Introduction
In Async programming, multiple operations are run on a single thread. The difference with multi-threading is that; in multi-threading, multiple operations (tasks) are run by separate threads, but in async, there is only one thread but still it supports multiple task to run simultaneously.
In Rust, async creates a asynchronous functions which returns a future. To execte the body of the async functions, future must be run to completion.

Async is used primiarly for Input/Output extensive scenarios.

## Pros
- better resource utilization in a single thread 

## Cons


## Example
In Rust, async creates a asynchronous functions which returns a future. To execte the body of the async functions, future must be run to completion. For this we have different executors. One such is block_on. Block_on executes the future and it blocks the thread unless future is executed. the  This behaviour is shown in example 3 and 4. In example 3, future is not called, thus there is not desired output in the console. Whereas, in example 4, output can be seen on console using future.

Another executor is await. It doesn't blocks the thread and allows the other tasks to run if future is busy or unable to progress. The limitation of this executor is that it can be run inside an async function. Example 5 explains this scenario. A family is going on a vacation. They need to clean a car and packing. Husband is cleaning a car while listening a music and packing the stuff. Cleanig a car, listening to a music and packing stuff requires 2 sec each. If synchronous programming is used, then these tasks will be done in 6 sec however, if async programinh with await is used then this can be done in 4 sec by putting together clean a car and listening to music simultaneously. As block_on is also used, untill two tasks (cleaning a car and music listening) is not done, packing cann't be started as shown in the console output.
