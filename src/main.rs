use clap::Parser;
use curl::easy::Easy;
use dns_lookup::lookup_addr;
use icmp_socket::{self, IcmpSocket};
use log::error;
use prettytable::{Cell, Row, Table};
use serde_json::{self, Value};
use std::fmt::Debug;
use std::str::from_utf8;
use std::time::Duration;
use std::time::Instant;
use std::{
    net::{IpAddr, Ipv4Addr, ToSocketAddrs},
    vec,
};

extern crate clap;
extern crate prettytable;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct GarRouteArgs {
    /// ip addres to trace
    #[arg(short, long)]
    pub ip: Option<String>,

    /// domain addres to trace
    #[arg(short, long)]
    pub domain: Option<String>,
}

#[derive(Debug)]
pub struct RouteNode {
    pub hop: u32,
    pub ip_address: String,
    pub hostname: String,
    pub duration: Duration,
    pub city: String,
    pub country: String,
    pub region: String,
    pub org: String,
    pub location: String,
}

pub fn tracer(ip_address: String) -> Vec<RouteNode> {
    let mut result: Vec<RouteNode> = Vec::new();
    let dest_address: Ipv4Addr = ip_address.parse::<Ipv4Addr>().unwrap();
    let mut easy = Easy::new();
    let mut socket: icmp_socket::IcmpSocket4 =
        icmp_socket::IcmpSocket4::new().expect("unable to create icmp4 socket");

    for hop in 1..30 {
        socket.set_max_hops(hop);
        socket.set_timeout(Some(Duration::from_secs(2)));

        let message = icmp_socket::Icmpv4Message::Echo {
            identifier: 1,
            sequence: hop as u16,
            payload: vec![1],
        };
        // Construct an ICMP Echo Request
        let request = icmp_socket::Icmpv4Packet {
            typ: 8,
            code: 0,
            checksum: 0,
            message: message,
        };
        let _ = socket.send_to(dest_address, request);

        let start = Instant::now();
        // Receive ICMP replies or Time Exceeded messages
        match socket.rcv_from() {
            Ok((_resp_packet, src_socket)) => {
                let end = Instant::now();
                let s = src_socket.as_socket().unwrap();
                let mut r = RouteNode {
                    hop: hop,
                    ip_address: s.ip().to_string(),
                    hostname: lookup_addr(&s.ip()).unwrap_or("***".to_string()),
                    duration: end.duration_since(start),
                    city: "***".to_string(),
                    country: "***".to_string(),
                    region: "***".to_string(),
                    org: "***".to_string(),
                    location: "***".to_string(),
                };

                easy.url(&format!("ipinfo.io/{}", s.ip().to_string()))
                    .unwrap();

                // A vector to store the response body.
                let mut data = Vec::new();
                // Perform the HTTP request.
                {
                    // Set up a `write_function` to write the incoming data into our vector.
                    let mut transfer = easy.transfer();
                    transfer
                        .write_function(|new_data| {
                            data.extend_from_slice(new_data);
                            Ok(new_data.len())
                        })
                        .unwrap();

                    // Perform the HTTP request.
                    transfer.perform().unwrap();
                } // `transfer` goes out of scope and is dropped here.

                // Convert the response body to a string.
                let response_body = from_utf8(&data).unwrap();
                let json_value: Value = serde_json::from_str(response_body).unwrap();

                if let Some(city) = json_value.get("city").and_then(|v| v.as_str()) {
                    r.city = city.to_string();
                }
                if let Some(country) = json_value.get("country").and_then(|v| v.as_str()) {
                    r.country = country.to_string();
                }
                if let Some(location) = json_value.get("region").and_then(|v| v.as_str()) {
                    r.region = location.to_string();
                }
                if let Some(location) = json_value.get("org").and_then(|v| v.as_str()) {
                    r.org = location.to_string();
                }
                if let Some(location) = json_value.get("loc").and_then(|v| v.as_str()) {
                    r.location = location.to_string();
                }
                result.push(r);
                if s.ip().to_string() == ip_address {
                    break;
                }
            }
            Err(_) => {
                result.push(RouteNode {
                    hop: hop,
                    ip_address: "***".to_string(),
                    hostname: "***".to_string(),
                    duration: Duration::from_secs(0),
                    city: "***".to_string(),
                    country: "***".to_string(),
                    region: "***".to_string(),
                    org: "***".to_string(),
                    location: "***".to_string(),
                });
                continue;
            }
        }
    }
    return result;
}

fn main() {
    env_logger::init();
    // check if input argument is ip address or domain name
    let args: GarRouteArgs = GarRouteArgs::parse();

    // check if either of the ip address and domain provided as input
    if (args.ip.is_none() && args.domain.is_none()) || (args.ip.is_some() && args.domain.is_some())
    {
        error!("Provide either ip address or domain name to trace against");
        std::process::exit(1);
    }

    let mut ip_addresses: Vec<String> = Vec::new();

    if args.domain.is_some() {
        let socket_addrs_iter: Vec<std::net::SocketAddr> = (args.domain.unwrap(), 0)
            .to_socket_addrs()
            .unwrap()
            .into_iter()
            .collect();
        for socket_addr in socket_addrs_iter {
            if socket_addr.is_ipv4() {
                let ip_addr: IpAddr = socket_addr.ip(); // Extract the IpAddr
                ip_addresses.push(ip_addr.to_string()); // Convert IpAddr to String and push
            }
        }
    } else {
        ip_addresses.push(args.ip.unwrap());
    }

    let results: Vec<RouteNode> = tracer(ip_addresses[0].clone());
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("hop"),
        Cell::new("ip_address"),
        Cell::new("hostname"),
        Cell::new("duration"),
        Cell::new("city"),
        Cell::new("country"),
        Cell::new("region"),
        Cell::new("org"),
        Cell::new("location"),
    ]));

    for result in results.iter() {
        table.add_row(Row::new(vec![
            Cell::new(&result.hop.to_string()),
            Cell::new(&result.ip_address),
            Cell::new(&result.hostname),
            Cell::new(&format!("{:?}", result.duration)),
            Cell::new(&result.city),
            Cell::new(&result.country),
            Cell::new(&result.region),
            Cell::new(&result.org),
            Cell::new(&result.location),
        ]));
    }
    table.printstd();
}
