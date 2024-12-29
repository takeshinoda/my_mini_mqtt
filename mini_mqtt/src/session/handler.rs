use std::sync::{Arc};
use crate::session;

pub struct Handler {
    session_id_counter: u32,
    sessions : std::collections::HashMap<session::SessionId, session::Session>,
}

impl Handler {
    pub fn new() -> Arc<std::sync::RwLock<Handler>> {
        Arc::new(std::sync::RwLock::new(Handler {
            session_id_counter: 0,
            sessions: std::collections::HashMap::new(),
        }))
    }

    fn increment_session_id_counter(&mut self) {
        self.session_id_counter += 1;
    }

    pub fn create_session(&mut self, client_id: &session::ClientId, keep_alive: chrono::Duration) -> session::Session {
        self.increment_session_id_counter();

        let session_id = session::SessionId(self.session_id_counter);
        let session = session::Session::new(session_id.clone(), client_id.clone(), keep_alive);
        self.sessions.insert(session_id, session.clone());

        session
    }

    pub fn get_session(&self, session_id: &session::SessionId) -> Option<&session::Session> {
        self.sessions.get(session_id)
    }

    // update_session is used to update the session state.
    // As a note, the session is not mutable, so the getting session and
    // the updating session manipulating should be done in the same thread.
    // As a TODO: it is necessary to change the hashmap supporting lock for each value.
    pub fn update_session(&mut self, session: session::Session) {
        self.sessions.insert(session.session_id.clone(), session);
    }

    pub fn remove_session(&mut self, session_id: session::SessionId) -> Option<session::Session> {
        self.sessions.remove(&session_id)
    }
}
