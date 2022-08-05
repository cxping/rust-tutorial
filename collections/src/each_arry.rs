use serde::{Deserialize,Serialize};
use serde_json::json;




#[derive(Debug,Deserialize,Serialize)]
struct  Configurations{
    trueSampleRate:i32,
    clockDivider:i32,
    acquisitionCycles:i32,
    oversampleRate:i32,
    sampleRate:i32,
    sampleRateDivider:i32,
    recordCurrent:f64,
    energySaverRecordCurrent:f64,
    listenCurrent:f64,
    energySaverListenCurrent:f64
}
pub fn configurations(){
    let config_josn = json!([
        {
            "trueSampleRate":8,
            "clockDivider": 4,
            "acquisitionCycles": 16,
            "oversampleRate": 1,
            "sampleRate": 384000,
            "sampleRateDivider": 48,
            "recordCurrent": 9.22,
            "energySaverRecordCurrent": 5.92,
            "listenCurrent": 8.59,
            "energySaverListenCurrent": 5.41
        }, {
            "trueSampleRate": 16,
            "clockDivider": 4,
            "acquisitionCycles": 16,
            "oversampleRate": 1,
            "sampleRate": 384000,
            "sampleRateDivider": 24,
            "recordCurrent": 9.83,
            "energySaverRecordCurrent": 6.63,
            "listenCurrent": 8.72,
            "energySaverListenCurrent": 5.54
        }, {
            "trueSampleRate": 32,
            "clockDivider": 4,
            "acquisitionCycles": 16,
            "oversampleRate": 1,
            "sampleRate": 384000,
            "sampleRateDivider": 12,
            "recordCurrent": 11.3,
            "energySaverRecordCurrent": 8.04,
            "listenCurrent": 8.95,
            "energySaverListenCurrent": 5.78
        }, {
            "trueSampleRate": 48,
            "clockDivider": 4,
            "acquisitionCycles": 16,
            "oversampleRate": 1,
            "sampleRate": 384000,
            "sampleRateDivider": 8,
            "recordCurrent": 12.3,
            "energySaverRecordCurrent": 8.93,
            "listenCurrent": 9.14,
            "energySaverListenCurrent": 5.98
        }, {
            "trueSampleRate": 96,
            "clockDivider": 4,
            "acquisitionCycles": 16,
            "oversampleRate": 1,
            "sampleRate": 384000,
            "sampleRateDivider": 4,
            "recordCurrent": 15.8,
            "energySaverRecordCurrent": 15.8,
            "listenCurrent": 10.0,
            "energySaverListenCurrent": 10.0
        }, {
            "trueSampleRate": 192,
            "clockDivider": 4,
            "acquisitionCycles": 16,
            "oversampleRate": 1,
            "sampleRate": 384000,
            "sampleRateDivider": 2,
            "recordCurrent": 24.1,
            "energySaverRecordCurrent": 24.1,
            "listenCurrent": 11.5,
            "energySaverListenCurrent": 11.5
        }, {
            "trueSampleRate": 250,
            "clockDivider": 4,
            "acquisitionCycles": 16,
            "oversampleRate": 1,
            "sampleRate": 250000,
            "sampleRateDivider": 1,
            "recordCurrent": 26.4,
            "energySaverRecordCurrent": 26.4,
            "listenCurrent": 10.6,
            "energySaverListenCurrent": 10.6
        }, {
            "trueSampleRate": 384,
            "clockDivider": 4,
            "acquisitionCycles": 16,
            "oversampleRate": 1,
            "sampleRate": 384000,
            "sampleRateDivider": 1,
            "recordCurrent": 38.5,
            "energySaverRecordCurrent": 38.5,
            "listenCurrent": 12.7,
            "energySaverListenCurrent": 12.7
        }
    ]);
    let configurations = serde_json::from_value::<Vec<Configurations>>(config_josn).unwrap();
    configurations.iter().for_each(|f|{
        println!("{},{},{},{},{},{},{},{},{},{}",f.trueSampleRate,f.clockDivider,f.acquisitionCycles,f.oversampleRate,f.sampleRate,f.sampleRateDivider,f.recordCurrent,f.energySaverRecordCurrent,f.listenCurrent,f.energySaverListenCurrent)
    });


}