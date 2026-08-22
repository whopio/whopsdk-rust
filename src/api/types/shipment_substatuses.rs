pub use crate::prelude::*;

/// The substatus of a shipment
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShipmentSubstatuses {
    AddressCorrection,
    ArrivedAtDestination,
    ArrivedAtFacility,
    ArrivedAtPickupLocation,
    AwaitingInformation,
    SubstatusCancelled,
    Damaged,
    Delayed,
    DeliveryException,
    DepartedFacility,
    DepartedOriginFacility,
    Expired,
    SubstatusFailure,
    Held,
    SubstatusInTransit,
    LabelCreated,
    Lost,
    Missorted,
    SubstatusOutForDelivery,
    ReceivedAtDestinationFacility,
    ReceivedAtOriginFacility,
    Refused,
    Return,
    StatusUpdate,
    TransferredToDestinationCarrier,
    TransitException,
    SubstatusUnknown,
    WeatherDelay,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ShipmentSubstatuses {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AddressCorrection => serializer.serialize_str("address_correction"),
            Self::ArrivedAtDestination => serializer.serialize_str("arrived_at_destination"),
            Self::ArrivedAtFacility => serializer.serialize_str("arrived_at_facility"),
            Self::ArrivedAtPickupLocation => serializer.serialize_str("arrived_at_pickup_location"),
            Self::AwaitingInformation => serializer.serialize_str("awaiting_information"),
            Self::SubstatusCancelled => serializer.serialize_str("substatus_cancelled"),
            Self::Damaged => serializer.serialize_str("damaged"),
            Self::Delayed => serializer.serialize_str("delayed"),
            Self::DeliveryException => serializer.serialize_str("delivery_exception"),
            Self::DepartedFacility => serializer.serialize_str("departed_facility"),
            Self::DepartedOriginFacility => serializer.serialize_str("departed_origin_facility"),
            Self::Expired => serializer.serialize_str("expired"),
            Self::SubstatusFailure => serializer.serialize_str("substatus_failure"),
            Self::Held => serializer.serialize_str("held"),
            Self::SubstatusInTransit => serializer.serialize_str("substatus_in_transit"),
            Self::LabelCreated => serializer.serialize_str("label_created"),
            Self::Lost => serializer.serialize_str("lost"),
            Self::Missorted => serializer.serialize_str("missorted"),
            Self::SubstatusOutForDelivery => serializer.serialize_str("substatus_out_for_delivery"),
            Self::ReceivedAtDestinationFacility => {
                serializer.serialize_str("received_at_destination_facility")
            }
            Self::ReceivedAtOriginFacility => {
                serializer.serialize_str("received_at_origin_facility")
            }
            Self::Refused => serializer.serialize_str("refused"),
            Self::Return => serializer.serialize_str("return"),
            Self::StatusUpdate => serializer.serialize_str("status_update"),
            Self::TransferredToDestinationCarrier => {
                serializer.serialize_str("transferred_to_destination_carrier")
            }
            Self::TransitException => serializer.serialize_str("transit_exception"),
            Self::SubstatusUnknown => serializer.serialize_str("substatus_unknown"),
            Self::WeatherDelay => serializer.serialize_str("weather_delay"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ShipmentSubstatuses {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "address_correction" => Ok(Self::AddressCorrection),
            "arrived_at_destination" => Ok(Self::ArrivedAtDestination),
            "arrived_at_facility" => Ok(Self::ArrivedAtFacility),
            "arrived_at_pickup_location" => Ok(Self::ArrivedAtPickupLocation),
            "awaiting_information" => Ok(Self::AwaitingInformation),
            "substatus_cancelled" => Ok(Self::SubstatusCancelled),
            "damaged" => Ok(Self::Damaged),
            "delayed" => Ok(Self::Delayed),
            "delivery_exception" => Ok(Self::DeliveryException),
            "departed_facility" => Ok(Self::DepartedFacility),
            "departed_origin_facility" => Ok(Self::DepartedOriginFacility),
            "expired" => Ok(Self::Expired),
            "substatus_failure" => Ok(Self::SubstatusFailure),
            "held" => Ok(Self::Held),
            "substatus_in_transit" => Ok(Self::SubstatusInTransit),
            "label_created" => Ok(Self::LabelCreated),
            "lost" => Ok(Self::Lost),
            "missorted" => Ok(Self::Missorted),
            "substatus_out_for_delivery" => Ok(Self::SubstatusOutForDelivery),
            "received_at_destination_facility" => Ok(Self::ReceivedAtDestinationFacility),
            "received_at_origin_facility" => Ok(Self::ReceivedAtOriginFacility),
            "refused" => Ok(Self::Refused),
            "return" => Ok(Self::Return),
            "status_update" => Ok(Self::StatusUpdate),
            "transferred_to_destination_carrier" => Ok(Self::TransferredToDestinationCarrier),
            "transit_exception" => Ok(Self::TransitException),
            "substatus_unknown" => Ok(Self::SubstatusUnknown),
            "weather_delay" => Ok(Self::WeatherDelay),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ShipmentSubstatuses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AddressCorrection => write!(f, "address_correction"),
            Self::ArrivedAtDestination => write!(f, "arrived_at_destination"),
            Self::ArrivedAtFacility => write!(f, "arrived_at_facility"),
            Self::ArrivedAtPickupLocation => write!(f, "arrived_at_pickup_location"),
            Self::AwaitingInformation => write!(f, "awaiting_information"),
            Self::SubstatusCancelled => write!(f, "substatus_cancelled"),
            Self::Damaged => write!(f, "damaged"),
            Self::Delayed => write!(f, "delayed"),
            Self::DeliveryException => write!(f, "delivery_exception"),
            Self::DepartedFacility => write!(f, "departed_facility"),
            Self::DepartedOriginFacility => write!(f, "departed_origin_facility"),
            Self::Expired => write!(f, "expired"),
            Self::SubstatusFailure => write!(f, "substatus_failure"),
            Self::Held => write!(f, "held"),
            Self::SubstatusInTransit => write!(f, "substatus_in_transit"),
            Self::LabelCreated => write!(f, "label_created"),
            Self::Lost => write!(f, "lost"),
            Self::Missorted => write!(f, "missorted"),
            Self::SubstatusOutForDelivery => write!(f, "substatus_out_for_delivery"),
            Self::ReceivedAtDestinationFacility => write!(f, "received_at_destination_facility"),
            Self::ReceivedAtOriginFacility => write!(f, "received_at_origin_facility"),
            Self::Refused => write!(f, "refused"),
            Self::Return => write!(f, "return"),
            Self::StatusUpdate => write!(f, "status_update"),
            Self::TransferredToDestinationCarrier => {
                write!(f, "transferred_to_destination_carrier")
            }
            Self::TransitException => write!(f, "transit_exception"),
            Self::SubstatusUnknown => write!(f, "substatus_unknown"),
            Self::WeatherDelay => write!(f, "weather_delay"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
