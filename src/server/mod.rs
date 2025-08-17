//! # MQTT Server Module
//! 
//! This module provides a comprehensive MQTT broker/server implementation that can handle
//! multiple client connections, manage subscriptions, route messages, and maintain session
//! state. It supports both MQTT 3.1.1 and MQTT 5.0 protocols with full broker functionality.

pub mod config;
pub mod auth;
pub mod session;
pub mod connection;
pub mod router;

pub use config::ServerConfig;
pub use auth::Authentication;
pub use session::{Session, Subscription};
pub use connection::ServerConnection;
pub use router::MessageRouter;

use crate::error::Result;
use log::info;
use tokio::net::TcpListener;
use std::sync::Arc;


use self::session::SessionManager;
use self::router::MessageRouter as Router;

/// MQTT server
pub struct Server {
    config: ServerConfig,
    listener: Option<TcpListener>,
    session_manager: Arc<SessionManager>,
    message_router: Arc<Router>,
}

impl Server {
    /// Create a new MQTT server
    pub fn new(config: ServerConfig) -> Self {
        Self {
            config,
            listener: None,
            session_manager: Arc::new(SessionManager::new()),
            message_router: Arc::new(Router::new()),
        }
    }

    /// Start the server
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting MQTT server on {}", self.config.bind_addr);
        
        let listener = TcpListener::bind(&self.config.bind_addr).await
            .map_err(|e| crate::error::Error::Server(format!("Failed to bind to {}: {}", self.config.bind_addr, e)))?;
        
        self.listener = Some(listener);
        info!("MQTT server started successfully");

        self.accept_connections().await
    }

    /// Accept incoming connections
    async fn accept_connections(&self) -> Result<()> {
        let listener = self.listener.as_ref()
            .ok_or_else(|| crate::error::Error::Server("Server not started".to_string()))?;

        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    info!("New connection from {}", addr);
                    
                    let config = self.config.clone();
                    let session_manager = Arc::clone(&self.session_manager);
                    let message_router = Arc::clone(&self.message_router);
                    
                    tokio::spawn(async move {
                        if let Err(e) = ServerConnection::handle_connection(
                            stream,
                            addr,
                            config,
                            session_manager,
                            message_router,
                        ).await {
                            log::error!("Connection error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    log::error!("Accept error: {}", e);
                }
            }
        }
    }
}
