//!Liabilities types

use serde::{Deserialize, Serialize};

use crate::types::*;

pub struct Aprs
{
    pub apr_percentage: u32,
    pub apr_type: String,
    pub balance_subject_to_apr: Option<u32>,
    pub interest_charge_amount: Option<u32>    
}

pub struct Credit 
{
    pub account_id: Option<String>,
    pub aprs: Aprs,
    pub is_overdue: Option<bool>,
    pub last_payment_amount: u32,
    pub last_payment_date: String,
    pub last_statement_balance: u32,
    pub last_statement_issue_date: String,
    pub minimum_payment_amount: u32,
    pub next_payment_due_date: String
}

#[derive(Serialize, Deserialize)] 
pub struct InterestRate
{
    pub percentage: Option<u32>,
    #[serde(rename = "type")]
    pub interest_rate_type: Option<String>
}

pub struct PropertyAddress
{
    pub city: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub region: Option<String>,
    pub street: Option<String>,
}

pub struct Mortgage
{
    pub account_id: Option<String>,
    pub account_number: String,
    pub current_late_fee: Option<u32>,
    pub escrow_balance: Option<u32>,
    pub has_pml: Option<bool>,
    pub has_prepayment_penalty: Option<bool>,
    pub interest_rate: InterestRate,
    pub last_payment_amount: Option<u32>,
    pub last_payment_date: Option<String>,
    pub loan_type_description: Option<String>,
    pub loan_term: Option<String>,
    pub maturity_date: Option<String>,
    pub next_monthly_payment: Option<u32>,
    pub next_payment_due_date: Option<String>,
    pub origination_date: Option<String>,
    pub origination_principal_amount: Option<u32>,
    pub past_due_amount: Option<u32>,
    pub property_address: PropertyAddress,
    pub ytd_interest_paid: Option<u32>,
    pub ytd_principal_paid: Option<u32>
}

#[derive(Serialize, Deserialize)] 
pub struct LoanStatus
{
    pub end_date: Option<String>,
    #[serde(rename = "type")]
    pub loan_status_type: Option<String>
}

pub struct PslfStatus
{
    pub estimated_eligibility_date: Option<String>,
    pub payments_made: Option<u32>,
    pub payments_remaining: Option<u32>
}

#[derive(Serialize, Deserialize)] 
pub struct RepaymentPlan
{
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub repaymentplan_type: Option<String>
}

pub struct ServicerAddress
{
    pub city: Option<String>,
    pub region: Option<String>,
    pub street: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>
}

pub struct Student
{
    pub account_id: Option<String>,
    pub account_number: Option<String>,
    pub disbursement_dates: Option<String>,
    pub expected_payoff_date: Option<String>,
    pub guarantor: Option<String>,
    pub interest_rate_percentage: u32,
    pub is_overdue: Option<bool>,
    pub last_payment_amount: Option<u32>,
    pub last_payment_date: Option<String>,
    pub last_statement_balance: Option<u32>,
    pub last_statement_issue_date: Option<String>,
    pub loan_name: Option<String>,
    pub loan_status: LoanStatus,
    pub minimum_payment_amount: Option<u32>,
    pub next_payment_due_date: Option<String>,
    pub origination_date: Option<String>,
    pub origination_principal_amount: Option<u32>,
    pub outstanding_interest_amount: Option<u32>,
    pub payment_reference_number: Option<String>,
    pub pslf_status: PslfStatus,
    pub repayment_plan: RepaymentPlan,
    pub sequence_number: Option<String>,
    pub servicer_address: ServicerAddress,
    pub ytd_interest_paid: Option<u32>,
    pub ytd_principal_paid: Option<u32>
}

pub struct Liabilities
{
    pub credit: Credit,
    pub mortgage: Mortgage,
    pub student: Student,
    pub request_id: String
}

/// The response from performing an `Liabilities` request.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LiabilitiesResponse
{
    /// The financial institution accounts associated with the Item.
    #[serde(default)]
    pub accounts: Vec<Account>,

    /// Metadata about the Item.
    pub item: super::Item,

    /// A unique identifier for the request, which can be used for
    /// troubleshooting. This identifier, like all Plaid identifiers, is case
    /// sensitive.
    pub request_id: String
}
