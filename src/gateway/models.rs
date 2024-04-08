use std::fmt::Debug;

use chrono::Utc;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StateEntityDetailsRequest {
    pub at_ledger_state: Option<LedgerStateSelector>,
    pub opt_ins: Option<StateEntityDetailsRequestOptIns>,
    pub addresses: Vec<String>,
    pub aggregation_level: Option<AggregationLevel>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StateEntityDetails200Response {
    pub ledger_state: LedgerState,
    pub items: Vec<StateEntityDetailsResponseItem>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StateEntityDetailsResponseItem {
    pub address: String,
    // Skipping some items here.
    pub fungible_resources: FungibleResourcesCollection,
    // non_fungible_resources: Vec<NonFungibleResourcesCollection>,
    // ancestor_identities: Vec<StateEntityDetailsResponseItemAncestorIdentities>,
    // metadata: EntityMetadataCollection,
    // explicit_metadata: EntityMetadataCollection,
    pub details: StateEntityDetailsResponseItemDetails,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FungibleResourcesCollection {
    pub total_count: Option<u64>,
    pub next_cursor: Option<String>,
    pub items: Vec<FungibleResourcesCollectionItem>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum StateEntityDetailsResponseItemDetails {
    Component(StateEntityDetailsResponseItemDetailsComponent),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StateEntityDetailsResponseItemDetailsComponent {
    pub package_address: Option<String>,
    pub blueprint_name: String,
    pub blueprint_version: String,
    pub state: serde_json::Value,
    // role_assignments: ComponentEntityRoleAssignments,
    // royalty_vault_balance
}

#[derive(Clone, Debug, Deserialize, Serialize)]
// Left this empty, don't need it yet.
pub struct StateEntityDetailsRequestOptIns {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StateEntityFungiblesPageRequest {
    pub at_ledger_state: Option<LedgerStateSelector>,
    pub cursor: Option<String>,
    pub limit_per_page: Option<u32>,
    pub address: String,
    pub aggregation_level: Option<AggregationLevel>,
    pub opt_ins: Option<StateEntityFungiblesPageRequestOptIns>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AggregationLevel {
    Global,
    Vault,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct LedgerStateSelector {
    pub state_version: Option<u64>,
    pub timestamp: Option<u64>,
    pub epoch: Option<u64>,
    pub round: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StateEntityFungiblesPageRequestOptIns {
    explicit_metadata: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StateEntityFungiblesPage200Response {
    pub ledger_state: LedgerState,
    pub total_count: Option<u64>,
    pub next_cursor: Option<String>,
    pub items: Vec<FungibleResourcesCollectionItem>,
    pub address: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "aggregation_level")]
pub enum FungibleResourcesCollectionItem {
    Global {
        resource_address: String,
        // explicit_metadata: Option<Vec<String>>,
        amount: Decimal,
        last_updated_at_state_version: u64,
    },
    Vault {
        resource_address: String,
        // explicit_metadata: Option<Vec<String>>,
        vaults: FungibleResourcesCollectionItemVaultAggregatedVault,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FungibleResourcesCollectionItemVaultAggregatedVault {
    pub total_count: Option<u64>,
    pub next_cursor: Option<String>,
    pub items: Vec<FungibleResourcesCollectionItemVaultAggregatedVaultItem>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FungibleResourcesCollectionItemVaultAggregatedVaultItem {
    pub vault_address: String,
    pub amount: Decimal,
    pub last_updated_at_state_version: u64,
}

// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct EntityMetadataCollection {
//     pub total_count: Option<u64>,
//     pub next_cursor: Option<String>,
// }

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LedgerState {
    pub network: String,
    pub state_version: u64,
    pub proposer_round_timestamp: String,
    pub epoch: u64,
    pub round: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PublicKeyType {
    EcdsaSecp256k1,
    EddsaEd25519,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PublicKey {
    pub key_type: PublicKeyType,
    // The hex-encoded compressed EdDSA Ed25519 public key (32 bytes)
    pub key_hex: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PreviewTransactionFlags {
    pub use_free_credit: bool,
    pub assume_all_signature_proofs: bool,
    pub skip_epoch_check: bool,
}

impl Default for PreviewTransactionFlags {
    fn default() -> Self {
        PreviewTransactionFlags {
            use_free_credit: false,
            assume_all_signature_proofs: false,
            skip_epoch_check: false,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct TransactionPreviewRequestBody {
    pub manifest: String,
    pub blobs_hex: Option<Vec<String>>,
    pub start_epoch_inclusive: i64,
    pub end_epoch_exclusive: i64,
    pub notary_public_key: Option<PublicKey>,
    pub notary_is_signatory: Option<bool>,
    pub tip_percentage: i32,
    pub nonce: String,
    pub signer_public_keys: Vec<PublicKey>,
    pub flags: PreviewTransactionFlags,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionPreview200ResponseBody {
    pub encoded_receipt: String,
    pub receipt: Receipt,
    pub resource_changes: Vec<InstructionResourceChanges>,
    pub logs: Vec<Log>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstructionResourceChanges {
    pub index: u64,
    pub resource_changes: Vec<ResourceChange>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResourceChange {
    pub resource_address: String,
    pub component_entity: EntityReference,
    pub vault_entity: EntityReference,
    pub amount: Decimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Log {
    pub level: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Receipt {
    pub status: Option<TransactionStatus>,
    pub fee_summary: Option<FeeSummary>,
    pub costing_parameters: Option<CostingParameters>,
    pub fee_source: Option<FeeSource>,
    pub fee_destination: Option<FeeDestination>,
    pub state_updates: Option<StateUpdates>,
    pub events: Option<Vec<Event>>,
    pub next_epoch: Option<NextEpoch>,
    pub output: Option<Vec<SborData>>,
    pub error_message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NextEpoch {
    pub epoch: u64,
    pub validators: Vec<ActiveValidator>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActiveValidator {
    pub address: String,
    pub key: EcdsaSecp256k1PublicKey,
    pub stake: Decimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EcdsaSecp256k1PublicKey {
    pub key_type: PublicKeyType,
    pub key_hex: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub name: String,
    pub emitter: EventEmitterIdentifier,
    pub data: Value,
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
    Method {
        entity: EntityReference,
        object_module_id: ModuleId,
    },
    Function {
        package_address: String,
        blueprint_name: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ModuleId {
    Main,
    Metadata,
    Royalty,
    RoleAssignment,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SborData {
    pub hex: Option<String>,
    pub programmatic_json: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LocalTypeIdKind {
    WellKnown,
    SchemaLocal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EventEmitterIdentifierType {
    Function,
    Method,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeeSummary {
    pub execution_cost_units_consumed: u64,
    pub finalization_cost_units_consumed: u64,
    pub xrd_total_execution_cost: Decimal,
    pub xrd_total_finalization_cost: Decimal,
    pub xrd_total_royalty_cost: Decimal,
    pub xrd_total_storage_cost: Decimal,
    pub xrd_total_tipping_cost: Decimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StateUpdates {
    pub deleted_partitions: Vec<serde_json::Value>, // Assuming it's a list of generic JSON values
    pub created_substates: Vec<serde_json::Value>, // Assuming it's a list of generic JSON values
    pub updated_substates: Vec<serde_json::Value>, // Assuming it's a list of generic JSON values
    pub deleted_substates: Vec<serde_json::Value>, // Assuming it's a list of generic JSON values
    pub new_global_entities: Vec<serde_json::Value>, // Assuming it's a list of generic JSON values
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionSubmitRequestBody {
    pub notarized_transaction_hex: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transactionsubmit200ResponseBody {
    pub duplicate: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetKeyValueStoreKeysRequestBody {
    pub at_ledger_state: Option<LedgerStateSelector>,
    pub cursor: Option<String>,
    pub limit_per_page: Option<u32>,
    pub key_value_store_address: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetKeyValueStoreKeys200ResponseBody {
    pub ledger_state: LedgerState,
    pub total_count: Option<u64>,
    pub next_cursor: Option<String>,
    pub items: Vec<StateKeyValueStoreKeysResponseItem>,
    pub key_value_store_address: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StateKeyValueStoreKeysResponseItem {
    pub key: serde_json::Value,
    pub last_updated_at_state_version: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetKeyValueStoreDataRequestBody {
    pub at_ledger_state: Option<LedgerStateSelector>,
    pub key_value_store_address: String,
    pub keys: Vec<StateKeyValueStoreDataRequestKeyItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StateKeyValueStoreDataRequestKeyItem {
    pub key_hex: Option<String>,
    pub key_json: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetKeyValueStoreData200ResponseBody {
    pub ledger_state: LedgerState,
    pub key_value_store_address: String,
    pub entries: Vec<StateKeyValueStoreDataResponseItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StateKeyValueStoreDataResponseItem {
    pub key: serde_json::Value,
    pub value: serde_json::Value,
    pub last_updated_at_state_version: u64,
    pub is_locked: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetGatewayStatus200Response {
    pub ledger_state: LedgerState,
    pub release_info: ReleaseInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReleaseInfo {
    pub release_version: String,
    pub open_api_schema_version: String,
    pub image_tag: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionStreamRequestBody {
    pub at_ledger_state: Option<LedgerStateSelector>,
    pub from_ledger_state: Option<LedgerStateSelector>,
    pub cursor: Option<String>,
    pub limit_per_page: Option<u32>,
    pub kind_filter: Option<TransactionKindFilter>,
    pub manifest_accounts_withdrawn_from_filter: Option<Vec<String>>,
    pub manifest_accounts_deposited_into_filter: Option<Vec<String>>,
    pub manifest_resources_filter: Option<Vec<String>>,
    pub affected_global_entities_filter: Option<Vec<String>>,
    // Not implemented now.
    // pub events_filter
    pub order: Option<Order>,
    pub opt_ins: Option<TransactionStreamOptIns>,
}

impl Default for TransactionStreamRequestBody {
    fn default() -> Self {
        TransactionStreamRequestBody {
            at_ledger_state: None,
            from_ledger_state: None,
            cursor: None,
            limit_per_page: None,
            kind_filter: None,
            manifest_accounts_withdrawn_from_filter: None,
            manifest_accounts_deposited_into_filter: None,
            manifest_resources_filter: None,
            affected_global_entities_filter: None,
            order: None,
            opt_ins: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TransactionKindFilter {
    User,
    EpochChange,
    All,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Order {
    Asc,
    Desc,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TransactionStreamOptIns {
    pub raw_hex: bool,
    pub receipt_state_changes: bool,
    pub receipt_fee_summary: bool,
    pub receipt_fee_source: bool,
    pub receipt_fee_destination: bool,
    pub receipt_costing_parameters: bool,
    pub receipt_events: bool,
    pub receipt_output: bool,
    pub affected_global_entities: bool,
    pub manifest_instructions: bool,
    pub balance_changes: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionStream200ResponseBody {
    pub ledger_state: LedgerState,
    pub total_count: Option<u64>,
    pub next_cursor: Option<String>,
    pub items: Vec<CommittedTransactionInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommittedTransactionInfo {
    pub state_version: u64,
    pub epoch: u64,
    pub round: u64,
    pub round_timestamp: String,
    pub transaction_status: TransactionStatus,
    pub payload_hash: Option<String>,
    pub intent_hash: Option<String>,
    pub fee_paid: Option<Decimal>,
    pub affected_global_entities: Option<Vec<EntityReference>>,
    pub confirmed_at: Option<chrono::DateTime<Utc>>,
    pub error_message: Option<String>,
    pub raw_hex: Option<String>,
    pub receipt: Option<Receipt>,
    pub manifest_instructions: Option<String>,
    // A collection of zero or more manifest
    // classes ordered from the most specific
    // class to the least specific one.
    // This field will be present only for user transactions.
    pub manifest_classes: Option<Vec<ManifestClass>>,
    pub message: Option<TransactionMessage>,
    // not implemented now
    // pub balance_changes: Option<Vec<TransactionBalanceChanges>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum TransactionMessage {
    Plaintext {
        mime_type: String,
        content: PlaintextMessageContent,
    },
    // didn't need this.
    // Encrypted(
    //     ecrypted_hex: String,

    // ),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum PlaintextMessageContent {
    String { value: String },
    Binary { value_hex: String },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TransactionStatus {
    Unknown,
    CommittedSuccess,
    CommittedFailure,
    Pending,
    Rejected,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ManifestClass {
    General,
    Transfer,
    PoolContribution,
    PoolRedemption,
    ValidatorStake,
    ValidatorUnstake,
    ValidatorClaim,
    AccountDepositSettingsUpdate,
}
