#![forbid(unsafe_code)]

use sabaton_mw::PublishOptions;
use std::{error, thread};
use std::{sync::Arc, time::Duration};
// Main Library file
use sabaton_mw::{error::MiddlewareError, NodeBuilder};
use tracing::{debug, info, span, Level};
//use vehicle_signals::{v2::{vehicle::Speed, self}, units::KilometrePerHour };
use vehicle_signals::{
    units::KilometrePerHour,
    v3::{self, vehicle::Speed},
};

pub fn example_node_main() -> Result<(), MiddlewareError> {
    let node = NodeBuilder::default()
        //.multi_threaded()  Enable this if you want a multi-threaded runtime
        //.with_num_workers(4)    // Number of work threads. Fixed to 1 for single threaded runtime.
        .build("example-node".to_owned())
        .expect("Node creation error");

    let publish_options = PublishOptions::default();
    let mut SpeedWriter = node
        .advertise::<v3::vehicle::Speed>(&publish_options)
        .expect("Unable to advertise");

    let res = node.spin(move || {
        span!(target: "MAIN", Level::DEBUG, "Application Main Loop");
        debug!("Application Main Loop Started with tick interval 100mS");

        let mut ticker = tokio::time::interval(Duration::from_millis(100));

        let _task = tokio::spawn(async move {
            let span = span!(target: "MAIN", Level::DEBUG, "Application Main Loop");
            let _root_span = span.enter();

            loop {
                let _ = ticker.tick().await;
                tracing::error!("Tick");
                let speed = Arc::new(Speed::new(KilometrePerHour(10.0), None).unwrap());
                let mut res = SpeedWriter.publish(speed.clone());
                match res {
                    Ok(v) => println!("Speed: {:?}", speed.value().0),
                    Err(e) => println!("Error in publishing speed: {:?}", e),
                }
            }
        });
    });

    res
}
