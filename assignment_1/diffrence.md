# Diffrence Between Multi Threading and Async

Multi-threading and Async is type of concurrent programming where different parts of a program executes indepedently or sometimes dependently. It's also known as parallel programming which is becoming increasingly important as more new era computers (CPU) take advantage of their multiple processors and threads.

## Multi-threading

* In multi-threaded workflows we assign some tasks to a thread. The thread will be excuted synchronously. A thread can do multiple task.
    * As shown in Example - 1 : There are two threads first is main thread and second spawn thread that I created by executing the program you can see both the threads running indepedently and performing task on their own but since main thread is excuted before the spawn thread it exits before even finishing spawn thread.

* Multi-threaded workflows depends on CPU and Memory overheads.

* Threads executes at the moment, it can be call directly.

* Spawing and Switching between threads is more of CPU extensive processing even idle threads consume system resources.

* Multi-threading works well with CPU Bounds workloads which helps in parallel programming. 

## Async

* In asynchronous workflows we can assign many tasks to a single thread or main thread here some tasks can be dependent on the results of other tasks, as each task completes it invokes the code that schedules the next task that can run, given the results of the just-completed task.
    * As shown in Example - 2 : A client orders breakfast to a restaurant which have only one cook at the momment (One Thread) client ask him for coffee and toast so the cook start the timer to brew the coffee which we call Subtask 1 then leaving the coffee machine he begins to toast the bread which we call Subtask 2 to serve the complete order which is Final task the cook gets ready both things simultaneously but the final task is dependent on both task to be completed first and there is continues task which is independent from the other tasks.

* Asynchoronus workflows depends on OS threads.

* Async functions can't be called directly, instead of actually running it returns the futures (which is executed result of the function).

* Async provides significantly reduced CPU and memory overhead because uses a small amount of (expensive) threads to handle a large amount of (cheap) tasks.

* Async concurrency is a good match with the IO Bounds workloads.