use std::cmp::max;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, env, near_bindgen, ext_contract, log};
use serde::Deserialize;

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Daobot {
}

#[derive(Deserialize)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum ProposalKinds {
    ChangeConfig {  },
    ChangePolicy {  },
    ChangePolicyAddOrUpdateRole {  },
    ChangePolicyRemoveRole {  },
    ChangePolicyUpdateDefaultVotePolicy {  },
    ChangePolicyUpdateParameters {  },
    AddMemberToRole {  },
    RemoveMemberFromRole {  },
    FunctionCall {  },
    UpgradeSelf {  },
    UpgradeRemote {  },
    Transfer {  },
    SetStakingContract {  },
    AddBounty {  },
    BountyDone {  },
    Vote,
    FactoryInfoUpdate {  },
}

#[derive(Deserialize)]
#[derive(Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Proposal {
    id: u64,
    status: String,
    kind: ProposalKinds,
}

#[ext_contract(ext_astrodao)]
pub trait Astrodao {
     fn version(&self) -> String;
     fn get_proposals(&self, from_index: u64, limit: u64) -> Vec<Proposal>;
     fn act_proposal(&self, id: u64, action: String);
     fn get_last_proposal_id(&self) -> u64;
}



// Recieve callbacks from external contract.
#[ext_contract(ext_self)]
trait Callbacks {
    fn on_get_proposals(&self, dao_id: AccountId, #[callback] proposals: Vec<Proposal>);
    fn on_get_last_proposal_id(&self, dao_id: AccountId, #[callback] last_proposal_id: u64);
    fn proposal_approved(&self, id: u64);
}   

impl Default for Daobot {
    fn default() -> Self {
        Self {
        }
    }
}

#[near_bindgen]
impl Daobot {

    pub fn approve_members(&self, dao_id: AccountId) {
        let total_gas = env::prepaid_gas();
        let num_calls = 6;
        let gas_per_call = total_gas / num_calls;

        let callback = ext_self::on_get_last_proposal_id(dao_id.clone(), &env::current_account_id(), 0, gas_per_call);
        ext_astrodao::get_last_proposal_id(&dao_id, 0, gas_per_call).then(callback);
}

    #[private]
    pub fn on_get_last_proposal_id(&self, dao_id: AccountId, #[callback] last_proposal_id: u64) {
        let total_gas = env::prepaid_gas();
        let num_calls = 6;
        let gas_per_call = total_gas / num_calls;
        let callback = ext_self::on_get_proposals(dao_id.clone(),&env::current_account_id(), 0, gas_per_call);
        log!("on_get_last_proposal_id: {:?}", last_proposal_id);
        
        ext_astrodao::get_proposals(max(100,last_proposal_id)-100, 100, &dao_id, 0, gas_per_call*2 )
        .then(callback);
    }

    #[private]
    pub fn on_get_proposals(&self, dao_id: &near_sdk::AccountId, #[callback] proposals: Vec<Proposal>)  {
        let mut active_proposals = proposals.iter().filter(|p| p.status == "InProgress".to_string() && p.kind == ProposalKinds::AddMemberToRole{} ).peekable();
        if active_proposals.peek().is_none() {
            panic!("No active proposals");
        }
        log!("Used gas in callback: {:?} out of {:?}", env::used_gas(), env::prepaid_gas());
        let proposal_ids = active_proposals.map(|p| p.id).collect::<Vec<u64>>();
        let proposal_id_count = (proposal_ids.len()) as u64 * 4;
        proposal_ids.iter().for_each(|id| {  
            let approved = ext_self::proposal_approved(*id, &env::current_account_id(),0, env::prepaid_gas()/proposal_id_count);
            ext_astrodao::act_proposal(*id, "VoteApprove".to_string(), &dao_id, 0, env::prepaid_gas()/proposal_id_count).then(approved);

        });
      
    }

    #[private]
    pub fn proposal_approved(&self, id: u64) {
        log!("Proposal {} approved", id);
    }
}
