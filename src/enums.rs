mod ip_address {
    
    #[derive(Debug)]
    pub enum IPAddrKind {
        V4,
        V6
    }

    pub fn create_new_ip(){

    }
}

pub use self::ip_address::*;