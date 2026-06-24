#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug)]
pub struct AcademicCredential {
    pub id: u64,
    pub candidate: Address,
    pub institution: Address,
    pub hash_proof: String,
    pub is_valid: bool,
}

const CRED_KEY: Symbol = symbol_short!("AC_CREDS");

#[contract]
pub struct CertiChainContract;

#[contractimpl]
impl CertiChainContract {
    pub fn get_credentials(env: Env) -> Vec<AcademicCredential> {
        env.storage().instance().get(&CRED_KEY).unwrap_or(Vec::new(&env))
    }

    pub fn issue_credential(env: Env, institution: Address, candidate: Address, hash_proof: String) -> u64 {
        institution.require_auth();

        let mut credentials: Vec<AcademicCredential> = env.storage().instance().get(&CRED_KEY).unwrap_or(Vec::new(&env));
        let id = env.prng().gen::<u64>();
        let cred = AcademicCredential {
            id,
            candidate,
            institution,
            hash_proof,
            is_valid: true,
        };

        credentials.push_back(cred);
        env.storage().instance().set(&CRED_KEY, &credentials);

        id
    }

    pub fn verify_credential(env: Env, hash_proof: String) -> bool {
        let credentials: Vec<AcademicCredential> = env.storage().instance().get(&CRED_KEY).unwrap_or(Vec::new(&env));
        for i in 0..credentials.len() {
            let cred = credentials.get(i).unwrap();
            if cred.hash_proof == hash_proof && cred.is_valid {
                return true;
            }
        }
        false
    }

    pub fn revoke_credential(env: Env, institution: Address, id: u64) -> String {
        institution.require_auth();

        let mut credentials: Vec<AcademicCredential> = env.storage().instance().get(&CRED_KEY).unwrap_or(Vec::new(&env));
        let mut found = false;

        for i in 0..credentials.len() {
            let mut cred = credentials.get(i).unwrap();
            if cred.id == id && cred.institution == institution && cred.is_valid {
                cred.is_valid = false;
                credentials.set(i, cred);
                found = true;
                break;
            }
        }

        if found {
            env.storage().instance().set(&CRED_KEY, &credentials);
            String::from_str(&env, "Credential successfully revoked")
        } else {
            String::from_str(&env, "Credential not found or unauthorized")
        }
    }
}

mod test;
