use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::{Base64VecU8};
use near_sdk::{env, near_bindgen, ext_contract, Gas};
//use serde::{Serialize, Deserialize};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Daobot {
    records: LookupMap<String, String>,
}


// #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
// pub struct ProposalOutput {
//     /// Id of the proposal.
//     pub id: u64,
//     #[serde(flatten)]
//     pub proposal: Proposal,
// }


// Trigger macro to create interfact to external contract.
#[ext_contract(ext_astrodao)]
pub trait Astrodao {
     fn version(&self) -> String;
     fn get_proposals(&self, from_index: u64, limit: u64) -> Vec<(u64, String, String, Base64VecU8, String, Base64VecU8, Base64VecU8, String)>;
    fn get_proposal(&self, id: u64) -> (u64, String, String, Base64VecU8, String, Base64VecU8, Base64VecU8, String);
     fn act_proposal(&self, id: u64, action: String);
}



// Recieve callbacks from external contract.
#[ext_contract(ext_self)]
trait Callbacks {
    fn on_get_proposals(&self,#[callback] proposals: Vec<(u64, String, String, Base64VecU8, String, Base64VecU8, Base64VecU8, String)>);
    fn on_get_proposal(&self,#[callback] proposal: (u64, String, String, Base64VecU8, String, Base64VecU8, Base64VecU8, String));
}   

impl Default for Daobot {
    fn default() -> Self {
        Self {
            records: LookupMap::new(b"r".to_vec()),
        }
    }
}

// Gas needed for common operations.
pub const GAS_FOR_COMMON_OPERATIONS: Gas = 30_000_000_000_000;

// Gas reserved for the current call.
pub const GAS_RESERVED_FOR_CURRENT_CALL: Gas = 20_000_000_000_000;

pub const GAS_ESTIMATE: Gas = 10_000_000_000_000;
#[near_bindgen]
impl Daobot {

    pub fn set_status(&mut self, message: String) {
        let account_id = env::signer_account_id();
        self.records.insert(&account_id, &message);
    }

    pub fn get_status(&self, account_id: String) -> Option<String> {
        return self.records.get(&account_id);
    }

    pub fn something(&self, arg1: String) -> String {
        return "Something".to_string() + &arg1;
    }

    pub fn approve_members(&self, dao_id: String){
        ext_astrodao::get_proposals(0, 100, &dao_id, 0, GAS_FOR_COMMON_OPERATIONS ).then(
            ext_self::on_get_proposals(&env::current_account_id(), 0, GAS_FOR_COMMON_OPERATIONS));    
        }

    #[private]
    pub fn on_get_proposals(&self, #[callback] proposals: Vec<(u64, String, String, Base64VecU8, String, Base64VecU8, Base64VecU8, String)>)  {

        proposals.iter().for_each(|(id, _action, _proposer, _proposal, _proposal_hash, _proposal_hash_signature, _proposal_signature, _proposal_signature_signature)| {
            ext_astrodao::act_proposal(*id, "VoteApprove".to_string(), &env::current_account_id(), 0, GAS_FOR_COMMON_OPERATIONS);
        });
    }

    #[private]
    pub fn on_get_proposal(&self, #[callback] proposal: (u64, String, String, Base64VecU8, String, Base64VecU8, Base64VecU8, String)) -> u64 {
        let id = proposal.0;


        let available_gas = env::prepaid_gas();
        let remaining_gas: Gas = env::prepaid_gas()
        - env::used_gas()
        - GAS_FOR_COMMON_OPERATIONS
        - GAS_RESERVED_FOR_CURRENT_CALL;
        ext_astrodao::act_proposal(id, "VoteApprove".to_string(),&env::current_account_id(), 0, available_gas ).then(
            ext_self::on_get_proposal(&env::current_account_id(), 0, remaining_gas));   
        return id;
    }
}


#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    #[test]
    fn set_get_message() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Daobot::default();
        contract.set_status("hello".to_string());
        assert_eq!(
            "hello".to_string(),
            contract.get_status("bob_near".to_string()).unwrap()
        );
    }

    #[test]
    fn get_nonexistent_message() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = Daobot::default();
        assert_eq!(None, contract.get_status("francis.near".to_string()));
    }
}
