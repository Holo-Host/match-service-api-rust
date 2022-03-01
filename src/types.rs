use rocket::{
    response::Responder,
    serde::{Deserialize, Serialize},
};

use bson::oid::ObjectId;
use mongodb::{bson, error::Error};
use rocket::response::Debug;

// [rocket::response::Debug](https://api.rocket.rs/v0.5-rc/rocket/response/struct.Debug.html) implements Responder to Error
pub type Result<T, E = Debug<Error>> = std::result::Result<T, E>;

// Debug errors default to 500
pub type Error500 = Debug<Error>;

#[derive(Responder, Debug)]
#[response(status = 400)]
pub enum Error400 {
    Info(String),
    Message(&'static str),
}

#[derive(Responder, Debug)]
#[response(status = 401)]

pub enum Error401 {
    Info(String),
    Message(&'static str),
}

#[derive(Responder, Debug)]
#[response(status = 404)]
// Disable warning if any type within 404 enum is unused
#[allow(dead_code)]
pub enum Error404 {
    Info(String),
    Message(&'static str),
}

#[derive(Responder, Debug)]
pub enum ApiError {
    BadRequest(Error400),
    Database(Error500),
    InvalidPayload(Error400),
    MissingRecord(Error404),
    MissingSignature(Error401),
    InvalidSignature(Error401),
}

// Return type for /network/capacity endpoint
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Capacity {
    pub total_hosts: u16,
    pub read_only: u16,
    pub source_chain: u16,
}

impl Capacity {
    pub fn add_host(&mut self, uptime: f32) {
        self.total_hosts += 1;
        if uptime >= 0.5 {
            self.read_only += 1
        };
        if uptime >= 0.9 {
            self.source_chain += 1
        };
    }
}

// Data schema in `performance_summary` collection
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Performance {
    _id: ObjectId,
    name: String,
    description: String,
    #[serde(rename = "physicalAddress")]
    physical_address: Option<String>,
    zt_ipaddress: String,
    created_at: i64,
    pub uptime: f32,
}

// Return type for /hosts/uptime endpoint
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Uptime {
    pub uptime: f32,
}

// Data schema in `holoports_assignment` collection
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct Assignment {
    pub name: String,
}

// Input type for /hosts/stats endpoint
// Data schema in collection `holoport_status`
// Note: We wrap each feild value in Option<T> because if the HPOS `netstatd` fails to collect data, it will send null in failed field.
#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct HostStats {
    pub holo_network: Option<String>,
    pub channel: Option<String>,
    pub holoport_model: Option<String>,
    pub ssh_status: Option<bool>,
    pub zt_ip: Option<String>,
    pub wan_ip: Option<String>,
    pub holoport_id: String,
    pub timestamp: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct NumberInt {
    number_int: u16,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct NumberLong {
    number_long: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct DateCreated {
    date: NumberLong,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct AgentPubKeys {
    pub pub_key: String,
    role: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct RegistrationCode {
    code: String,
    role: String,
    pub agent_pub_keys: Vec<AgentPubKeys>,
}

// Data schema in database `opsconsoledb`, collection `registration`
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct HostRegistration {
    #[serde(skip)]
    _id: ObjectId,
    #[serde(skip)]
    _v: NumberInt,
    given_names: String,
    last_name: String,
    is_jurisdiction_not_in_list: bool,
    legal_jurisdiction: String,
    created: DateCreated,
    old_holoport_ids: Vec<String>,
    pub registration_code: Vec<RegistrationCode>,
}
