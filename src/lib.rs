
use std::process::Command;
use std::time::Duration;
use std::{env, thread};
use rodio::source::SineWave;

use rodio::{Source, Sink};


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_per(){
        let value: u32 = 81;
        assert_eq!(value, percentage());
    }

    #[test]
    fn test_limit(){
        let value: u32 = 80;
        assert_eq!(value, limit());
    }
    
    #[test]
    fn test_alert(){
        alert();
    }

    #[test]
    fn test_charging(){
        assert_eq!(charging(), false);
    }
}

pub fn percentage()-> u32{
    let value = Command::new("cat")
        .arg("/home/hannes/Projekte/Pinebook/bat3/battery_percentage")
        //.arg("/sys/class/power_supply/cw2015-battery/capacity")
        .output()
        .unwrap()
        .stdout;
    let value = std::str::from_utf8(&value).expect("error").to_owned();
    let value: u32 = value.trim().parse().unwrap();

    println!("{}", &value);
    value
}

pub fn limit()-> u32{
    let value = env::var("BAT_LIMIT").unwrap();
    let value: u32 = value.trim().parse().unwrap();
    value
}
pub fn charging() -> bool{
    let val = Command::new("cat")
        .arg("/sys/class/power_supply/cw2015-battery/status")
        .output()
        .unwrap()
        .stdout;
    let val = std::str::from_utf8(&val).expect("failed to parse charging file").trim();
    if val == "Charging"{
        true
    } else {
        false
    }
}
pub fn alert(){
    let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

        let sink = Sink::try_new(&stream_handle).unwrap();
        let source = SineWave::new(440).take_duration(Duration::from_secs_f32(0.5));
        sink.append(source);
        sink.sleep_until_end();
        thread::sleep(Duration::from_secs_f32(0.5));
}
