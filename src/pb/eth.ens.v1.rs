// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Domains {
    #[prost(message, repeated, tag="1")]
    pub domains: ::prost::alloc::vec::Vec<Domain>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Domain {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub label_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub label_hash: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NameTransfers {
    #[prost(message, repeated, tag="1")]
    pub name_transfers: ::prost::alloc::vec::Vec<NameTransfer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NameTransfer {
    #[prost(message, optional, tag="1")]
    pub from: ::core::option::Option<Account>,
    #[prost(message, optional, tag="2")]
    pub to: ::core::option::Option<Account>,
    #[prost(string, tag="3")]
    pub token_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="5")]
    pub tx_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="4")]
    pub block_number: u64,
    #[prost(uint32, tag="6")]
    pub log_index: u32,
}
// @@protoc_insertion_point(module)
