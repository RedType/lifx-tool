use std::{
    net::UdpSocket,
    time::{Duration, Instant},
};
use anyhow::Result;
use log::*;
use crate::packing::*;

const BUFFER_SZ: usize = 1000;
//const LOCALHOST: &'static str = "127.0.0.1:56700";
const BROADCAST: &'static str = "255.255.255.255:56700";
const READ_TIMEOUT: Duration = Duration::from_millis(100);

#[derive(Debug)]
pub struct Device {
    pub address: String,
    pub target: u64,
}

pub fn roll_call<T>(timeout: u64, mut wait_loop: T) -> Result<Vec<Device>>
where
    T: FnMut(&Instant)
{
    let socket = UdpSocket::bind(BROADCAST)?;
    socket.set_broadcast(true)?;
    socket.set_read_timeout(Some(READ_TIMEOUT))?;
    trace!("building packet");
    let get_service = pack(Header {
        tagged: true,
        source: 1,
        target: 0,
        res_required: false,
        ack_required: false,
        sequence: 0,
    }, Message::GetService);

    trace!("sending packet");
    socket.send_to(&get_service, BROADCAST)?;

    let mut devices = Vec::new();
    let now = Instant::now();
    while now.elapsed().as_millis() < timeout.into() {
        let mut buffer = [0u8; BUFFER_SZ];
        trace!("listening for replies...");
        let (n, addr) = socket.recv_from(&mut buffer)?;
        trace!("got one!");

        let (r_head, _) = unpack(&buffer[..n])?;

        devices.push(Device {
            address: addr.to_string(),
            target: r_head.target,
        });

        wait_loop(&now);
    }

    Ok(devices)
}
