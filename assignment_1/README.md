## Multi-threading

# Brief Introduction
It is the ability to perform multiple tasks simultaneoulsy using different threads. In other words it can be said that using multiple threads in parallel fot ultiple tasks is called multi-threading.

# Pros:
It saves time for complex resource-heavy tasks.

# Cons:
One thread performs only one task at a time.
Have difficult cases where threads lock each others' memory.

# Example:
Making tea (task 1) and egg (task 2) in parallel using two stoves (threads).

# Types of Tasks:
Multi-threading is useful in the scenerios where program requires additional computing power.


## Async-Programming 

# Brief Introduction
The use of one thread to perform multiple tasks/ functions is termed as Async-Programming. It can be implemented using various executors like block_on, await etc. It is also termed as event-driven Asynchronous-Programming.

# Pros:
One thread is enough for multiple tasks.
Avoid many problems of multi-threading.

# Cons:
Eextensive scheduling is required.

# Example:
Making tea and egg one after the other in a manner that while waiting for the tea to be ready, crack the eggs and put them in a pan. Meanwhile the tea is ready to put in the cup while egg is cooking. Once the tea is in the cup, egg is ready to be served.

# Types of Tasks:
Async-Programming is useful in the scenerio whwre a lot of input & output is required