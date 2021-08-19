//! Contains methods useful for implementing a new competition

use std::{future::Future, pin::Pin, sync::Arc, time::Duration};

use async_trait::async_trait;

use doxa_core::actix_web;
use doxa_mq::model::UploadEvent;

// pub trait Callback<T = ()>: Fn() -> Pin<Box<dyn Future<Output = T>>> {}
// //

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
//pub type BoxedCallback<T = ()> = Box<dyn Fn() -> BoxFuture<'static, T>>;
pub type BoxedCallback = Box<dyn Callback>;

/// Returns true if the competition name is valid.
/// Competition names must be entirely consisting of ASCII lowercase letters [a-z]
/// or hyphens '-'.
/// No other characters are allowed.
/// They length must be greater than 3 characters and less than 20.
///
/// The name must also be unique but this function will not check that.
pub fn validate_competition_name(name: &str) -> bool {
    if !(name.len() > 3 && name.len() < 20) {
        return false;
    }

    name.chars().find(|c| !c.is_ascii_lowercase()).is_none()
}

pub trait Callback: 'static {
    fn call(&self) -> BoxFuture<'static, ()>;
}

impl<F: Future<Output = ()> + Send + 'static, T: Fn() -> F + 'static> Callback for T {
    fn call(&self) -> BoxFuture<'static, ()> {
        Box::pin(self())
    }
}

impl<T: Callback> From<T> for Box<dyn Callback> {
    fn from(callback: T) -> Self {
        Box::new(callback)
    }
}

// Maybe rename to BaseContext which contains stuff that can be cloned around then before actually
// passing it into things such as routes we extract the db_pool and store it in the Context, or we
// make those kinds of methods take in a DbConnection and provide another method which takes in
// DbPool and returns DbConnection
// Maybe context should be an Arc, pub type Context = Arc<Base(or other name)Context>;
#[derive(Clone)]
pub struct Context {}

impl Context {
    pub fn mongo(&self) {
        todo!();
    }

    // pub fn register_timer<A: Fn(&mut Context) -> B, B: Future<Output = ()>>(
    //     &self,
    //     duration: Duration,
    // ) {
    //     todo!()
    // }
}

#[async_trait]
pub trait Competition: 'static + Send + Sync {
    // Maybe &mut self could be enforced as startup happens before everything else
    // could also be the case that startup returns Self.
    /// Runs exactly once at startup before all other functions
    // Maybe add StartupContext for things such as registering timers.
    async fn startup(&self, context: &mut Context);

    /// Sets up routes with an automatic prefix of `/competition/{competition_name}`.
    /// This may run multiple times.
    ///
    /// This is the appropriate place to insert `app_data` such as configs.
    fn configure_routes(&self, service: &mut actix_web::web::ServiceConfig);

    /// Runs whenever a new agent has been successfull uploaded.
    /// TODO: upload info
    async fn on_upload(&self, context: &mut Context, upload_event: UploadEvent);

    /// Runs whenever the result of an execution (commonly called a match) has been completed.
    /// TODO: execution info
    /// maybe have a `.save(conn)` method to save to the database
    async fn on_execution_result(&self, context: &mut Context);

    /// Returns the name of the competition.
    ///
    /// See [`validate_competition_name`] for more info.
    fn name(&self) -> String;
}

// pub struct Competition {
//     name: String,
//     on_startup: BoxedCallback,
//     on_upload: BoxedCallback,
//     timers: Vec<(Duration, BoxedCallback)>,
// }
//
// impl Competition {
//     pub fn new(
//         name: impl Into<String>,
//         on_startup: impl Callback,
//         on_upload: impl Callback,
//     ) -> Self {
//         Competition {
//             name: name.into(),
//             on_startup: on_startup.into(),
//             on_upload: on_upload.into(),
//             timers: Vec::new(),
//         }
//     }
// }

// pub trait Competition {
//     fn on_startup(&self, context: &mut Context) -> Pin<Box<dyn Future<Output = ()>>> {
//         // OR
//         // fn on_startup(&self, context: &mut Context) -> Pin<Box<dyn Future<Output = ()>>> where
//         // Self: Sized {
//         // (Could just add trait Competition: Sized at the top)
//         Box::pin(async {})
//     }
//
//     fn on_upload(&self, context: &mut Context) -> Pin<Box<dyn Future<Output = ()>>>;
// }
