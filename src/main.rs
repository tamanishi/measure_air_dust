use std::error::Error;
use std::time::Duration;

use rppal::uart::{Parity, Uart};
use chrono::{Local};

mod data;
use data::AirDust;

fn main() -> Result<(), Box<dyn Error>> {
    let mut uart = Uart::new(2_400, Parity::None, 8, 1)?;

    uart.set_read_mode(1, Duration::default())?;

    let mut buffer = [0u8; 1];
    let mut buffer2 = [0u8; 7];
    loop {
        if uart.read(&mut buffer)? > 0 {
            if buffer[0] != 0xff {
                continue;
            }
        }
        for i in 0..7 {
            if uart.read(&mut buffer)? > 0 {
                buffer2[i] = buffer[0];
            }
        }
        // println!("Received byte: {0:x} {1:x} {2:x} {3:x} {4:x} {5:x} {6:x}", buffer2[0], buffer2[1], buffer2[2], buffer2[3], buffer2[4], buffer2[5], buffer2[6]);
        if buffer2[0] != 0xaa || buffer2[6] != 0xff {
            continue;
        }
        let vouth = buffer2[1];
        let voutl = buffer2[2];
        let vrefh = buffer2[3];
        let vrefl = buffer2[4];
        if buffer2[5] != (vouth + voutl + vrefh + vrefl) {
            continue;
        }
        let vout = (vouth as f32 * 256.0 + voutl as f32) / 1024.0 * 5.0;
        let dust_density = 100.0 / 0.35 * vout;
        // println!("vouth={:x} voutl={:x} vrefh={:x} vrefl={:x} vout={:.3}mV dust_density={:.3}ug/m3", vouth, voutl, vrefh, vrefl, vout, dust_density);
        let air_dust = AirDust {
            timestamp: Local::now(),
            vouth: vouth,
            voutl: voutl,
            vrefh: vrefh,
            vrefl: vrefl,
            vout: vout * 1000.0,
            dust_density: dust_density,
        };
        println!("{{{}}}", air_dust);
        break Ok(());
    }
}

