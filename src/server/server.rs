use std::env;
use std::sync::Arc;

use ::log::{info, warn};
use tokio::task::JoinHandle;
use tonic::transport::{Certificate, Identity, Server, ServerTlsConfig};
use protos::event::v1::booking_service_server::BookingServiceServer;
use protos::event::v1::closure_service_server::ClosureServiceServer;
use protos::event::v1::event_service_server::EventServiceServer;
use crate::database::{CacheClient, PgPool};
use crate::{create_socket_addr, report_error};
use crate::server::services::v1::booking::booking_service::BookingServiceServerImpl;
use crate::server::services::v1::closure::closure_service::ClosureServiceServerImpl;
use crate::server::services::v1::event::event_service::EventServiceServerImpl;

pub struct TonicServer {
    pub handle: JoinHandle<()>,
    pub tls: bool,
}

pub fn start_server(
    pool: Arc<PgPool>,
    cache_client: CacheClient,
    port: u16,
) -> Result<TonicServer, Box<dyn std::error::Error>> {
    let booking_service = BookingServiceServerImpl::new(pool.clone(), cache_client.clone());
    let event_service = EventServiceServerImpl::new(pool.clone(), cache_client.clone());
    let closure_service = ClosureServiceServerImpl::new(pool.clone(), cache_client.clone());

    let (mut tonic_server, secure_mode) = match get_tls_config() {
        Some(tls) => {
            info!("Configuring TLS...");
            match Server::builder().tls_config(tls) {
                Ok(server) => {
                    info!("TLS successfully configured.");
                    (server, true)
                }
                Err(details) => {
                    info!("Error configuring TLS. Connections are not secure.");
                    report_error(&details);
                    (Server::builder(), false)
                }
            }
        }
        _ => {
            warn!("No TLS keys available. Connections are not secure.");
            (Server::builder(), false)
        }
    };

    let grpc_booking_service = BookingServiceServer::new(booking_service);
    let grpc_event_service = EventServiceServer::new(event_service);
    let grpc_closure_service = ClosureServiceServer::new(closure_service);

    let reflect = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(protos::event::v1::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    let tonic_router = tonic_server
        .add_service(reflect)
        .add_service(grpc_booking_service)
        .add_service(grpc_event_service)
        .add_service(grpc_closure_service);

    let server = tokio::spawn(async move {
        let tonic_addr = create_socket_addr(port);
        info!("Starting server on port {}", port);
        match tonic_router.serve(tonic_addr).await {
            Ok(_) => info!("Server finished on {}", tonic_addr),
            Err(e) => {
                warn!("Unable to start server on port {}", port);
                report_error(&e);
            }
        };
        ()
    });

    Ok(TonicServer {
        handle: server,
        tls: secure_mode,
    })
}

fn get_tls_config() -> Option<ServerTlsConfig> {
    let cert = env::var("TLS_CERT").ok();
    let key = env::var("TLS_KEY").ok();
    let ca_cert = env::var("CA_CERT").ok();

    match (cert, key, ca_cert) {
        (Some(cert), Some(key), Some(ca_cert)) => {
            info!("Configuring TLS with custom CA...");
            Some(
                ServerTlsConfig::new()
                    .identity(Identity::from_pem(cert, key))
                    .client_ca_root(Certificate::from_pem(ca_cert)),
            )
        }
        (Some(cert), Some(key), None) => {
            info!("Configuring TLS with official CAs...");
            Some(ServerTlsConfig::new().identity(Identity::from_pem(cert, key)))
        }
        _ => None,
    }
}
