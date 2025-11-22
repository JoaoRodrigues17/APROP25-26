# What I learned in ex1

## Return value of threads with handle.join()

We can use handle.join().unwrap() to get the return value of a thread:

``` Rust
    let mut handles = Vec::new();
    let handle = thread::spawn(move || {
                (...) //Some thread function
            });

    handles.push(handle);

    for handle in handles {
        let res = handle.join().unwrap();
        (...) // Do something with the result
    }
```

## Chunking vectors to split work among threads

Useful to divide the vector in chunks for each thread to work on independent parts of it:
```Rust 
    let chunk_size = v.len() / NUM_THREADS;
    for chunk in v.chunks(chunk_size) {
        let chunk = chunk.to_owned(); //Necessary for thread to own chunk (I think)
        (...) //create thread here and use "chunk
    }
```

Important that the vector size is divisible by NUM_THREADS or else part of the end of the vector won't be analyzed.
```Rust
    let chunk_size = (map.len()+NUM_THREADS-1) / NUM_THREADS //This is preferable
```

## Sharing variables with Arc and Mutex

Creating a shared variable with Arc and Mutex:

```Rust
    let variable = Arc::new(Mutex::new(1));

    (...)
    let variable = Arc::clone(&variable); //Clone Arc before creating thread
    thread::spawn(move || {
                let mut variable = variable.lock().unwrap(); //Do this before using variable (safe access with mutex)
                variable = 0; // use variable (modify)
            });
    let variable = Arc::try_unwrap(map).unwrap().into_inner().unwrap(); //Get the final value
    
```
