use cloudflare::{
    endpoints::{
        dns::dns::{ListDnsRecords, ListDnsRecordsParams, UpdateDnsRecord, UpdateDnsRecordParams, DnsRecord},
    },
    framework::{
        auth::Credentials,
        response::{ApiSuccess, ApiFailure},
        client::{async_api::Client, ClientConfig},
        Environment,
        Error as CloudflareError,
    }
};

type ApiResult<T> = Result<ApiSuccess<T>, ApiFailure>;

pub async fn get_cloudflare_client(token: String) -> Result<Client, CloudflareError> {
    let creds = Credentials::UserAuthToken { token };
    Client::new(creds, ClientConfig::default(), Environment::Production)
}

pub async fn list_dns_records(client: &Client, zone_identifier: String, params: ListDnsRecordsParams) -> ApiResult<Vec<DnsRecord>> {
    let request = ListDnsRecords { zone_identifier: &zone_identifier, params };
    client.request(&request).await
}

pub async fn update_dns_record(client: &Client, zone_identifier: String, identifier: String, params: UpdateDnsRecordParams<'_>) -> ApiResult<DnsRecord> {
    let request = UpdateDnsRecord { zone_identifier: &zone_identifier, identifier: &identifier, params };
    client.request(&request).await
}