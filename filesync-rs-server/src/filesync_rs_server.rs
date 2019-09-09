
use futures::prelude::*;
use futures::{Future, Async, Poll};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::fmt;

#[derive(Clone)]
pub struct FSRServer {
    baseurl: String,
    port: String,
    targetdir: String,
}

impl FSRServer {
    pub fn new(baseurl : String, port : String, targetdir : String) -> Self {
        FSRServer {
            baseurl,
            port,
            targetdir
        }
    }
}


impl Stream for FSRServer {
    type Item = FSRServer;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<FSRServer>, ()> {
        Ok(Async::Ready(Some(self.clone())))
    }
}



// struct Display<T>(T);

// impl<T> Future for Display<T> 
// where
// T: Future,
// T::Item: fmt::Display,
// {
// type item = ();
// type Error = T::Error;

// fn poll(&mut self) -> Poll<(), T::Error> {
// let value = try_ready!(self.0.poll());
// println!("{}", value);
// Ok(Async::Ready(()))
    // }
// }



pub struct Display<T>(T);

impl<T> Future for Display<T>
where
    T: Future,
    T::Item: fmt::Display,
{
    type Item = ();
    type Error = T::Error;

    fn poll(&mut self) -> Poll<(), T::Error> {
        let value = match self.0.poll() {
            Ok(Async::Ready(value)) => value,
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Err(err) => return Err(err),
        };

        println!("{}", value);
        Ok(Async::Ready(()))
    }
}
