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
      
    una: usize,
    nxt: usize,
    wnd: usize,
    up:  bool,
    wl1: usize,
    wl2: usize,
    iss: usize,
}

struct RecvSequenceSpace {

    nxt: usize,
    wnd: usize,
    ip:  bool,
    irs: usize,
}


impl Default for Connection {
    fn default() -> Self {
        Connection{
            State,
        }
    }
    
}

impl Connection {
    pub fn packet<'a>(
        &mut self,
        nic: &mut tun::tap::Iface;
        iph: etherparse::Ipv4HeaderSlice<'a>, 
        tcph: etherparse::TcpHeaderSlice<'a>, 
        data: &'a[u8]) {
            let mut buf = [0u8, 1500];
            let mut unwritten = &mut buf[..];
            match *self{
                State::Closed => {
                    return Ok(0);
                }
                State::Listen => {
                    if !tcph.syn(){
                        return Ok(0);
                    }

                    self.recv.nxt = tcph.sequence_number() + 1;
                    self.recv.wnd = tcph.window_size();
                    self.recv.irs = tcph.sequence_number();

                    self.send.iss = 0;
                    self.send.una = self.iss;
                    self.send.nxt = self.una + 1;
                    self.send.wnd = 10;
                    
                    let mut syn_ack =
                        etherparse::TcpHeader::new(
                            tcph.destination_port(), 
                            tcph.source_port(), 
                            self.iss,
                            self.wnd,
                        );
                        syn_ack.acknowledgement_number = self.recv.nxt;
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
                    nic.send(&buf[..unwritten])
                }
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
}