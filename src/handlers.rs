use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use uuid::Uuid;
use crate::models::*;
use crate::state::AppState;

pub async fn list_scripts(
    State(state): State<AppState>,
) -> Json<Vec<Script>> {
    let scripts = state.scripts.lock().await;
    Json(scripts.values().cloned().collect())
}

pub async fn get_script(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Script>, (StatusCode, Json<ErrorResponse>)> {
    let scripts = state.scripts.lock().await;
    scripts.get(&id)
        .map(|script| Json(script.clone()))
        .ok_or((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error: "Script not found".to_string() }),
        ))
}

pub async fn create_script(
    State(state): State<AppState>,
    Json(payload): Json<CreateScript>,
) -> Json<Script> {
    let script = Script {
        id: Uuid::new_v4(),
        name: payload.name,
        description: payload.description,
        duration_minutes: payload.duration_minutes,
        player_count_min: payload.player_count_min,
        player_count_max: payload.player_count_max,
        created_at: Utc::now().naive_utc(),
    };
    let mut scripts = state.scripts.lock().await;
    scripts.insert(script.id, script.clone());
    Json(script)
}

pub async fn update_script(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateScript>,
) -> Result<Json<Script>, (StatusCode, Json<ErrorResponse>)> {
    let mut scripts = state.scripts.lock().await;
    if let Some(script) = scripts.get_mut(&id) {
        script.name = payload.name;
        script.description = payload.description;
        script.duration_minutes = payload.duration_minutes;
        script.player_count_min = payload.player_count_min;
        script.player_count_max = payload.player_count_max;
        Ok(Json(script.clone()))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error: "Script not found".to_string() }),
        ))
    }
}

pub async fn delete_script(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let mut scripts = state.scripts.lock().await;
    if scripts.remove(&id).is_some() {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error: "Script not found".to_string() }),
        ))
    }
}

pub async fn list_rooms(
    State(state): State<AppState>,
) -> Json<Vec<Room>> {
    let rooms = state.rooms.lock().await;
    Json(rooms.values().cloned().collect())
}

pub async fn get_room(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Room>, (StatusCode, Json<ErrorResponse>)> {
    let rooms = state.rooms.lock().await;
    rooms.get(&id)
        .map(|room| Json(room.clone()))
        .ok_or((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error: "Room not found".to_string() }),
        ))
}

pub async fn create_room(
    State(state): State<AppState>,
    Json(payload): Json<CreateRoom>,
) -> Json<Room> {
    let room = Room {
        id: Uuid::new_v4(),
        name: payload.name,
        capacity: payload.capacity,
        description: payload.description,
        created_at: Utc::now().naive_utc(),
    };
    let mut rooms = state.rooms.lock().await;
    rooms.insert(room.id, room.clone());
    Json(room)
}

pub async fn update_room(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateRoom>,
) -> Result<Json<Room>, (StatusCode, Json<ErrorResponse>)> {
    let mut rooms = state.rooms.lock().await;
    if let Some(room) = rooms.get_mut(&id) {
        room.name = payload.name;
        room.capacity = payload.capacity;
        room.description = payload.description;
        Ok(Json(room.clone()))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error: "Room not found".to_string() }),
        ))
    }
}

pub async fn delete_room(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let mut rooms = state.rooms.lock().await;
    if rooms.remove(&id).is_some() {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error: "Room not found".to_string() }),
        ))
    }
}

pub async fn list_sessions(
    State(state): State<AppState>,
) -> Json<Vec<Session>> {
    let sessions = state.sessions.lock().await;
    Json(sessions.values().cloned().collect())
}

pub async fn get_session(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Session>, (StatusCode, Json<ErrorResponse>)> {
    let sessions = state.sessions.lock().await;
    sessions.get(&id)
        .map(|session| Json(session.clone()))
        .ok_or((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error: "Session not found".to_string() }),
        ))
}

pub async fn create_session(
    State(state): State<AppState>,
    Json(payload): Json<CreateSession>,
) -> Result<Json<Session>, (StatusCode, Json<ErrorResponse>)> {
    let rooms = state.rooms.lock().await;
    let room = rooms.get(&payload.room_id)
        .ok_or((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse { error: "Room not found".to_string() }),
        ))?;

    let scripts = state.scripts.lock().await;
    if !scripts.contains_key(&payload.script_id) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse { error: "Script not found".to_string() }),
        ));
    }

    let total_slots = room.capacity;
    let session = Session {
        id: Uuid::new_v4(),
        script_id: payload.script_id,
        room_id: payload.room_id,
        start_time: payload.start_time,
        end_time: payload.end_time,
        total_slots,
        remaining_slots: total_slots,
        price: payload.price,
        created_at: Utc::now().naive_utc(),
    };

    drop(rooms);
    drop(scripts);

    let mut sessions = state.sessions.lock().await;
    sessions.insert(session.id, session.clone());
    Ok(Json(session))
}

pub async fn update_session(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateSession>,
) -> Result<Json<Session>, (StatusCode, Json<ErrorResponse>)> {
    let mut sessions = state.sessions.lock().await;
    if let Some(session) = sessions.get_mut(&id) {
        session.script_id = payload.script_id;
        session.room_id = payload.room_id;
        session.start_time = payload.start_time;
        session.end_time = payload.end_time;
        session.price = payload.price;
        Ok(Json(session.clone()))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error: "Session not found".to_string() }),
        ))
    }
}

pub async fn delete_session(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let mut sessions = state.sessions.lock().await;
    if sessions.remove(&id).is_some() {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error: "Session not found".to_string() }),
        ))
    }
}

pub async fn create_booking(
    State(state): State<AppState>,
    Json(payload): Json<CreateBooking>,
) -> Result<(StatusCode, Json<Booking>), (StatusCode, Json<ErrorResponse>)> {
    let mut sessions = state.sessions.lock().await;
    let session = sessions.get_mut(&payload.session_id)
        .ok_or((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error: "Session not found".to_string() }),
        ))?;

    if payload.player_count == 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse { error: "Player count must be greater than 0".to_string() }),
        ));
    }

    if session.remaining_slots < payload.player_count {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!(
                    "Not enough slots. Remaining: {}, requested: {}",
                    session.remaining_slots, payload.player_count
                ),
            }),
        ));
    }

    let total_price = session.price * payload.player_count as f64;
    let session_id = session.id;
    let now = Utc::now().naive_utc();
    let payment_deadline = now + chrono::Duration::minutes(15);

    let booking = Booking {
        id: Uuid::new_v4(),
        session_id,
        customer_name: payload.customer_name,
        customer_phone: payload.customer_phone,
        player_count: payload.player_count,
        total_price,
        status: BookingStatus::Confirmed,
        payment_status: PaymentStatus::Pending,
        payment_deadline,
        created_at: now,
    };

    let mut bookings = state.bookings.lock().await;
    session.remaining_slots -= payload.player_count;
    bookings.insert(booking.id, booking.clone());

    Ok((StatusCode::CREATED, Json(booking)))
}

pub async fn list_bookings(
    State(state): State<AppState>,
) -> Json<Vec<Booking>> {
    let bookings = state.bookings.lock().await;
    Json(bookings.values().cloned().collect())
}

pub async fn get_booking(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Booking>, (StatusCode, Json<ErrorResponse>)> {
    let bookings = state.bookings.lock().await;
    bookings.get(&id)
        .map(|booking| Json(booking.clone()))
        .ok_or((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error: "Booking not found".to_string() }),
        ))
}

pub async fn cancel_booking(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Booking>, (StatusCode, Json<ErrorResponse>)> {
    let mut sessions = state.sessions.lock().await;
    let mut bookings = state.bookings.lock().await;

    let booking = bookings.get_mut(&id)
        .ok_or((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error: "Booking not found".to_string() }),
        ))?;

    if booking.status == BookingStatus::Cancelled {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse { error: "Booking already cancelled".to_string() }),
        ));
    }

    let player_count = booking.player_count;
    let session_id = booking.session_id;

    let session = sessions.get_mut(&session_id)
        .ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: "Associated session not found".to_string() }),
        ))?;

    session.remaining_slots = std::cmp::min(
        session.remaining_slots + player_count,
        session.total_slots,
    );

    booking.status = BookingStatus::Cancelled;
    let booking_clone = booking.clone();

    Ok(Json(booking_clone))
}

pub async fn pay_booking(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Booking>, (StatusCode, Json<ErrorResponse>)> {
    let mut bookings = state.bookings.lock().await;
    let booking = bookings.get_mut(&id)
        .ok_or((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error: "Booking not found".to_string() }),
        ))?;

    if booking.status == BookingStatus::Cancelled {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse { error: "Booking has been cancelled".to_string() }),
        ));
    }

    if booking.payment_status == PaymentStatus::Paid {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse { error: "Booking already paid".to_string() }),
        ));
    }

    let now = Utc::now().naive_utc();
    if now > booking.payment_deadline {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse { error: "Payment deadline has passed, please rebook".to_string() }),
        ));
    }

    booking.payment_status = PaymentStatus::Paid;
    Ok(Json(booking.clone()))
}

pub async fn cancel_expired_bookings(state: &AppState) -> u32 {
    let mut cancelled_count = 0;
    let now = Utc::now().naive_utc();

    let mut sessions = state.sessions.lock().await;
    let mut bookings = state.bookings.lock().await;

    let expired_bookings: Vec<(Uuid, Uuid, u32)> = bookings
        .iter()
        .filter(|(_, b)| {
            b.status == BookingStatus::Confirmed
                && b.payment_status == PaymentStatus::Pending
                && now > b.payment_deadline
        })
        .map(|(id, b)| (*id, b.session_id, b.player_count))
        .collect();

    for (booking_id, session_id, player_count) in expired_bookings {
        if let Some(booking) = bookings.get_mut(&booking_id) {
            booking.status = BookingStatus::Cancelled;
        }

        if let Some(session) = sessions.get_mut(&session_id) {
            session.remaining_slots = std::cmp::min(
                session.remaining_slots + player_count,
                session.total_slots,
            );
        }

        cancelled_count += 1;
    }

    cancelled_count
}
