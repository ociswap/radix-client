use std::fmt::Debug;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Default, Serialize)]
pub struct GetMempoolTransactionsRequest {
    pub network: String,
    pub payload_hashes: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct GetMempoolTransactions200Response {
    pub count: u32,
    pub payloads: Vec<MempoolTransactionPayloads>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct MempoolTransactionPayloads {
    pub hash: String,
    pub hash_bech32m: String,
    pub hex: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default, Serialize)]
pub struct GetMempoolListRequest {
    pub network: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct GetMempoolList200Response {
    pub contents: Vec<MempoolTransactionHashes>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct MempoolTransactionHashes {
    pub intent_hash: String,
    pub intent_hash_bech32m: String,
    pub payload_hash: String,
    pub payload_hash_bech32m: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub enum PublicKeyType {
    #[default]
    EcdsaSecp256k1,
    EddsaEd25519,
}

#[derive(Serialize, Deserialize)]
pub struct PublicKey {
    pub key_type: PublicKeyType,
    // The hex-encoded compressed EdDSA Ed25519 public key (32 bytes)
    pub key_hex: String,
}

#[derive(Serialize, Deserialize)]
pub struct PreviewTransactionFlags {
    pub use_free_credit: bool,
    pub assume_all_signature_proofs: bool,
    pub skip_epoch_check: bool,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionPreviewRequestBody {
    pub network: String,
    pub manifest: String,
    pub blobs_hex: Option<Vec<String>>,
    pub start_epoch_inclusive: i64,
    pub end_epoch_exclusive: i64,
    pub notary_public_key: Option<PublicKey>,
    pub notary_is_signatory: Option<bool>,
    pub tip_percentage: i32,
    pub nonce: i64,
    pub signer_public_keys: Vec<PublicKey>,
    pub flags: PreviewTransactionFlags,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TransactionPreview200ResponseBody {
    pub encoded_receipt: String,
    pub receipt: Receipt,
    pub instruction_resource_changes: Vec<InstructionResourceChanges>,
    pub logs: Vec<Log>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Core4XXResponseBody {
    pub message: String,
    pub code: Option<i16>,
    pub details: Option<GatewayError>,
    pub trace_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GatewayError {
    pub r#type: GatewayErrorType,
    pub address: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GatewayErrorType {
    EntityNotFoundError,
    InvalidEntityError,
    NotSyncedUpError,
    InvalidRequestError,
    InvalidTransactionError,
    TransactionNotFoundError,
    InternalServerError,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstructionResourceChanges {
    pub index: u64,
    pub resource_changes: Vec<ResourceChange>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceChange {
    pub resource_address: String,
    pub component_entity: EntityReference,
    pub vault_entity: EntityReference,
    pub amount: Decimal,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Log {
    pub level: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Receipt {
    pub status: Status,
    pub fee_summary: FeeSummary,
    pub costing_parameters: CostingParameters,
    pub fee_source: Option<FeeSource>,
    pub fee_destination: Option<FeeDestination>,
    pub state_updates: StateUpdates,
    pub events: Option<Vec<Event>>,
    pub next_epoch: Option<NextEpoch>,
    pub output: Option<Vec<SborData>>,
    pub error_message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NextEpoch {
    pub epoch: u64,
    pub validators: Vec<ActiveValidator>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ActiveValidator {
    pub address: String,
    pub key: EcdsaSecp256k1PublicKey,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct EcdsaSecp256k1PublicKey {
    pub key_type: PublicKeyType,
    pub key_hex: String,
    pub stake: Option<Decimal>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub r#type: EventTypeIdentifier,
    pub data: SborData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "emitter")]
pub enum EmitterType {
    Function {
        package_address: String,
        blueprint_name: String,
    },
    Method {
        entity: EntityReference,
        object_module_id: ModuleID,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ModuleID {
    Main,
    Metadata,
    Royalty,
    RoleAssignment,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventTypeIdentifier {
    pub emitter: EventEmitterIdentifier,
    pub type_reference: PackageTypeReference,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum EventEmitterIdentifier {
    Function {
        package_address: String,
        blueprint_name: String,
    },
    Method {
        entity: EntityReference,
        object_module_id: ModuleID,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackageTypeReference {
    pub full_type_id: FullyScopedTypeId,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FullyScopedTypeId {
    pub entity_address: String,
    pub schema_hash: String,
    pub local_type_id: LocalTypeId,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalTypeId {
    pub kind: LocalTypeIdKind,
    pub id: u64,
    pub as_sbor: SborData,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SborData {
    pub hex: Option<String>,
    pub programmatic_json: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LocalTypeIdKind {
    WellKnown,
    SchemaLocal,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EventEmitterIdentifierType {
    Function,
    Method,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub enum Status {
    #[default]
    Succeeded,
    Failed,
    Rejected,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct FeeSource {
    pub from_vaults: Vec<PaymentFromVault>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentFromVault {
    pub vault_entity: EntityReference,
    pub xrd_amount: Decimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntityReference {
    pub entity_type: EntityType,
    pub is_global: bool,
    pub entity_address: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EntityType {
    GlobalPackage,
    GlobalConsensusManager,
    GlobalValidator,
    GlobalGenericComponent,
    GlobalAccount,
    GlobalIdentity,
    GlobalAccessController,
    GlobalVirtualSecp256k1Account,
    GlobalVirtualSecp256k1Identity,
    GlobalVirtualEd25519Account,
    GlobalVirtualEd25519Identity,
    GlobalFungibleResource,
    InternalFungibleVault,
    GlobalNonFungibleResource,
    InternalNonFungibleVault,
    InternalGenericComponent,
    InternalKeyValueStore,
    GlobalOneResourcePool,
    GlobalTwoResourcePool,
    GlobalMultiResourcePool,
    GlobalTransactionTracker,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct FeeDestination {
    pub to_proposer: Decimal,
    pub to_validator_set: Decimal,
    pub to_burn: Decimal,
    pub to_royalty_recipients: Vec<PaymentToRoyaltyRecipient>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentToRoyaltyRecipient {
    pub royalty_recipient: EntityReference,
    pub xrd_amount: Decimal,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct FeeSummary {
    pub execution_cost_units_consumed: u64,
    pub finalization_cost_units_consumed: u64,
    pub xrd_total_execution_cost: Decimal,
    pub xrd_total_finalization_cost: Decimal,
    pub xrd_total_royalty_cost: Decimal,
    pub xrd_total_storage_cost: Decimal,
    pub xrd_total_tipping_cost: Decimal,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CostingParameters {
    pub execution_cost_unit_price: Decimal,
    pub execution_cost_unit_limit: u64,
    pub execution_cost_unit_loan: u64,
    pub finalization_cost_unit_price: Decimal,
    pub finalization_cost_unit_limit: u64,
    pub xrd_usd_price: Decimal,
    pub xrd_storage_price: Decimal,
    pub tip_percentage: u8,
}

// Cutting some corners here until I ever need this.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct StateUpdates {
    pub deleted_partitions: Vec<serde_json::Value>, // Assuming it's a list of generic JSON values
    pub created_substates: Vec<serde_json::Value>, // Assuming it's a list of generic JSON values
    pub updated_substates: Vec<serde_json::Value>, // Assuming it's a list of generic JSON values
    pub deleted_substates: Vec<serde_json::Value>, // Assuming it's a list of generic JSON values
    pub new_global_entities: Vec<serde_json::Value>, // Assuming it's a list of generic JSON values
}

#[derive(Serialize, Deserialize)]
pub struct TransactionSubmitRequestBody {
    pub network: String,
    pub notarized_transaction_hex: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TransactionStreamRequestBody {
    pub network: String,
    pub from_state_version: u64,
    pub limit: u32,
    pub sbor_format_options: SborFormatOptions,
    pub transaction_format_options: TransactionFormatOptions,
    pub substate_format_options: SubstateFormatOptions,
    pub include_proofs: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SborFormatOptions {
    pub raw: bool,
    pub programmatic_json: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TransactionFormatOptions {
    pub manifest: bool,
    pub blobs: bool,
    pub message: bool,
    pub raw_system_transaction: bool,
    pub raw_notarized_transaction: bool,
    pub raw_ledger_transaction: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SubstateFormatOptions {
    pub raw: bool,
    pub hash: bool,
    pub typed: bool,
    pub previous: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TransactionStream200ResponseBody {
    // pub previous_state_identifier: Option<CommittedStateIdentifier>,
    pub from_state_version: Option<u64>,
    pub count: u32,
    pub max_ledger_state_version: u64,
    pub transactions: Vec<CommittedTransaction>,
    // pub proofs: Option<Vec<LedgerProof>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommittedTransaction {
    pub resultant_state_identifiers: CommittedStateIdentifier,
    pub ledger_transaction: LedgerTransactionType,
    pub receipt: Receipt,
    pub proposer_timestamp_ms: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommittedStateIdentifier {
    pub state_version: u64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct LedgerTransaction {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum LedgerTransactionType {
    Genesis(GenesisLedgerTransaction),
    User(UserLedgerTransaction),
    RoundUpdate(RoundUpdateLedgerTransaction),
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct GenesisLedgerTransaction {
    payload_hex: Option<String>,
    is_flash: bool,
    system_transaction: Option<SystemTransaction>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SystemTransaction {
    pub payload_hex: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct UserLedgerTransaction {
    pub payload_hex: Option<String>,
    pub notarized_transaction: NotarizedTransaction,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NotarizedTransaction {
    pub hash: String,
    pub hash_bech32m: String,
    pub payload_hex: Option<String>,
    pub signed_intent: SignedTransactionIntent,
    // pub notary_signature: Signature,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SignedTransactionIntent {
    pub hash: String,
    pub hash_bech32m: String,
    pub intent: TransactionIntent,
    // pub intent_signatures: Vec<SignatureWithPublicKey>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TransactionIntent {
    pub hash: String,
    pub hash_bech32m: String,
    pub header: TransactionHeader,
    pub instructions: Option<String>,
    // pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TransactionHeader {
    pub network_id: u32,
    pub start_epoch_inclusive: u64,
    pub end_epoch_exclusive: u64,
    pub nonce: u64,
    // pub notary_public_key: PublicKey,
    pub notary_is_signatory: bool,
    pub tip_percentage: u32,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RoundUpdateLedgerTransaction {
    // pub payload_hex: Option<String>,
}

pub enum CoreApiError {
    Network(reqwest::Error),
    Parsing {
        serde_error: serde_json::Error,
        response: String,
    },
    ClientError(Core4XXResponseBody),
    ServerError(String),
    Unknown,
}

impl std::fmt::Display for CoreApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CoreApiError::Network(e) => write!(f, "Network error: {}", e),
            CoreApiError::Parsing {
                serde_error,
                response,
            } => write!(
                f,
                "Parsing error: {}: Excerpt: {:#?}",
                serde_error,
                response.chars().take(1000).collect::<String>().to_string()
            ),
            CoreApiError::ClientError(e) => {
                write!(f, "Client error: {:?}", e)
            }
            CoreApiError::ServerError(e) => write!(f, "Server error: {}", e),
            CoreApiError::Unknown => write!(f, "Unknown error"),
        }
    }
}

impl Debug for CoreApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Use the Display implementation
        write!(f, "{}", self)
    }
}

impl From<reqwest::Error> for CoreApiError {
    fn from(e: reqwest::Error) -> Self {
        CoreApiError::Network(e)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transactionsubmit200ResponseBody {
    pub duplicate: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScryptoCallPreviewRequestBody {
    pub network: String,
    pub target: TargetIdentifier,
    pub arguments: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum TargetIdentifier {
    Function {
        package_address: String,
        blueprint_name: String,
        function_name: String,
    },
    Method {
        component_address: String,
        method_name: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScryptoCallPreview200ResponseBody {
    pub at_ledger_state: LedgerStateSummary,
    pub status: String,
    pub output: Option<SborData>,
    pub error_message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LedgerStateSummary {
    // NOT IMPLEMENTED
    // pub state_version: u64,
    // pub header_summary: LedgerHeaderSummary,
}
