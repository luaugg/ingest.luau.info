use digitalocean_api::{error::Error, prelude::*};
use std::net::Ipv4Addr;

pub struct DOClient {
    client: DigitalOcean,
    snapshot_id: String,
}

impl DOClient {
    pub fn new(api_key: String, snapshot_id: String) -> Self {
        let client = DigitalOcean::new(api_key).unwrap();
        Self {
            client,
            snapshot_id,
        }
    }

    pub async fn get_network_address(&self) -> Result<Ipv4Addr, Error> {
        let droplets = Droplet::list().execute(&self.client).await?;
        let droplet = droplets.iter().find(|d| d.name() == "ingest");
        match droplet {
            Some(droplet) => {
                let public = droplet.networks().v4.iter().find(|n| n.kind == "public");
                match public {
                    Some(network) => Ok(network.ip_address),
                    None => Err(Error::NotFound),
                }
            }
            None => Err(Error::NotFound),
        }
    }

    pub async fn create_droplet(&self) -> Result<Droplet, Error> {
        Droplet::create("ingest", "lon1", "c-4", &self.snapshot_id)
            .execute(&self.client)
            .await
    }

    pub async fn delete_droplet(&self) -> Result<(), Error> {
        let droplets = Droplet::list().execute(&self.client).await?;
        let droplet = droplets.iter().find(|d| d.name() == "ingest");
        match droplet {
            Some(droplet) => Droplet::delete(*droplet.id()).execute(&self.client).await,
            None => Err(Error::NotFound),
        }
    }
}
