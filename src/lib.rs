use std::process::{Command, Output};
use std::time::Duration;
use std::{env, thread};

use std::io::BufReader;
use std::fs::File;
use rodio::source::{SineWave, Source};
use rodio::{Decoder, Sink, OutputStream};

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_percentage(){
        let value: u32 = 81;
        assert_eq!(value, percentage());
        //for test uncomment other controll path in percentage()
    }

    #[test]
    fn test_limit(){
        let value: u32 = 80;
        assert_eq!(value, limit());
    }
    
    #[test]
    fn test_alert(){
        for _ in 0..5{
        alert()
        }
    }

    #[test]
    fn test_charging(){
        assert!(charging());
    }
}

pub fn percentage()-> u32{
    let path = env::var("BAT_FILE").unwrap();
    let value = Command::new("cat")
        .arg(format!("{path}/capacity"))
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
    let path = env::var("BAT_FILE").unwrap();
    let val = Command::new("cat")
        .arg(format!("{path}/status"))
        .output()
        .unwrap()
        .stdout;
    let val = std::str::from_utf8(&val).expect("failed to parse charging file").trim();
    val == "Charging"
}
/*
pub fn alert(){
    let (_, stream_handle) = rodio::OutputStream::try_default().unwrap();

    let sink = Sink::try_new(&stream_handle).unwrap();
    let source = SineWave::new(440).take_duration(Duration::from_secs_f32(0.5));
    sink.append(source);
    sink.sleep_until_end();
    thread::sleep(Duration::from_secs_f32(0.5));
}
*/

pub fn alert(){
    let (_stream, stram_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stram_handle).unwrap();
    let source = SineWave::new(440).take_duration(Duration::from_secs_f32(1.0));
    sink.append(source);

    sink.sleep_until_end();
}
