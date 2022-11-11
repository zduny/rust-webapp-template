use std::{sync::atomic::{AtomicUsize, Ordering}, collections::HashMap};

use common::message::server;
use tokio::sync::mpsc::UnboundedSender;

static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(Debug)]
pub struct User {
    pub id: usize,
    pub name: String,
    pub sender: UnboundedSender<server::Message>,
}

impl User {
    pub fn new(sender: UnboundedSender<server::Message>) -> Self {
        let id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);
        User {
            id,
            name: format!("User {id}"),
            sender,
        }
    }
}

#[derive(Debug, Default)]
pub struct State {
    pub users: HashMap<usize, User>,
}
