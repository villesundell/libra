// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use libra_metrics::{
    register_histogram, register_histogram_vec, register_int_counter, register_int_counter_vec,
    register_int_gauge, register_int_gauge_vec, DurationHistogram, Histogram, HistogramVec,
    IntCounter, IntCounterVec, IntGauge, IntGaugeVec,
};
use once_cell::sync::Lazy;

// network send result labels
pub const SEND_SUCCESS_LABEL: &str = "success";
pub const SEND_FAIL_LABEL: &str = "fail";

// msg type labels
pub const SYNC_MSG_LABEL: &str = "sync";
pub const COMMIT_MSG_LABEL: &str = "commit";
pub const CHUNK_REQUEST_MSG_LABEL: &str = "chunk_request";
pub const CHUNK_RESPONSE_MSG_LABEL: &str = "chunk_response";

// msg sender label
pub const CONSENSUS_SENDER_LABEL: &str = "consensus";

// version type labels
pub const COMMITTED_VERSION_LABEL: &str = "committed"; // Version of latest ledger info committed.
pub const SYNCED_VERSION_LABEL: &str = "synced"; // Version of most recent txn that was synced (even if it is not backed by an LI)
pub const TARGET_VERSION_LABEL: &str = "target"; // Version a node is trying to catch up to

// failed channel send type labels
pub const CONSENSUS_SYNC_REQ_CALLBACK: &str = "consensus_sync_req_callback";
pub const WAYPOINT_INIT_CALLBACK: &str = "waypoint_init_callback";

// reconfig result labels
pub const RECONFIG_SUCCESS_LABEL: &str = "success";
pub const RECONFIG_FAIL_LABEL: &str = "fail";

// network error type labels
pub const GENERAL_NETWORK_ERROR_LABEL: &str = "general";
pub const UNEXPECTED_MESSAGE_LABEL: &str = "unexpected_msg";

pub const SUCCESS_LABEL: &str = "success";
pub const FAIL_LABEL: &str = "fail";

/// Counter of pending network events to State Synchronizer
pub static PENDING_STATE_SYNCHRONIZER_NETWORK_EVENTS: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!(
        "libra_state_sync_pending_network_events",
        "Counters(queued,dequeued,dropped) related to pending network notifications for State Synchronizer",
        &["state"]
    ).unwrap()
});

/// Number of sync requests sent from a node
pub static REQUESTS_SENT: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!(
        // metric name
        "libra_state_sync_requests_sent_total",
        // metric description
        "Number of chunk requests sent from a node",
        // metric labels
        &["requested_peer_id", "result"]
    )
    .unwrap()
});

pub static RESPONSES_SENT: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!(
        "libra_state_sync_responses_sent_total",
        "Number of chunk responses sent from a node",
        &["recipient", "result"]
    )
    .unwrap()
});

/// Number of Success results of applying a chunk
pub static APPLY_CHUNK_SUCCESS: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!(
        "libra_state_sync_apply_chunk_success_total",
        "Number of Success results of applying a chunk",
        &["chunk_sender_id"]
    )
    .unwrap()
});

/// Number of failed attempts to apply a chunk
pub static APPLY_CHUNK_FAILURE: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!(
        "libra_state_sync_apply_chunk_failure_total",
        "Number of failed attempts to apply a chunk",
        &["chunk_sender_id"]
    )
    .unwrap()
});

pub static PROCESS_CHUNK_REQUEST_COUNT: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!(
        "libra_state_sync_process_chunk_request_total",
        "Number of times chunk request was processed",
        &["result", "sender"]
    )
    .unwrap()
});

/// Average number of transactions in a received chunk response
pub static STATE_SYNC_CHUNK_SIZE: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        "libra_state_sync_chunk_size",
        "Number of transactions in a state sync chunk response",
        &["sender"]
    )
    .unwrap()
});

/// Number of peers that are currently active and upstream.
/// They are the set of nodes a node can make sync requests to
pub static ACTIVE_UPSTREAM_PEERS: Lazy<IntGauge> = Lazy::new(|| {
    register_int_gauge!(
        "libra_state_sync_active_upstream_peers",
        "Number of upstream peers that are currently active"
    )
    .unwrap()
});

/// Notice: this metric is used in CT full node health check
/// ~/libra/testsuite/cluster-test/health/fullnode_check.rs
/// please make corresponding changes if this field is updated
pub static VERSION: Lazy<IntGaugeVec> = Lazy::new(|| {
    register_int_gauge_vec!(
        "libra_state_sync_version",
        "Version involved in state sync progress",
        &["type"] // see version type labels above
    )
    .unwrap()
});

pub static EPOCH: Lazy<IntGauge> = Lazy::new(|| {
    register_int_gauge!("libra_state_sync_epoch", "Current epoch in local state").unwrap()
});

/// How long it takes to make progress, from requesting a chunk to processing the response and
/// committing the block
pub static SYNC_PROGRESS_DURATION: Lazy<DurationHistogram> = Lazy::new(|| {
    DurationHistogram::new(
        register_histogram!(
            "libra_state_sync_sync_progress_duration_s",
            "Histogram of time it takes to sync a chunk, from requesting a chunk to processing the response and committing the block"
        )
        .unwrap()
    )
});

/// Number of timeouts that occur during sync
pub static TIMEOUT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "libra_state_sync_timeout_total",
        "Number of timeouts that occur during sync"
    )
    .unwrap()
});

pub static PROCESS_SYNC_REQUEST_FAILURE: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "libra_state_sync_sync_request_failure_total",
        "Number of sync requests (from consensus) that state sync failed to process"
    )
    .unwrap()
});

pub static SYNC_REQUEST_TIMEOUT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "libra_state_sync_sync_request_timeout_total",
        "Number of sync requests (from consensus) that timed out"
    )
    .unwrap()
});

/// Number of timeouts that occur during the commit flow across consensus, state sync, and mempool
pub static COMMIT_TIMEOUT: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!(
        "libra_state_sync_commit_timeout_total",
        "Number of timeouts that occur during the commit flow across consensus, state sync, and mempool",
        &["component"] // component with which state sync timed out with: consensus, mempool
    )
        .unwrap()
});

pub static FAILED_CHANNEL_SEND: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!(
        "libra_state_sync_failed_channel_sends_total",
        "Number of times a channel send failed in state sync",
        &["type"] // type of channel sends:
    )
    .unwrap()
});

/// Time it takes for state sync to fully execute a chunk (via executor proxy)
pub static EXECUTE_CHUNK_DURATION: Lazy<Histogram> = Lazy::new(|| {
    register_histogram!(
        "libra_state_sync_execute_chunk_duration_s",
        "Histogram of time it takes for state sync's executor proxy to fully execute a chunk"
    )
    .unwrap()
});

/// Number of times a long-poll subscription is delivered
pub static SUBSCRIPTION_DELIVERY_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "libra_state_sync_subscription_delivery_count",
        "Number of times a node delivers a subscription for FN long-poll"
    )
    .unwrap()
});

/// Time it takes to process a state sync message
pub static PROCESS_MSG_LATENCY: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        "libra_state_sync_process_msg_latency",
        "Time it takes to process a message in state sync",
        &["type", "sender"]
    )
    .unwrap()
});

pub static RECONFIG_PUBLISH_COUNT: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!(
        "libra_state_sync_reconfig_count",
        "Number of times on-chain reconfig notification is published in state sync",
        &["result"]
    )
    .unwrap()
});

pub static STORAGE_READ_FAIL_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "libra_state_sync_storage_read_fail_count",
        "Number of times storage read failed in state sync"
    )
    .unwrap()
});

pub static NETWORK_ERROR_COUNT: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!(
        "libra_state_sync_network_error_count",
        "Number of network errors encountered in state sync",
        &["type"]
    )
    .unwrap()
});
