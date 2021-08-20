/*Multithreading;
Multithreading has not only concurrency solution in Rust but it is also a safety feature. 
I think multithreading is more of a safety feature than a speed or concurrency solution, because Rust
uses 1:1 process-thread relationship.

Every data type knows whether it can safely be sent down between multiple threads or accessed by
them simultaneously. Rust enforces this safe usage; there are no data races, even for lock free data structures.
*/

#[lang = "send"]
pub unsafe trait Send { }

/*
You can even share stack frames between threads, and Rust will statically ensure that the frames
remain active while other threads are using them. Even the most daring forms of sharing are guaranteed safe in Rust.



*/



/*Futures Combinators; where you can build up these state machines but applying all these methodes to the Futures.
This function requests the rust_lang.org and converts that response to a String. So, instead of returning a String 
it returns a Future of a String because it is going to be an async function
and has these features in the body-Client.get() and response.get()- and will be parts of I/O, They all bind together
using this Combinator.
*/

fn fetch_rust_lang(client: Client) {
    -> impl Future <    Output = String>
}
  Client.get("rust-lang.org")).and_then(|response| {
      response.contact_body().map(|body|{
          String::from_utf8_lossy(body)
      })
  })
}

/* Deploying Futures Combinator had a downside of some sort of nested call-backs which can be very difficult to
read. Implementing async/await synatx was proposed as a solution to this downside. The first async await 
was not part of the language but instead a sort of a library that tried to fix the syntax bugging.


async/await
*/
#[async]
fn fetch_rust_lang(client: Client) -> String {
    let response = await!(client.get("rust-lang.org"));
    let body = await!(response.contact_body());
    String::from_utf8_lossy(body)
}

/* This function does the same thing as previos function did. It just fetches rust-lang and turns it into a string,
but it does so using async/await working much more like a normal blocking I/O works. These async innotations
turns the function into a Future instead of just returning immedialtely. The await innotation await on a Future 
that you construct inside the function.
*/

await!($future) = {
    loop {
        match $future.poll() {
            Poll::Pending =>yield Poll::Pending,
            Poll::Ready(value) => break value,
        }
    }
}

/* await function under this Poll model turns into a loop and every time you get pending back, you yield all the way
back to the executor that is pending untill it wakes up again. Finally the Future you are waiting on finishes 
its value and you break out of the loop. That is how these await expressions evaluate to.
*/

#[async]
fn fetch_rust_lang