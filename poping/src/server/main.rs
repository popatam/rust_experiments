use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::mem::MaybeUninit;
use socket2::{Socket, Domain, Type, Protocol};

use poping::{IcmpEcho, icmp_checksum, strip_ipv4_header};

fn main() -> io::Result<()> {
    let sock = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::ICMPV4))?;
    sock.bind(&SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0).into())?;

    println!("ICMP server: waiting for request... (sudo required)");

    loop {
        let mut buf: [MaybeUninit<u8>; 2048] = [MaybeUninit::uninit(); 2048];
        let (n, from) = sock.recv_from(&mut buf)?;
        let icmp: &[u8] = unsafe { strip_ipv4_header(std::slice::from_raw_parts(buf.as_ptr() as *const u8, n)) };

        match IcmpEcho::decode(icmp) {
            Ok(mut req) => {
                println!(
                    "from {}: id=0x{:04x} seq={} payload={:?}",
                    from.as_socket().unwrap_or(SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0)),
                    req.id, req.seq, String::from_utf8_lossy(&req.payload)
                );

                // ответ смысла не имеет, т.к. ядро ответит раньше
                // // echo reply, type=0
                // req.set_payload(Vec::from(""));
                // let mut reply = req.encode();
                // reply[0] = 0; // echo reply
                // reply[2] = 0;
                // reply[3] = 0;
                // let cs = icmp_checksum(&reply);
                // let [hi, lo] = cs.to_be_bytes();
                // reply[2] = hi;
                // reply[3] = lo;
                //
                // sock.send_to(&reply, &from)?;
            }
            Err(_) => continue,
        }
    }
}