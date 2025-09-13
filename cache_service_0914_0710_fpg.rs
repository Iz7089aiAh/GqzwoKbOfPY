use actix::{Actor, Context, Handler, Message, Supervised,SystemService};
use actix::prelude::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::sync::Mutex;

// Define the message to get a value from the cache
#[derive(Message)]
#[rtype(result = "Result<Option<String>, ()>")]
struct GetCacheValue {
    key: String,
}

// Define the message to set a value in the cache
#[derive(Message)]
#[rtype(result = "Result<(), ()>")]
struct SetCacheValue {
    key: String,
    value: String,
    duration: Duration,
}

// Define the cache actor
pub struct CacheActor {
    map: Mutex<HashMap<String, (String, Instant)>>,
}

impl CacheActor {
    pub fn new() -> Self {
        CacheActor {
            map: Mutex::new(HashMap::new()),
        }
    }
}

impl Actor for CacheActor {
    type Context = Context<Self>;
}

impl Handler<SetCacheValue> for CacheActor {
    type Result = Result<(), ()>;
    fn handle(&mut self, msg: SetCacheValue, _: &mut Self::Context) -> Self::Result {
        let mut map = self.map.lock().unwrap();
        map.insert(
            msg.key.clone(),
            (msg.value, Instant::now() + msg.duration),
        );
        Ok(())
    }
}

impl Handler<GetCacheValue> for CacheActor {
    type Result = Result<Option<String>, ()>;
    fn handle(&mut self, msg: GetCacheValue, _: &mut Self::Context) -> Self::Result {
        let mut map = self.map.lock().unwrap();
        if let Some((value, expiry)) = map.get_mut(&msg.key) {
            if Instant::now() < *expiry {
                return Ok(Some(value.clone()));
            } else {
                map.remove(&msg.key);
            }
        }
        Ok(None)
    }
}

// Implementation of SystemService to start the CacheActor as a system service
impl SystemService for CacheActor {
    fn service_started(&mut self, _ctx: &mut Self::Context) {
        // Service initialization logic can be added here
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix::System;

    #[test]
    fn test_cache_set_get() {
        let sys = System::new("test_cache_set_get");
        let addr = SyncArbiter::start(4, || CacheActor::new()).start();

        let set_value = SetCacheValue {
            key: "key".to_string(),
            value: "value".to_string(),
            duration: Duration::from_secs(1),
        };

        addr.do_send(set_value);
        let get_value = GetCacheValue { key: "key".to_string() };

        let result = addr.send(get_value).wait().unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "value");
    }
}
