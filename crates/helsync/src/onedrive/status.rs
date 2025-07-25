use serde::{Deserialize, Serialize};

/// Reports the status of a long-running operation.
#[derive(Serialize, Deserialize, PartialEq)]
pub enum StatusReport {

    /// The work has been enqueued but not yet picked up.
    #[serde(rename = "notStarted")]
    NotStarted,

    /// The work is being actively processed.
    #[serde(rename = "inProgress")]
    InProgress,

    /// The work has been completed.
    #[serde(rename = "completed")]
    Completed,

    /// The work failed.
    #[serde(rename = "failed")]
    Failed,

    /// The work was cancelled.
    #[serde(rename = "cancelled")]
    Cancelled,

    /// The work was interrupted, but will be tried again.
    #[serde(rename = "waiting")]
    Waiting,

    /// The work was cancelled, but processing has not yet aborted.
    #[serde(rename = "cancelPending")]
    CancelPending,
}

/// To check on the status of long-running actions, the OneDrive API
/// provides a [JobStatus] resource which monitors the progress of
/// the action.
#[derive(Serialize, Deserialize)]
pub struct JobStatus {

    /// Reports the progress of the operation as a percentage of its
    /// completion.
    #[serde(rename = "percentageComplete")]
    pub percentage_complete: f64,

    /// A unique identifier for the results.
    #[serde(rename = "resourceId")]
    pub resource_id: String,

    /// Status of the operation.
    pub status: StatusReport,
}
