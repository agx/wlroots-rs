use seat::RootsSeat;

pub struct RootsServer {
    pub seats: Vec<RootsSeat>
}

impl Default for RootsServer {
    fn default() -> RootsServer {
        RootsServer {
            seats: Vec::new()
        }
    }
}
