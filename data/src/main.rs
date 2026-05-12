use std::time::Instant;

use data::*;

fn main() -> Result<(), CanError> {
    let num_tests = 1000;

    // let mut msg = SensorSonars::new(0, 15)?;
    let mut msg = SensorSonarsMsg::new(15, SensorSonarsMsgMux::V0(
        SensorSonarsMsgMux0::new(0.9, 0.9, 0.9, 0.9)?
    ))?;

    let start = Instant::now();

    for _ in 0..num_tests {
        let _ = msg.set_sensor_sonars_err_count(10);
    }

    let elapsed = start.elapsed();
    let avg = elapsed.as_nanos() as f64 / num_tests as f64;

    println!("Average setter time: {avg:.2} ns");

    let start = Instant::now();

    for _ in 0..num_tests {
        msg.sensor_sonars_err_count();
    }

    let elapsed = start.elapsed();
    let avg = elapsed.as_nanos() as f64 / num_tests as f64;

    println!("Average getter time: {avg:.2} ns");

    let mut for_avg = Vec::new();
    for _ in 0..num_tests {
        // let sub_msg = SensorSonarsSensorSonarsMuxM1::new();
        let sub_msg = SensorSonarsMsgMux1::new(0.9, 0.9, 0.9, 0.9)?;

        let start = Instant::now();
        // let _ = msg.set_m1(sub_msg);
        let _ = msg.set_mux1(sub_msg);
    
        let elapsed = start.elapsed().as_nanos() as f64;
        for_avg.push(elapsed);
    }

    let avg = for_avg.iter().sum::<f64>() / num_tests as f64;
    println!("Average mux setter time: {avg:.2} ns");

    let mut for_avg = Vec::new();
    for _ in 0..num_tests {
        let start = Instant::now();
    
        // let _= msg.sensor_sonars_mux();
        let _ = msg.mux();
    
        let elapsed = start.elapsed().as_nanos() as f64;
        for_avg.push(elapsed);
    }

    let avg = for_avg.iter().sum::<f64>() / num_tests as f64;
    println!("Average mux getter time: {avg:.2} ns");

    Ok(())
}
