use std::net::IpAddr;

use chrono::{DateTime, Utc};
use mac_address::MacAddress;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SpeedtestResult {
    #[serde(rename = "type")]
    pub _type: String,
    pub timestamp: DateTime<Utc>,
    pub ping: Ping,
    pub download: Speed,
    pub upload: Speed,
    #[serde(rename = "packetLoss")]
    #[serde(default)]
    pub packet_loss: f64,
    pub isp: String,
    pub interface: Interface,
    pub server: Server,
    pub result: Metadata,
}

#[derive(Debug, Deserialize)]
pub struct Ping {
    pub jitter: f64,
    pub latency: f64,
    pub low: f64,
    pub high: f64,
}

#[derive(Debug, Deserialize)]
pub struct Speed {
    pub bandwidth: usize,
    pub bytes: usize,
    pub elapsed: usize,
    pub latency: Latency,
}

#[derive(Debug, Deserialize)]
pub struct Latency {
    pub jitter: f64,
    pub iqm: f64,
    pub low: f64,
    pub high: f64,
}

#[derive(Debug, Deserialize)]
pub struct Interface {
    #[serde(rename = "internalIp")]
    pub internal_ip: IpAddr,
    pub name: String,
    #[serde(rename = "macAddr")]
    pub mac_addr: MacAddress,
    #[serde(rename = "isVpn")]
    pub is_vpn: bool,
    #[serde(rename = "externalIp")]
    pub external_ip: IpAddr,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub id: usize,
    pub host: String,
    pub port: u16,
    pub name: String,
    pub location: String,
    pub country: String,
    pub ip: IpAddr,
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub id: String,
    pub url: String,
    pub persisted: bool,
}
