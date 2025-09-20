use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static RESULT_NAMES: Lazy<HashMap<u32, &'static str>> = Lazy::new(|| {
	let mut m = HashMap::new();
	// Core
	m.insert(0x00010001, "Core::Unknown");
	m.insert(0x00010002, "Core::NotImplemented");
	m.insert(0x00010003, "Core::InvalidPointer");
	m.insert(0x00010004, "Core::OperationAborted");
	m.insert(0x00010005, "Core::Exception");
	m.insert(0x00010006, "Core::AccessDenied");
	m.insert(0x00010007, "Core::InvalidHandle");
	m.insert(0x00010008, "Core::InvalidIndex");
	m.insert(0x00010009, "Core::OutOfMemory");
	m.insert(0x0001000A, "Core::InvalidArgument");
	m.insert(0x0001000B, "Core::Timeout");
	m.insert(0x0001000C, "Core::InitializationFailure");
	m.insert(0x0001000D, "Core::CallInitiationFailure");
	m.insert(0x0001000E, "Core::RegistrationError");
	m.insert(0x0001000F, "Core::BufferOverflow");
	m.insert(0x00010010, "Core::InvalidLockState");
	m.insert(0x00010011, "Core::InvalidSequence");
	m.insert(0x00010012, "Core::SystemError");
	m.insert(0x00010013, "Core::Cancelled");
	// DDL
	m.insert(0x00020001, "DDL::InvalidSignature");
	m.insert(0x00020002, "DDL::IncorrectVersion");
	// RendezVous (partial list for brevity)
	m.insert(0x00030001, "RendezVous::ConnectionFailure");
	m.insert(0x00030002, "RendezVous::NotAuthenticated");
	m.insert(0x00030064, "RendezVous::InvalidUsername");
	m.insert(0x00030065, "RendezVous::InvalidPassword");
	m.insert(0x00030066, "RendezVous::UsernameAlreadyExists");
	m.insert(0x00030067, "RendezVous::AccountDisabled");
	m.insert(0x00030068, "RendezVous::AccountExpired");
	m.insert(0x00030069, "RendezVous::ConcurrentLoginDenied");
	m.insert(0x0003006A, "RendezVous::EncryptionFailure");
	m.insert(0x0003006B, "RendezVous::InvalidPID");
	m.insert(0x0003006C, "RendezVous::MaxConnectionsReached");
	m.insert(0x0003006D, "RendezVous::InvalidGID");
	m.insert(0x0003006E, "RendezVous::InvalidControlScriptID");
	m.insert(0x0003006F, "RendezVous::InvalidOperationInLiveEnvironment");
	m.insert(0x00030070, "RendezVous::DuplicateEntry");
	m.insert(0x00030071, "RendezVous::ControlScriptFailure");
	m.insert(0x00030072, "RendezVous::ClassNotFound");
	m.insert(0x00030073, "RendezVous::SessionVoid");
	m.insert(0x00030075, "RendezVous::DDLMismatch");
	m.insert(0x00030076, "RendezVous::InvalidConfiguration");
	m.insert(0x000300C8, "RendezVous::SessionFull");
	m.insert(0x000300C9, "RendezVous::InvalidGatheringPassword");
	m.insert(0x000300CA, "RendezVous::WithoutParticipationPeriod");
	m.insert(0x000300CB, "RendezVous::PersistentGatheringCreationMax");
	m.insert(0x000300CC, "RendezVous::PersistentGatheringParticipationMax");
	m.insert(0x000300CD, "RendezVous::DeniedByParticipants");
	m.insert(0x000300CE, "RendezVous::ParticipantInBlackList");
	m.insert(0x000300CF, "RendezVous::GameServerMaintenance");
	m.insert(0x000300D0, "RendezVous::OperationPostpone");
	m.insert(0x000300D1, "RendezVous::OutOfRatingRange");
	m.insert(0x000300D2, "RendezVous::ConnectionDisconnected");
	m.insert(0x000300D3, "RendezVous::InvalidOperation");
	m.insert(0x000300D4, "RendezVous::NotParticipatedGathering");
	m.insert(0x000300D5, "RendezVous::MatchmakeSessionUserPasswordUnmatch");
	m.insert(0x000300D6, "RendezVous::MatchmakeSessionSystemPasswordUnmatch");
	m.insert(0x000300D7, "RendezVous::UserIsOffline");
	m.insert(0x000300D8, "RendezVous::AlreadyParticipatedGathering");
	m.insert(0x000300D9, "RendezVous::PermissionDenied");
	m.insert(0x000300DA, "RendezVous::NotFriend");
	m.insert(0x000300DB, "RendezVous::SessionClosed");
	m.insert(0x000300DC, "RendezVous::DatabaseTemporarilyUnavailable");
	m.insert(0x000300DD, "RendezVous::InvalidUniqueID");
	m.insert(0x000300DE, "RendezVous::MatchmakingWithdrawn");
	m.insert(0x000300DF, "RendezVous::LimitExceeded");
	m.insert(0x000300E0, "RendezVous::AccountTemporarilyDisabled");
	m.insert(0x000300E1, "RendezVous::PartiallyServiceClosed");
	m.insert(0x000300E2, "RendezVous::ConnectionDisconnectedForConcurrentLogin");
	// For brevity, additional categories omitted; extend as needed.
	m
});

pub fn result_code_to_name(code: u32) -> String {
	RESULT_NAMES
		.get(&code)
		.copied()
		.unwrap_or("Invalid Result Code")
		.to_string()
}


