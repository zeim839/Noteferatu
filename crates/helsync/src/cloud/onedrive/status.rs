use serde::{Deserialize, Serialize};

/// Reports the status of a long-running operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum StatusReport {

    /// The work has been enqueued but not yet picked up.
    NotStarted,

    /// The work is being actively processed.
    InProgress,

    /// The work has been completed.
    Completed,

    /// The work failed.
    Failed,

    /// The work was cancelled.
    Cancelled,

    /// The work was interrupted, but will be tried again.
    Waiting,

    /// The work was cancelled, but processing has not yet aborted.
    CancelPending,
}

/// To check on the status of long-running actions, the OneDrive API
/// provides a [JobStatus] resource which monitors the progress of
/// the action.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobStatus {

    /// Reports the progress of the operation as a percentage of its
    /// completion.
    pub percentage_complete: f64,

    /// A unique identifier for the results.
    pub resource_id: String,

    /// Status of the operation.
    pub status: StatusReport,
}
