pub enum State {
    Closed,
    Listen,
    SynRcvd,
    Estab,
}

pub struct Connection {
    state: State,
}

struct SendSequenceSpace {
      
    una: u32,
    nxt: u32,
    wnd: u16,
    up:  bool,
    wl1: u32,
    wl2: u32,
    iss: u32,
}

struct RecvSequenceSpace {

    nxt: u32,
    wnd: u16,
    ip:  bool,
    irs: u32,
}


impl Default for Connection {
    fn default() -> Self {
        Connection{
            State,
        }
    }
    
}

impl Connection {
    pub fn accept<'a>(
        &mut self,
        nic: &mut tun::tap::Iface;
        iph: etherparse::Ipv4HeaderSlice<'a>, 
        tcph: etherparse::TcpHeaderSlice<'a>, 
        data: &'a[u8]) -> io::Result<Self> {
            let mut buf = [0u8, 1500];
            if !tcph.syn(){
                return Ok(None);
            }

            let mut c = Connection {
                state: State::SynRcvd,
                send: SendSequenceSpace{
                    iss: 0,
                    una: self.send.iss,
                    nxt: self.send.una + 1,
                    wnd: 10,
                    up: false,

                    wl1: 0,
                    wl2: 0,
                }
                recv: RecvSequenceSpace{
                    irs: tcph.sequence_number(),
                    nxt: tcph.sequence_number() + 1,
                    wnd: tcph.window_size(),
                    up: false,
                }
            };
                    
                    let mut syn_ack =
                        etherparse::TcpHeader::new(
                            tcph.destination_port(), 
                            tcph.source_port(), 
                            c.send.iss,
                            c.send.wnd,
                        );
                        syn_ack.acknowledgement_number = c.recv.nxt;
                        syn_ack.syn = true;
                        syn_ack.ack = true;
                    let mut ip = etherparse::Ipv4Header::new(
                        syn_ack.slice().len(),
                        64,
                        etherparse::IpTrafficClass::Tcp,
                        [
                           iph.destination()[0],
                           iph.destination()[1],
                           iph.destination()[2],
                           iph.destination()[3],
                        ],
                        [
                           iph.source()[0],
                           iph.source()[1],
                           iph.source()[2],
                           iph.source()[3],
                        ],
                    );
                    let unwritten = {
                        let mut unwritten = &mut buf[..];
                        ip.write(&mut unwritten);
                        syn.ack.write(&mut unwritten);
                        unwritten.len();
                    };
                    nic.send(&buf[..unwritten])?;
                    Ok(Some(c))
                }
        eprintln!(
            "{} -> {} {}b of tcp port {}", 
               iph.source_addr(),
               tcph.source_addr(),
               iph.destination_addr(),
               tcph.destination_addr(),
               data.len(),
              );
    }
