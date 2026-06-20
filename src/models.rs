use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Script {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub duration_minutes: u32,
    pub player_count_min: u32,
    pub player_count_max: u32,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateScript {
    pub name: String,
    pub description: String,
    pub duration_minutes: u32,
    pub player_count_min: u32,
    pub player_count_max: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub id: Uuid,
    pub name: String,
    pub capacity: u32,
    pub description: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoom {
    pub name: String,
    pub capacity: u32,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub script_id: Uuid,
    pub room_id: Uuid,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub total_slots: u32,
    pub remaining_slots: u32,
    pub price: f64,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSession {
    pub script_id: Uuid,
    pub room_id: Uuid,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub price: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Booking {
    pub id: Uuid,
    pub session_id: Uuid,
    pub customer_name: String,
    pub customer_phone: String,
    pub player_count: u32,
    pub total_price: f64,
    pub status: BookingStatus,
    pub payment_status: PaymentStatus,
    pub payment_deadline: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BookingStatus {
    Confirmed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaymentStatus {
    Pending,
    Paid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBooking {
    pub session_id: Uuid,
    pub customer_name: String,
    pub customer_phone: String,
    pub player_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}
