use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Once;
use std::sync::{Arc, Mutex};
use std::mem;
use core::arch::x86_64::_rdtsc;

fn systemTime() -> u128{
    SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis() as u128
}

pub struct TimingData{
    frameNumber: u32,
    pub lastFrameTimestamp: u128,
    pub lastFrameDuration: u128,
    lastFrameClockstamp: u128,
    lastFrameClockTicks: u128,
    isPaused: bool,
    averageFrameDuration: f64,
    fps: f32,
}

impl TimingData{
    pub fn get() -> Arc<Mutex<TimingData>>{
        static mut VAL: *const Arc<Mutex<TimingData>> = 0 as *const Arc<Mutex<TimingData>>;
        static INIT: Once = Once::new();

        unsafe{
            INIT.call_once(|| {
                let val = TimingData::new();
                VAL = mem::transmute(Box::new(Arc::new(Mutex::new(val))));
            });

            (*VAL).clone()
        }
    }

    pub fn new() -> TimingData{
        TimingData{
            frameNumber: 0,
            lastFrameTimestamp: systemTime(),
            lastFrameDuration: 0,
            lastFrameClockstamp: systemTime(),
            lastFrameClockTicks: 0,
            isPaused: false,
            averageFrameDuration: 0.0,
            fps: 0.0,
        }
    }

    pub fn getTime() -> u128{
        systemTime()
    }

    pub fn getClock() -> i64{
        unsafe{
            _rdtsc()
        }
    }

    pub fn update(&mut self){
        self.frameNumber += 1;
        let this_time = systemTime();
        self.lastFrameDuration = this_time - self.lastFrameTimestamp;
        self.lastFrameTimestamp = this_time;

        let this_clock = Self::getClock() as u128;
        self.lastFrameClockTicks = this_clock - self.lastFrameClockstamp;
        self.lastFrameClockstamp = this_clock;

        if self.frameNumber > 1{
            if self.averageFrameDuration <= 0.0{
                self.averageFrameDuration = self.lastFrameDuration as f64;
            }else{
                self.averageFrameDuration *= 0.99;
                self.averageFrameDuration += 0.01*self.lastFrameDuration as f64;

                self.fps = (1000.0 / self.averageFrameDuration) as f32;
            }
        }
    }
}
