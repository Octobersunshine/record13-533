use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use crate::models::{Script, Room, Session, Booking};

#[derive(Clone, Default)]
pub struct AppState {
    pub scripts: Arc<Mutex<HashMap<Uuid, Script>>>,
    pub rooms: Arc<Mutex<HashMap<Uuid, Room>>>,
    pub sessions: Arc<Mutex<HashMap<Uuid, Session>>>,
    pub bookings: Arc<Mutex<HashMap<Uuid, Booking>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }
}
