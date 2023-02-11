use masking::PeekInterface;
use serde::{Deserialize, Serialize};
use crate::{core::errors,types::{self,api, storage::enums}};

#[derive(Debug, Serialize)]
pub struct CardHolderInfo {
    pub first_name: String,
    pub last_name: String,
    pub zip: String,
}

#[derive(Debug, Serialize)]
pub struct CreditCard {
    pub expiration_year: String,
    pub security_code: String,
    pub expiration_month: String,
    pub card_number: String,
}


//TODO: Fill the struct with respective fields
#[derive(Debug, Serialize)]
pub struct BluesnapPaymentsRequest {
    pub soft_descriptor:String,
    pub amount:i64,
    pub currency:String,
    pub card_holder_info: CardHolderInfo,
    pub credit_card:CreditCard,
    pub card_transaction_type:String,
}

impl TryFrom<&types::PaymentsAuthorizeRouterData> for BluesnapPaymentsRequest  {
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(item: &types::PaymentsAuthorizeRouterData) -> Result<Self,Self::Error> {
        let amount = item.request.amount;
        let currency = item.request.currency.to_string();
        let soft_descriptor = match &item.description {
            Some(a)=>a.clone(),
            None => "".to_string()
        };
        let card_holder_info_var = match &item.address.billing {
                Some(add)  => { match &add.address {
                    Some(add_det) => CardHolderInfo{
                        first_name: match &add_det.first_name {
                            Some(a)=>a.peek().clone(),
                            None => "".to_string()
                        },
                        
                        last_name: match &add_det.last_name {
                            Some(a)=>a.peek().clone(),
                            None => "".to_string()
                        },
                        zip: match &add_det.zip {
                            Some(a)=>a.peek().clone(),
                            None => "".to_string()
                        },
                    },
                    None => CardHolderInfo{
                        first_name:"".to_owned(),
                        last_name:"".to_owned(),
                        zip:"".to_owned()
                    },
                }},
                None => CardHolderInfo{
                        first_name:"".to_owned(),
                        last_name:"".to_owned(),
                        zip:"".to_owned()
                    },
        };
        let credit_card_val = match item.request.payment_method_data {
        api::PaymentMethod::Card(ref ccard) => Ok(
            CreditCard {
                expiration_year: ccard.card_exp_year.peek().clone(),
                security_code: ccard.card_cvc.peek().clone(),
                expiration_month:  ccard.card_exp_month.peek().clone(),
                card_number: ccard.card_number.peek().clone(),
            }
        ),
        _ => Err(errors::ConnectorError::NotImplemented(format!(
                "Current Payment Method - {:?}",
                item.request.payment_method_data
            ))),
    }?;
    Ok(Self {
        soft_descriptor,
        amount,
        currency,
        card_holder_info: card_holder_info_var,
        credit_card: credit_card_val,
        card_transaction_type:"AUTH_CAPTURE".to_owned(),
    })
    }
}


//TODO: Fill the struct with respective fields
// Auth Struct
pub struct BluesnapAuthType {
    pub(super) api_key: String
}

impl TryFrom<&types::ConnectorAuthType> for BluesnapAuthType  {
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(auth_type: &types::ConnectorAuthType) -> Result<Self, Self::Error> {
       match auth_type {
            types::ConnectorAuthType::BodyKey { api_key, key1:_ } => Ok(Self {
                    api_key: api_key.to_string()}),
            _ => Err(errors::ConnectorError::FailedToObtainAuthType)?,
       }
    }
}
// PaymentsResponse
//TODO: Append the remaining status flags
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BluesnapPaymentStatus {
    Succeeded,
    Failed,
    #[default]
    Processing,
}

impl From<BluesnapPaymentStatus> for enums::AttemptStatus {
    fn from(item: BluesnapPaymentStatus) -> Self {
        match item {
            BluesnapPaymentStatus::Succeeded => Self::Charged,
            BluesnapPaymentStatus::Failed => Self::Failure,
            BluesnapPaymentStatus::Processing => Self::Authorizing,
        }
    }
}

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BluesnapPaymentsResponse {
    status: BluesnapPaymentStatus,
    amount: i64,
    id: String,
}

impl<F,T> TryFrom<types::ResponseRouterData<F, BluesnapPaymentsResponse, T, types::PaymentsResponseData>> for types::RouterData<F, T, types::PaymentsResponseData> {
    type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(item: types::ResponseRouterData<F, BluesnapPaymentsResponse, T, types::PaymentsResponseData>) -> Result<Self,Self::Error> {
        Ok(Self {
            status: enums::AttemptStatus::from(item.response.status),
            amount_captured: Some(item.response.amount),
            response: Ok(types::PaymentsResponseData::TransactionResponse {
                resource_id: types::ResponseId::ConnectorTransactionId(item.response.id),
                redirection_data: None,
                redirect: false,
                mandate_reference: None,
                connector_metadata: None,
            }),
            ..item.data
        })
    }
}

//TODO: Fill the struct with respective fields
// REFUND :
// Type definition for RefundRequest
#[derive(Default, Debug, Serialize)]
pub struct BluesnapRefundRequest {}

impl<F> TryFrom<&types::RefundsRouterData<F>> for BluesnapRefundRequest {
    type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(_item: &types::RefundsRouterData<F>) -> Result<Self,Self::Error> {
       todo!()
    }
}

// Type definition for Refund Response

#[allow(dead_code)]
#[derive(Debug, Serialize, Default, Deserialize, Clone)]
pub enum RefundStatus {
    Succeeded,
    Failed,
    #[default]
    Processing,
}

impl From<RefundStatus> for enums::RefundStatus {
    fn from(item: RefundStatus) -> Self {
        match item {
            RefundStatus::Succeeded => Self::Success,
            RefundStatus::Failed => Self::Failure,
            RefundStatus::Processing => Self::Pending,
            //TODO: Review mapping
        }
    }
}

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RefundResponse {
}

impl TryFrom<types::RefundsResponseRouterData<api::Execute, RefundResponse>>
    for types::RefundsRouterData<api::Execute>
{
    type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(
        _item: types::RefundsResponseRouterData<api::Execute, RefundResponse>,
    ) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<types::RefundsResponseRouterData<api::RSync, RefundResponse>> for types::RefundsRouterData<api::RSync>
{
     type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(_item: types::RefundsResponseRouterData<api::RSync, RefundResponse>) -> Result<Self,Self::Error> {
         todo!()
     }
 }

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct BluesnapErrorResponse {}
