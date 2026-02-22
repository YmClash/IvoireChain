//! User entity representing a platform user.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


use crate::domain::value_objects::Money;


/// User status in the platform
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_status", rename_all = "snake_case")]
pub enum UserStatus {
    Active,
    Suspended,
    PendingVerification,
}

impl std::fmt::Display for UserStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserStatus::Active => write!(f, "active"),
            UserStatus::Suspended => write!(f, "suspended"),
            UserStatus::PendingVerification => write!(f, "pending_verification"),
        }
    }
}

/// User entity representing a registred user on the platform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User{
    pub id : Uuid,
    pub phone_number: String,
    #[serde(skip_serializing)]
    pub hashed_password: String,
    pub balance: Money,
    pub status: UserStatus,
    pub created_at : DateTime<Utc>,
    pub updated_at : DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub user_id: Uuid,
    pub currency: String,
    pub balance: Money,
    pub last_updated: DateTime<Utc>,
}


impl User {

    pub fn new(phone_number: String, hashed_password: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            phone_number,
            hashed_password,
            balance: Money::zero(),
            status: UserStatus::PendingVerification,
            created_at: now,
            updated_at: now,
        }
    }

    /// Check if user can afford a purchase
    pub fn can_afford(&self,amount:&Money) -> bool {
        self.balance.centimes() >= amount.centimes()
    }

    pub fn debit(&mut self, amount: &Money) -> Result<(), String> {
        if !self.can_afford(amount) {
            return Err(format!(
                "Solde insuffisant: requis {} FCFA, disponible {} FCFA",
                amount.to_fcfa(),
                self.balance.to_fcfa()
            ));
        }
        self.balance = Money::from_centimes(self.balance.centimes() - amount.centimes());
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Credit user balance
    pub fn credit(&mut self, amount:&Money){
        self.balance = Money::from_centimes(self.balance.centimes() + amount.centimes());
        self.updated_at = Utc::now();
    }
    pub fn activate(&mut self){
        self.status = UserStatus::Active;
        self.updated_at = Utc::now();
    }

    /// Suspend user account
    pub fn suspend(&mut self){
        self.status = UserStatus::Suspended;
        self.updated_at = Utc::now();
    }

    /// Check if user is active
    pub fn is_active(&self) ->bool {
        self.status == UserStatus::Active
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_user() {
        let user = User::new("+2250701234567".to_string(), "hashed".to_string());
        assert_eq!(user.balance.centimes(), 0);
        assert_eq!(user.status, UserStatus::PendingVerification);
    }

    //noinspection RsDetachedFile
    #[test]
    fn test_debit_credit() {
        let mut user = User::new("+2250701234567".to_string(), "hashed".to_string());

        // Credit 1000 FCFA
        user.credit(&Money::from_fcfa(1000.0));
        assert_eq!(user.balance.to_fcfa(), 1000.0);

        // Debit 500 FCFA
        assert!(user.debit(&Money::from_fcfa(500.0)).is_ok());
        assert_eq!(user.balance.to_fcfa(), 500.0);

        // Try to debit more than available
        assert!(user.debit(&Money::from_fcfa(600.0)).is_err());
    }
}
