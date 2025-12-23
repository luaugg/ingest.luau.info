use std::net::Ipv4Addr;

use cloudflare::{
    endpoints::dns::dns::{
        CreateDnsRecord, CreateDnsRecordParams, DnsContent, DnsRecord, ListDnsRecords,
        ListDnsRecordsParams, UpdateDnsRecord, UpdateDnsRecordParams,
    },
    framework::{
        Environment,
        auth::Credentials,
        client::{ClientConfig, async_api::Client},
        response::{ApiFailure, ApiSuccess},
    },
};

type ApiResult<T> = Result<ApiSuccess<T>, ApiFailure>;

pub struct CFClient {
    client: Client,
    zone_identifier: String,
}

impl CFClient {
    pub fn new(token: String, zone_identifier: String) -> Self {
        let creds = Credentials::UserAuthToken { token };
        let client = Client::new(creds, ClientConfig::default(), Environment::Production).unwrap();
        Self {
            client,
            zone_identifier,
        }
    }

    pub async fn list_dns_records(&self) -> ApiResult<Vec<DnsRecord>> {
        let request = ListDnsRecords {
            zone_identifier: &self.zone_identifier,
            params: ListDnsRecordsParams::default(),
        };
        self.client.request(&request).await
    }

    pub async fn update_dns_record(&self, addr: Ipv4Addr) -> ApiResult<DnsRecord> {
        let request = UpdateDnsRecord {
            zone_identifier: &self.zone_identifier,
            identifier: "ingest.luau.info",
            params: UpdateDnsRecordParams {
                ttl: Some(60),
                proxied: Some(false),
                name: "ingest.luau.info",
                content: DnsContent::A { content: addr },
            },
        };
        self.client.request(&request).await
    }

    pub async fn create_dns_record(&self, addr: Ipv4Addr) -> ApiResult<DnsRecord> {
        let request = CreateDnsRecord {
            zone_identifier: &self.zone_identifier,
            params: CreateDnsRecordParams {
                ttl: Some(60),
                priority: None,
                proxied: Some(false),
                name: "ingest.luau.info",
                content: DnsContent::A { content: addr },
            },
        };
        self.client.request(&request).await
    }
}
