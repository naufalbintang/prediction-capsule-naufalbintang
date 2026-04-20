#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, String, Symbol, Vec,
};

// ===========================
// TIPE DATA
// ===========================

#[contracttype]
#[derive(Clone, Debug)]
pub struct Prediction {
    pub id: u64,
    pub owner: Address,
    pub content: String,
    pub category: Symbol,
    pub unlock_time: u64,
    pub created_at: u64,
    pub verdict: Symbol,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Reputation {
    pub total: u32,
    pub correct: u32,
    pub wrong: u32,
}

// ===========================
// STORAGE KEYS
// ===========================

const PREDICTIONS: Symbol = symbol_short!("PREDS");
const NEXT_ID: Symbol = symbol_short!("NEXT_ID");

// ===========================
// VERDICT CONSTANTS
// ===========================

fn verdict_pending(env: &Env) -> Symbol {
    Symbol::new(env, "PENDING")
}

fn verdict_correct(env: &Env) -> Symbol {
    Symbol::new(env, "CORRECT")
}

fn verdict_wrong(env: &Env) -> Symbol {
    Symbol::new(env, "WRONG")
}

fn verdict_locked(env: &Env) -> Symbol {
    Symbol::new(env, "LOCKED")
}

// ===========================
// CONTRACT
// ===========================

#[contract]
pub struct PredictionContract;

#[contractimpl]
impl PredictionContract {

    // Membuat prediksi baru
    pub fn submit_prediction(
        env: Env,
        owner: Address,
        content: String,
        category: Symbol,
        unlock_time: u64,
    ) -> String {
        // Verifikasi bahwa pemanggil adalah owner
        owner.require_auth();

        // Validasi unlock_time harus di masa depan
        let now = env.ledger().timestamp();
        if unlock_time <= now {
            return String::from_str(&env, "unlock_time harus di masa depan");
        }

        // Ambil ID berikutnya
        let id: u64 = env.storage().instance().get(&NEXT_ID).unwrap_or(0u64);

        // Buat prediksi baru
        let prediction = Prediction {
            id,
            owner,
            content,
            category,
            unlock_time,
            created_at: now,
            verdict: verdict_pending(&env),
        };

        // Simpan ke list prediksi
        let mut predictions: Vec<Prediction> = env
            .storage()
            .instance()
            .get(&PREDICTIONS)
            .unwrap_or(Vec::new(&env));

        predictions.push_back(prediction);
        env.storage().instance().set(&PREDICTIONS, &predictions);

        // Increment ID counter
        env.storage().instance().set(&NEXT_ID, &(id + 1));

        String::from_str(&env, "Prediksi berhasil disimpan")
    }

    // Ambil semua prediksi
    // Konten yang masih terkunci akan disembunyikan
    pub fn get_predictions(env: Env) -> Vec<Prediction> {
        let now = env.ledger().timestamp();

        let predictions: Vec<Prediction> = env
            .storage()
            .instance()
            .get(&PREDICTIONS)
            .unwrap_or(Vec::new(&env));

        let mut result = Vec::new(&env);

        for i in 0..predictions.len() {
            let mut p = predictions.get(i).unwrap();

            // Sembunyikan konten kalau belum waktunya
            if now < p.unlock_time {
                p.content = String::from_str(&env, "LOCKED");
                p.verdict = verdict_locked(&env);
            }

            result.push_back(p);
        }

        result
    }

    // Ambil satu prediksi berdasarkan ID
    pub fn get_prediction_by_id(env: Env, id: u64) -> Option<Prediction> {
        let now = env.ledger().timestamp();

        let predictions: Vec<Prediction> = env
            .storage()
            .instance()
            .get(&PREDICTIONS)
            .unwrap_or(Vec::new(&env));

        for i in 0..predictions.len() {
            let mut p = predictions.get(i).unwrap();

            if p.id == id {
                if now < p.unlock_time {
                    p.content = String::from_str(&env, "LOCKED");
                    p.verdict = verdict_locked(&env);
                }
                return Some(p);
            }
        }

        None
    }

    // Submit verdict setelah unlock_time tercapai
    // Hanya bisa dipanggil oleh owner prediksi
    pub fn submit_verdict(
        env: Env,
        caller: Address,
        id: u64,
        is_correct: bool,
    ) -> String {
        caller.require_auth();

        let now = env.ledger().timestamp();

        let mut predictions: Vec<Prediction> = env
            .storage()
            .instance()
            .get(&PREDICTIONS)
            .unwrap_or(Vec::new(&env));

        for i in 0..predictions.len() {
            let mut p = predictions.get(i).unwrap();

            if p.id == id {
                // Cek apakah pemanggil adalah owner
                if p.owner != caller {
                    return String::from_str(&env, "Bukan owner prediksi ini");
                }

                // Cek apakah sudah waktunya
                if now < p.unlock_time {
                    return String::from_str(&env, "Prediksi masih terkunci");
                }

                // Cek apakah verdict sudah diisi
                if p.verdict != verdict_pending(&env) {
                    return String::from_str(&env, "Verdict sudah pernah diisi");
                }

                // Set verdict
                p.verdict = if is_correct {
                    verdict_correct(&env)
                } else {
                    verdict_wrong(&env)
                };

                predictions.set(i, p);
                env.storage().instance().set(&PREDICTIONS, &predictions);

                return String::from_str(&env, "Verdict berhasil disimpan");
            }
        }

        String::from_str(&env, "Prediksi tidak ditemukan")
    }

    // Hitung reputasi berdasarkan address
    pub fn get_reputation(env: Env, owner: Address) -> Reputation {
        let predictions: Vec<Prediction> = env
            .storage()
            .instance()
            .get(&PREDICTIONS)
            .unwrap_or(Vec::new(&env));

        let mut total: u32 = 0;
        let mut correct: u32 = 0;
        let mut wrong: u32 = 0;

        for i in 0..predictions.len() {
            let p = predictions.get(i).unwrap();

            if p.owner == owner {
                total += 1;

                if p.verdict == verdict_correct(&env) {
                    correct += 1;
                } else if p.verdict == verdict_wrong(&env) {
                    wrong += 1;
                }
            }
        }

        Reputation { total, correct, wrong }
    }
}

mod test;