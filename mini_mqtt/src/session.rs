use chrono;

use crate::{errors, packets};

pub mod handler;

// ClientId is the identifier of the client. This is defined by the MQTT v5.0 protocol.
// You can confirm them at the 3.1.3.1 Client Identifier (ClientID) subsection.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ClientId(String);

impl ClientId {
    pub fn new(id: &str) -> Result<ClientId, errors::Error> {
        packets::connect::validate_client_id(id)?;

        Ok(ClientId(id.to_string()))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

// Session Id is the identifier of the session.
// This is a concept in this MQTT implementation.
// Sometimes, the ClientID concept is not enough to manage the session.
// For example, in the future, the implementation will support the session management concept,
// the ClientID concept will be limited to enhance the session management.
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct SessionId(u32);

impl SessionId {
    pub fn new(id: u32) -> SessionId {
        SessionId(id)
    }
}

// SessionState is the state of the session. As a note, it is not the state for the publish and subscribe,
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum SessionState {
    BeforeTcpConnectionEstablished, // TCP connection is not established.
    BeforeConnect, // TCP connection is established, but before the CONNECT Packet is received.
    ReceivedConnect, // CONNECT Packet is received, but before the CONNACK Packet is sent.
    Connected,     // CONNACK Packet is sent, or received several next packets.
    Disconnected,  // DISCONNECT Packet is received, or the TCP connection is closed.
}

// Session represents the session of the client.
// It contains the client_id, the time when the TCP connection is established, the keep_alive time, and the state of the session.
// When the session state changes, you can get a new session instance by the change methods.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Session {
    pub session_id: SessionId,
    pub client_id: ClientId,
    pub tcp_connection_established_at: Option<chrono::DateTime<chrono::Utc>>,
    pub keep_alive: chrono::Duration,
    pub state: SessionState,
}

impl Session {
    pub fn new(session_id: SessionId, client_id: ClientId, keep_alive: chrono::Duration) -> Session {
        Session {
            session_id,
            client_id,
            tcp_connection_established_at: None,
            keep_alive,
            state: SessionState::BeforeTcpConnectionEstablished,
        }
    }

    pub fn tcp_connection_established(&self) -> Result<Session, errors::Error> {
       if self.tcp_connection_established_at.is_some() {
           return Err(errors::Error::Common("TCP connection is already established".to_string()));
       }

        Ok(Session {
            tcp_connection_established_at: Some(chrono::Utc::now()),
            ..self.clone()
        })
    }

    pub fn received_connect(&self) -> Result<Session, errors::Error> {
        if self.tcp_connection_established_at.is_none() {
            return Err(errors::Error::Common(
                format!("TCP connection has not been established,  the {} session's state cannot be changed to ReceivedConnect", self.client_id.as_str()).to_string()));
        }

        Ok(Session {
            state: SessionState::ReceivedConnect,
            ..self.clone()
        })
    }

    pub fn connected(&self) -> Result<Session, errors::Error> {
        if self.tcp_connection_established_at.is_none() {
            return Err(errors::Error::Common(
                format!("TCP connection has not been established,  the {} session's state cannot be changed to Connected", self.client_id.as_str()).to_string()));
        }

        Ok(Session {
            state: SessionState::Connected,
            ..self.clone()
        })
    }

    pub fn disconnected(&self) -> Session {
        Session {
            state: SessionState::Disconnected,
            ..self.clone()
        }
    }
}
