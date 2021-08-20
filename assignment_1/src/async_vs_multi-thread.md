#  Multi-threading vs Asynchronous Programming

  
  

##  What is Multi-threading?

  

Multi-threading is the name given to parallel(concurrent) execution of multiple set(threads) of instructions.

  

###  *Multi-threading example*

An example would be downloading two files from two different tabs in a web browser. A tab each uses a thread to download the file. No tab waits for the other to finish. Hence they are downloading concurrently.

  

###  Multi-Threading drawbacks:-

- Race Conditions : Threads access data inconsistently(order)

- DeadLocks : Threads wait for each other to finish using resource that other thread has, preventing both

- Bugs in situations and hard to find.

  

##  What is Asynchronous?

  

Async programming allows operations or tasks to run in a **single OS thread**. If a function is called, it doesn't block the excution flow, and your program continues to run.

  

###  *Async example*

  

The basic example would be fetching of two files over a network & combining them. Basically, We start an additional thread of control. Thread One fetches file *physics.pdf*  & Thread Two fetches *Biology.pdf*(without waiting for Thread one to finish) then both threads wait for result to resynchronize to combine their results.

  

###  Async Drawbacks:-

- Overhead in switching between threads.

- A thread even doing nothing still uses resources.

  

##  *The difference between both*

  

To basic difference between the two is that ***Threading is about Workers while Async is about tasks***.

  

>Let us take an analogy; Ahmed and Bilal decide to make lunch together.

  

>>  Async will be Ahmed says to Bilal, "Go to store and buy Biriyani Masala. **When you get back let me know**,we will make Biriyani together. **Whilst, I ready the rice and meat**.

  

>>  Threading is "Bilal boils the water. Ahmed Makes the Masala. When water boils, bilal tells ahmed to put rice in. When Masala is ready, Bilal mixes it with rice. When both are done, ahmed tells bilal to serve it and they both eat". To sum it up, the ***Sequences of tasks*** represent the instructions per person which is thread.