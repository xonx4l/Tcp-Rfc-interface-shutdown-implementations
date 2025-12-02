pub enum State {
    Closed,
    Listen,
    SynRcvd,
    Estab,
}

impl Default for State {
    fn default() -> Self {
        State{}
    }
    
}

impl State {
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
                    return;
                }
                State::Listen => {
                    if !tcph.syn(){
                        return;
                    }

                    let mut syn_ack =
                        etherparse::TcpHeader::new(
                            tcph.destination_port(), 
                            tcph.source_port(), 
                            unimplemented!(),
                            unimplemented!(),
                        );
                        syn_ack.syn = true;
                        syn_ack.ack = true;
                    let mut ip = etherparse::Ipv4Header::new(
                        syn_ack.slice().len(),
                        64,
                        etherparse::IpTrafficClass::Tcp,
                        iph.destination_addr(),
                        iph.source_addr(),
                    );
                    let unwritten = {
                        let mut unwritten = &mut buf[..];
                        ip.write(unwritten);
                        syn.ack.write(unwritten);
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