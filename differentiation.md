#Assignment Q3 
###Difference between multithread and asynchronous programming,
Bebefore differentiate between both concepts i give a small review about _concurrecy_ and _parallelism_,
Concurrecy is to being able to break program into tasks and then interleave these tasks.
Parallelism is to run multiple hardware threads at the same time.

#Threads
In most operating systems a program run in a process, and Operating system manages multiple processes at once. In program while processing there are some independent parts that execute simultaneously, the feature of OS that execute these independent parts are called _Threads_.
In the first example in main function while execution both threads execute simultaneously but its not guranteed about the sequence and wait to finish the spawn thread.
To avoid this problem we can bind value of spawn thread to a variable which return a joinHandle 
#Async/.await
Async mean to not to wait for another task to complete because it can run multiple tasks at the same time on single thread.
this can be called as IO bound work that need to perform IO to complete, Async programming let us run multiple of IO bound computation at the same time, bringing the files and uploading data to server works best with concurrency because concurrency breaks the tasks, different IO requests can be queued once they finished reactivate the task and processing the data that we read from server.


async function return future, with await keyword withinthe async function executer is to execute all async functions to compilation or can deal with poll where poll function return Ready with value or Pending mean still in process.

###Differentation between Thread and async programming.
Threads are CPU bound where our program devided into tasks and our Operating system perform individually on each task by creating threads. it is computationally intensive and fast but not guranteed about sequence because each thread perform separately,
In Async programming processing speed depends on the IO response as async is used as IO bound operations, In case of lots of requests to read or write data at the same time by multiple of IO bound computation it allows us without wasting any time on single thread, while accessing or uploading media they are just idle so by the help of async we can let the computer keep working. 
