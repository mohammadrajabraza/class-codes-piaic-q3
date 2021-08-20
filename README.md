# Difference Between Multi-Threaded and Async Programming in Rust

## Multi-Threaded:

The term Multi-Threaded is means running multiple threads parallel in a core.
In simple programming we run one function then move towards another function. The basic programming approach increases the program completion time. 
Suppose you are currently using a text editor and you want to see some previous stuff which is on another text file so you cannot open it because to open it you have to free the current text editor resource by closin the current program which makes you frustrating so to handle such scenarios multi-threading is used. In multi-threading multiple threads are running parallel to each other. You can run/do different things at a time on a machine 
for example: surfing web pages and writing on a word file to make your notes or downloading two or three files from browser while playing games this can be done only by using Multi-Threading but it has some issues like if all the resource can make the CPU pipeline congestive so it can cause issue and fail all the threads means a dead lock can appear because some OS uses First Come First Serve algorithm or Round Robin Algorithm to schedule the CPU programs when all the threads come for finding some resource or stuff at the time then the CPU will fall in dead lock state because the data or resource they are looking for already be given to another thread which is taking time so it will create an infinite waiting till the resources has been released which causes the dead lock.

### Example as in my code:

Inside iot_home there are four threads running parallel to each other. 
> Turn on Fan
> Turn on bulb
> Turn off Fan
> Turn off bulb
All the above will run concurrently but they can produce the race condition if multiple user consumed these resources at the same time.

## Async Programming:

The term Async comes from Asynchronous and async means not to wait for another task because of this async functions are also known as independent functions and it also means different clock rates. The tasks which are bound by some requests or finishing of some work response for example I/O tasks takes a lot of time waiting for response, so to handle such scenarios Async Programming is used to keep the functions idle until they get some input or response from the server. This parallel program functionality increase the durability and stability of the servers and end devices.

### Example as in my code:

When the async function motion sensor starts it will not wait cleaner bot to clean the house it keeps it keeps working and when the bot completed the task anytime it will return to it's pod the program will be terminated but on the other hand the program will not wait for any other non async process threads to wait _*like the bulb turn off thread_ never finished but the program terminated*.