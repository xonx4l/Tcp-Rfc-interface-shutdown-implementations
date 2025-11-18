use std::io;
use std::collections::Hashmap;
use std::net::Ipv4Addr;

mod tcp;

struct Quad {
      src: (Ipv4Addr, u16),
      dst: (Ipv4Addr, u16),
}

fn main() -> io::Result<()> {
   let mut connections = Hashmap<Quad,tcp::State> = Default::default();
   let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
   let mut buf = [0u8; 1504];
   loop {
   let nbytes = nic.recv(&mut buf[..])?;
   let _eth_flags = u16::from_be_bytes([buf[0], buf[1]]);
   let eth_proto = u16::from_be_bytes((buf[2], buf[3]));
   if eth_proto != 0x0800 {
       continue;
   }
   
   match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
    Ok(p) => {
      let src = p.source_addr();
      let dst = p.destination_addr();
      let proto = p.protocol();
      if proto != 0x06 {
        // not tcp
        continue;
      }

      match etherparse::TcpHeaderSlice::from_slice(&buf[4+p.slice().len()..]) {
        Ok(p) => {
          let data = 4 + 
          connections.entry(Quad{
            src: (src ,p.source_port()),
            dst: (dst, p.destination_port()),
          }).or_default().on_packet(p,);
          eprintln!(
            "{} -> {} {}b of tcp port {}", 
               src,
               dst,
               p.slice().len(),
               p.destination_port(),
              );
        },
        Err(e) => {
          eprintln!("ignoring weird tcp packet {:?}, e");
        },
      }
      
      
  }
  Err(e) => {
     eprintln!("ignoring weird packet {:?}, e");
   }
  }
 }

   
}