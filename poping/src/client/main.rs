use std::env;
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::mem::MaybeUninit;
use socket2::{Socket, Domain, Type, Protocol};

use poping::{IcmpEcho, icmp_checksum, strip_ipv4_header};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: client <ipv4> <message>");
        std::process::exit(1);
    }
    let dst_ip: Ipv4Addr = args[1].parse().expect("bad IPv4");
    let message = args[2].as_bytes().to_vec();

    let sock = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::ICMPV4))?;
    sock.bind(&SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0).into())?;

    let pid = std::process::id() as u16;
    let packet = IcmpEcho::new(pid, 1, message).encode();

    // send
    let dst = SocketAddr::new(IpAddr::V4(dst_ip), 0);
    let sent = sock.send_to(&packet, &dst.into())?;
    println!("sent {} bytes to {}", sent, dst);

    let mut buf: [MaybeUninit<u8>; 2048] = [MaybeUninit::uninit(); 2048];
    let (n, _from) = sock.recv_from(&mut buf)?;
    let icmp: &[u8] = unsafe { strip_ipv4_header(std::slice::from_raw_parts(buf.as_ptr() as *const u8, n)) };


    match IcmpEcho::decode(icmp) {
        Ok(reply) => {
            println!(
                "reply: id=0x{:04x}, seq={}, payload={:?}",
                reply.id, reply.seq, String::from_utf8_lossy(&reply.payload)
            );
            println!("checksum (reply) = 0x{:04x}", icmp_checksum(icmp));
        }
        Err(e) => eprintln!("decode error: {e}"),
    }

    Ok(())
}
