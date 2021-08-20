We learned in quarter 2 how rust is surprisingly better programming language in performance and security. For the same reasons, Rust is being used assross a wide variety of organizations- ranging from experimental projects to production systems. In this assignment, after scouring through a score of web references, I will discuss the two facets of Rust that enhances the perofrmance side of this language, namely Multithreading and async features that makes up what is called concurrency. Concurrency in programming means that multiple tasks can be worked on at the same time. This is possible for even a single thread of execution by interleaving the work for different tasks in chunks rather than working on tasks as entire chunks.

One of the concurrency model is asynchronous programming, or aysnc for short. You can run a large number of concurrent tasks on a small number of OS threads through the async/await syntax.

While hoping through web pages the past few days, I came across terms and associated concepts related to async programming, e.g, generators, coroutines, futures/promises, parallelism etc. The more I try to contemplate understanding these concepts, the farther away I drift from what is all this about. If it was not for the time constrain, I would have copied (after understanding myself) all these paragraphs I have read but still don't understand fully, having little background in low-level programming. 

In Quarter 2 of PIAIC last year I wrote this note on slack IoT channel about "Using threads to Run Code Simultaneously".
In an executed program, code is run in a process. The OS manages these processes. Within a program, there is a feature that run processes independently. This feature is called "thread". Programs split the computation into multiple threads, hence term multi-threading.

Pros of Threads

Improve performance so that the program dow multiple tasks at the same time.

Cons;

But it also adds complexity. Because threads can run simultaneously, there is no available guarantee about the order in whic parts of your code on different threads will run. This cn lead to problems such as;
 * Race Conditions, where threads are accessing data or resources in an inconsistent order.
 * Deadlocks, where two threads are waiting for each other to finish using a resource the other thread has, preventing both threads from continuing.
 *Bugs that happen only in certain situations and are hard to repeoduce and fix reliably.

 Rust attempts to mitigate these negative effects of using threads, but programming in a multi-threaded context still takes careful thought and requires a code structure that is different from that in programs running in single thread.

 There are many ways to implement threads.

 1:1 - For example an OS can provide an API to the Rust programming language to create threads. Here, OS provides one OS thread per one programming language thread. So there is 1:1 relationshipbetween OS thread and programming language thread.

 M:N Relationship

 This means that the programming language provides M number of threads and the OS provides N number of threads to a process. The M threads are called green threads. This makes Rust effective, fast at runtime, the code that is included by the language in every binary. This code can be large or small, and small can be so small that it is called no runtime code. In realitythere is always some kind of runtime code on the programming language side. Small is good and bad. Rust needa to maintain small runtime code to be faster and cannot compromise for speed as compared to other features. Since the green thread in M:N relationship needs large runtime, Rust depend on 1:1 relationship only.
  
async/.await Primer (an introduction to the language's syntax, memory model, and cocurrency model)
async is a modifier that can be applied to functions, where functions instead of running all the way through, calling an async function immediately returns a 'Future' that will resolve eventually whatever the function will return. Inside the async funtion you can take the 'await' operator and apply it to other features which will pause the function untill those features are ready to operate. In this way handling asynchronous concurrent operation using these anotaions (async/await) which makes them easier to write.

It is an intersting discussion for me and as much as I want to explore and try to present it in this assignment, I would better leave the rest for you to watch this one youtube video prentation of asynchronous I/O in Rust, straight from the horse mouth. Here it is;
https://www.youtube.com/watch?v=skos4B5x7qE