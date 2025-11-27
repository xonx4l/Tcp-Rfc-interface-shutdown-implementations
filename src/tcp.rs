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
        iph: etherparse::Ipv4HeaderSlice<'a>, 
        tcph: etherparse::TcpHeaderSlice<'a>, 
        data: &'a[u8]) {
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