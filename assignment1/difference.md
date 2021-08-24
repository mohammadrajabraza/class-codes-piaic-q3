##  ASYNC.(SINGLE THREAD) vs MULTI-THREADING

Consider an analogy that you owned a dhabah hotel, on very first morning you have costumer with an 
order of one toast and full fry egg. Now think in the context of programming language, there are 
there ways to do:

- 1.   Sync. way: You first make full-fry and then toast.

- 2.   Async., single threaded way:  You start by cooking full-fry and toast simultaneously and set a timer
or each. While they are cooking, you may set the table for costumer. When the timer goes off turn 
off heat take out toast and egg serve to your first costumer.

- 3.   Async., multi-threaded way: Hire two cooks for each job, one cook will do full-fry and second will 
toast the bread. 

You can say multi-threading is a kind of async. 
# Think, Async. are Tasks and Threading as Workers.
In multi-threaded program/instructions you assign tasks to the workers, one worker for each task. 
Whereas, in async. Single threaded Program/instructions you have a schedule tasks and each task depends on the other tasks, as one task completed it calls the next schedule task and will use the result of completed task, while awaiting you can do something else. You need one worker to complete all listed tasks, but not one worker for each task.

# There are two types of workloads: 
- CPU bound (Processor bound) 
- IO bound 

For CPU bound processes, you may hire as many workers/threads for each processors available to you, assign one task to one worker and one processor. The job of processor is to complete the computation task quickly.
But for IO bound tasks that are not waiting on processor you are not supposed to assign workers.
