#![forbid(unsafe_code)]

use std::{time::Duration, sync::Arc};
// Main Library file
use sabaton_mw::{NodeBuilder, error::MiddlewareError};
use tracing::{debug, info, span, Level};
//use vehicle_signals::{v2::{vehicle::Speed, self}, units::KilometrePerHour };
use vehicle_signals::{
    units::KilometrePerHour,
    v2::vehicle::cabin::door::window::Position,
    v2::{vehicle::Speed, self},
    v2::vehicle::IsMoving,
    v2::vehicle::IgnitionOn,
};

pub fn example_node_main() -> Result<(),MiddlewareError> {

    let node =   NodeBuilder::default()
        //.multi_threaded()  Enable this if you want a multi-threaded runtime
        //.with_num_workers(4)    // Number of work threads. Fixed to 1 for single threaded runtime.
        .build("example-node".to_owned()).expect("Node creation error");


    let mut SpeedWriter= node.advertise::<v2::vehicle::Speed>().expect("Unable to advertise");
    let mut IsmovingWriter= node.advertise::<v2::vehicle::IsMoving>().expect("Unable to advertise");
    let mut IgnitionOnWriter=node.advertise::<v2::vehicle::IgnitionOn>().expect("Unable to advertise");
    let res = node.spin(move || {
        
        span!(target: "MAIN", Level::TRACE, "Application Main Loop");
        info!("Application Main Loop Started with tick interval 100mS");

        let mut ticker = tokio::time::interval(Duration::from_millis(100));

        let _task = tokio::spawn( async move {


            loop {
                let _ = ticker.tick().await;
                debug!("Tick");
                let speed = Arc::new(Speed::new(KilometrePerHour(0.0), None).unwrap());
                let mut res = SpeedWriter.publish(speed);
                let moving = Arc::new(IsMoving::new(false,None).unwrap());
                let mut res = IsmovingWriter.publish(moving);
                let ignition = Arc::new(IgnitionOn::new(false,None).unwrap());
                let mut res = IgnitionOnWriter.publish(ignition);
            }

         });
         
    });


    res

}