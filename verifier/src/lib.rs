pub mod types;
pub mod verification;

pub use types::{
    AcpiTables, ErrorResponse, RtmrEventEntry, RtmrEventStatus, RtmrMismatch, VerificationDetails,
    VerificationRequest, VerificationResponse,
};
pub use verification::CvmVerifier;

// Re-export Attestation from ra_tls for convenience
pub use ra_tls::attestation::Attestation;
