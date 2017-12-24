extern crate gfapi_sys;
extern crate r2d2;

use gfapi_sys::gluster::*;
use r2d2::ManageConnection;

pub struct GlusterPool{
    server: String,
    port: u16,
    volume: String,
}

impl GlusterPool{
    pub fn new(server: &str, port: u16, volume: &str) -> Self{
        GlusterPool{
            server: server.to_owned(),
            port: port,
            volume: volume.to_owned(),
        }
    }
}

impl ManageConnection for GlusterPool {
    type Connection = Gluster;
    type Error = GlusterError;

    fn connect(&self) -> Result<Self::Connection, Self::Error>{
        let conn = Gluster::connect(&self.volume, &self.server, self.port)?;
        Ok(conn)
    }

    fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error>{
       Ok(()) 
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool{
       false 
    }
}
