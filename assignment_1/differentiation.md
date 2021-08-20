>###Multi-threading
> - Multi threading refers to CPU bound operations that can be run in parallel using seprate threads. 
> - It requires multiple threads.
> - It requires context switching by OS

> In example code we take the example of file download using 2 browser tabs each tab running on the seprate thread. Now 2 threads downloading the file seprately. We also mention the single thread excution just for the concept.


>###Async / Await
> - Async model refers to I/O bound operations waiting for another task to complete and upon the completion of the second task transfer the control to the next task. 
> - It can be run with in the single thread. 
> - It utilizes underlying programing framework to do the Task switching.

> In example we take example of Morning routine i.e TakeShower - EatBreakfast - DressingUp - ReadNews the EatBreakfast and ReadNews can be done at the same time so they are asynchronously processing. The other 2 activities run synchronously.