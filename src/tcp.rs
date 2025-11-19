struct State {}

impl Default for State {
    fn default() -> Self {
        State{}
    }
    
}

impl State {
    pub fn packet<'a>(
        iph: etherparse::Ipv4HeaderSlice<'a>, 
        tcph: etherparse::TcpHeaderSlice<'a>, 
        data: &'a[u8]) {
        eprintln!(
            "{} -> {} {}b of tcp port {}", 
               src,
               dst,
               p.slice().len(),
               p.destination_port(),
              );
    }
}