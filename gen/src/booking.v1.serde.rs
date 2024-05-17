// @generated
impl serde::Serialize for Booking {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.booking_holder_key.is_empty() {
            len += 1;
        }
        if !self.slot_id.is_empty() {
            len += 1;
        }
        if self.slot.is_some() {
            len += 1;
        }
        if self.date_time.is_some() {
            len += 1;
        }
        if self.persons != 0 {
            len += 1;
        }
        if self.created_at != 0 {
            len += 1;
        }
        if self.updated_at != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("booking.v1.Booking", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.booking_holder_key.is_empty() {
            struct_ser.serialize_field("bookingHolderKey", &self.booking_holder_key)?;
        }
        if !self.slot_id.is_empty() {
            struct_ser.serialize_field("slotId", &self.slot_id)?;
        }
        if let Some(v) = self.slot.as_ref() {
            struct_ser.serialize_field("slot", v)?;
        }
        if let Some(v) = self.date_time.as_ref() {
            struct_ser.serialize_field("dateTime", v)?;
        }
        if self.persons != 0 {
            struct_ser.serialize_field("persons", &self.persons)?;
        }
        if self.created_at != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("createdAt", ToString::to_string(&self.created_at).as_str())?;
        }
        if self.updated_at != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("updatedAt", ToString::to_string(&self.updated_at).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Booking {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "booking_holder_key",
            "bookingHolderKey",
            "slot_id",
            "slotId",
            "slot",
            "date_time",
            "dateTime",
            "persons",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            BookingHolderKey,
            SlotId,
            Slot,
            DateTime,
            Persons,
            CreatedAt,
            UpdatedAt,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "bookingHolderKey" | "booking_holder_key" => Ok(GeneratedField::BookingHolderKey),
                            "slotId" | "slot_id" => Ok(GeneratedField::SlotId),
                            "slot" => Ok(GeneratedField::Slot),
                            "dateTime" | "date_time" => Ok(GeneratedField::DateTime),
                            "persons" => Ok(GeneratedField::Persons),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Booking;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct booking.v1.Booking")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Booking, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut booking_holder_key__ = None;
                let mut slot_id__ = None;
                let mut slot__ = None;
                let mut date_time__ = None;
                let mut persons__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BookingHolderKey => {
                            if booking_holder_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bookingHolderKey"));
                            }
                            booking_holder_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SlotId => {
                            if slot_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slotId"));
                            }
                            slot_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Slot => {
                            if slot__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slot"));
                            }
                            slot__ = map_.next_value()?;
                        }
                        GeneratedField::DateTime => {
                            if date_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dateTime"));
                            }
                            date_time__ = map_.next_value()?;
                        }
                        GeneratedField::Persons => {
                            if persons__.is_some() {
                                return Err(serde::de::Error::duplicate_field("persons"));
                            }
                            persons__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Booking {
                    id: id__.unwrap_or_default(),
                    booking_holder_key: booking_holder_key__.unwrap_or_default(),
                    slot_id: slot_id__.unwrap_or_default(),
                    slot: slot__,
                    date_time: date_time__,
                    persons: persons__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("booking.v1.Booking", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Cancellation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.canceled_by.is_empty() {
            len += 1;
        }
        if !self.reason.is_empty() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("booking.v1.Cancellation", len)?;
        if !self.canceled_by.is_empty() {
            struct_ser.serialize_field("canceledBy", &self.canceled_by)?;
        }
        if !self.reason.is_empty() {
            struct_ser.serialize_field("reason", &self.reason)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Cancellation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "canceled_by",
            "canceledBy",
            "reason",
            "created_at",
            "createdAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CanceledBy,
            Reason,
            CreatedAt,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "canceledBy" | "canceled_by" => Ok(GeneratedField::CanceledBy),
                            "reason" => Ok(GeneratedField::Reason),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Cancellation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct booking.v1.Cancellation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Cancellation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut canceled_by__ = None;
                let mut reason__ = None;
                let mut created_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CanceledBy => {
                            if canceled_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("canceledBy"));
                            }
                            canceled_by__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Cancellation {
                    canceled_by: canceled_by__.unwrap_or_default(),
                    reason: reason__.unwrap_or_default(),
                    created_at: created_at__,
                })
            }
        }
        deserializer.deserialize_struct("booking.v1.Cancellation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Closure {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.closing_from.is_some() {
            len += 1;
        }
        if self.closing_to.is_some() {
            len += 1;
        }
        if !self.organizer_key.is_empty() {
            len += 1;
        }
        if self.created_at != 0 {
            len += 1;
        }
        if self.updated_at != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("booking.v1.Closure", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.closing_from.as_ref() {
            struct_ser.serialize_field("closingFrom", v)?;
        }
        if let Some(v) = self.closing_to.as_ref() {
            struct_ser.serialize_field("closingTo", v)?;
        }
        if !self.organizer_key.is_empty() {
            struct_ser.serialize_field("organizerKey", &self.organizer_key)?;
        }
        if self.created_at != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("createdAt", ToString::to_string(&self.created_at).as_str())?;
        }
        if self.updated_at != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("updatedAt", ToString::to_string(&self.updated_at).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Closure {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "closing_from",
            "closingFrom",
            "closing_to",
            "closingTo",
            "organizer_key",
            "organizerKey",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            ClosingFrom,
            ClosingTo,
            OrganizerKey,
            CreatedAt,
            UpdatedAt,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "closingFrom" | "closing_from" => Ok(GeneratedField::ClosingFrom),
                            "closingTo" | "closing_to" => Ok(GeneratedField::ClosingTo),
                            "organizerKey" | "organizer_key" => Ok(GeneratedField::OrganizerKey),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Closure;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct booking.v1.Closure")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Closure, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut closing_from__ = None;
                let mut closing_to__ = None;
                let mut organizer_key__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClosingFrom => {
                            if closing_from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("closingFrom"));
                            }
                            closing_from__ = map_.next_value()?;
                        }
                        GeneratedField::ClosingTo => {
                            if closing_to__.is_some() {
                                return Err(serde::de::Error::duplicate_field("closingTo"));
                            }
                            closing_to__ = map_.next_value()?;
                        }
                        GeneratedField::OrganizerKey => {
                            if organizer_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizerKey"));
                            }
                            organizer_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Closure {
                    id: id__.unwrap_or_default(),
                    closing_from: closing_from__,
                    closing_to: closing_to__,
                    organizer_key: organizer_key__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("booking.v1.Closure", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateBookingRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.booking_holder_key.is_empty() {
            len += 1;
        }
        if !self.slot_id.is_empty() {
            len += 1;
        }
        if !self.date_time.is_empty() {
            len += 1;
        }
        if self.persons != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("booking.v1.CreateBookingRequest", len)?;
        if !self.booking_holder_key.is_empty() {
            struct_ser.serialize_field("bookingHolderKey", &self.booking_holder_key)?;
        }
        if !self.slot_id.is_empty() {
            struct_ser.serialize_field("slotId", &self.slot_id)?;
        }
        if !self.date_time.is_empty() {
            struct_ser.serialize_field("dateTime", &self.date_time)?;
        }
        if self.persons != 0 {
            struct_ser.serialize_field("persons", &self.persons)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateBookingRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "booking_holder_key",
            "bookingHolderKey",
            "slot_id",
            "slotId",
            "date_time",
            "dateTime",
            "persons",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BookingHolderKey,
            SlotId,
            DateTime,
            Persons,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "bookingHolderKey" | "booking_holder_key" => Ok(GeneratedField::BookingHolderKey),
                            "slotId" | "slot_id" => Ok(GeneratedField::SlotId),
                            "dateTime" | "date_time" => Ok(GeneratedField::DateTime),
                            "persons" => Ok(GeneratedField::Persons),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateBookingRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct booking.v1.CreateBookingRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateBookingRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut booking_holder_key__ = None;
                let mut slot_id__ = None;
                let mut date_time__ = None;
                let mut persons__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BookingHolderKey => {
                            if booking_holder_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bookingHolderKey"));
                            }
                            booking_holder_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SlotId => {
                            if slot_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slotId"));
                            }
                            slot_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DateTime => {
                            if date_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dateTime"));
                            }
                            date_time__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Persons => {
                            if persons__.is_some() {
                                return Err(serde::de::Error::duplicate_field("persons"));
                            }
                            persons__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(CreateBookingRequest {
                    booking_holder_key: booking_holder_key__.unwrap_or_default(),
                    slot_id: slot_id__.unwrap_or_default(),
                    date_time: date_time__.unwrap_or_default(),
                    persons: persons__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("booking.v1.CreateBookingRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateBookingResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.booking.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("booking.v1.CreateBookingResponse", len)?;
        if let Some(v) = self.booking.as_ref() {
            struct_ser.serialize_field("booking", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateBookingResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "booking",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Booking,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "booking" => Ok(GeneratedField::Booking),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateBookingResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct booking.v1.CreateBookingResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateBookingResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut booking__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Booking => {
                            if booking__.is_some() {
                                return Err(serde::de::Error::duplicate_field("booking"));
                            }
                            booking__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateBookingResponse {
                    booking: booking__,
                })
            }
        }
        deserializer.deserialize_struct("booking.v1.CreateBookingResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateClosureRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.closing_from.is_empty() {
            len += 1;
        }
        if !self.closing_to.is_empty() {
            len += 1;
        }
        if !self.organizer_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("booking.v1.CreateClosureRequest", len)?;
        if !self.closing_from.is_empty() {
            struct_ser.serialize_field("closingFrom", &self.closing_from)?;
        }
        if !self.closing_to.is_empty() {
            struct_ser.serialize_field("closingTo", &self.closing_to)?;
        }
        if !self.organizer_key.is_empty() {
            struct_ser.serialize_field("organizerKey", &self.organizer_key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateClosureRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "closing_from",
            "closingFrom",
            "closing_to",
            "closingTo",
            "organizer_key",
            "organizerKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClosingFrom,
            ClosingTo,
            OrganizerKey,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "closingFrom" | "closing_from" => Ok(GeneratedField::ClosingFrom),
                            "closingTo" | "closing_to" => Ok(GeneratedField::ClosingTo),
                            "organizerKey" | "organizer_key" => Ok(GeneratedField::OrganizerKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateClosureRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct booking.v1.CreateClosureRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateClosureRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut closing_from__ = None;
                let mut closing_to__ = None;
                let mut organizer_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClosingFrom => {
                            if closing_from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("closingFrom"));
                            }
                            closing_from__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClosingTo => {
                            if closing_to__.is_some() {
                                return Err(serde::de::Error::duplicate_field("closingTo"));
                            }
                            closing_to__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrganizerKey => {
                            if organizer_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizerKey"));
                            }
                            organizer_key__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateClosureRequest {
                    closing_from: closing_from__.unwrap_or_default(),
                    closing_to: closing_to__.unwrap_or_default(),
                    organizer_key: organizer_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("booking.v1.CreateClosureRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateClosureResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.closure.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("booking.v1.CreateClosureResponse", len)?;
        if let Some(v) = self.closure.as_ref() {
            struct_ser.serialize_field("closure", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateClosureResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "closure",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Closure,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "closure" => Ok(GeneratedField::Closure),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateClosureResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct booking.v1.CreateClosureResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateClosureResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut closure__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Closure => {
                            if closure__.is_some() {
                                return Err(serde::de::Error::duplicate_field("closure"));
                            }
                            closure__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateClosureResponse {
                    closure: closure__,
                })
            }
        }
        deserializer.deserialize_struct("booking.v1.CreateClosureResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateEventRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.start.is_empty() {
            len += 1;
        }
        if !self.end.is_empty() {
            len += 1;
        }
        if !self.timezone.is_empty() {
            len += 1;
        }
        if !self.organizer_key.is_empty() {
            len += 1;
        }
        if self.slot_duration != 0 {
            len += 1;
        }
        if self.capacity != 0 {
            len += 1;
        }
        if self.slot_capacity != 0 {
            len += 1;
        }
        if !self.recurrence_rule.is_empty() {
            len += 1;
        }
        if self.event_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("booking.v1.CreateEventRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.start.is_empty() {
            struct_ser.serialize_field("start", &self.start)?;
        }
        if !self.end.is_empty() {
            struct_ser.serialize_field("end", &self.end)?;
        }
        if !self.timezone.is_empty() {
            struct_ser.serialize_field("timezone", &self.timezone)?;
        }
        if !self.organizer_key.is_empty() {
            struct_ser.serialize_field("organizerKey", &self.organizer_key)?;
        }
        if self.slot_duration != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("slotDuration", ToString::to_string(&self.slot_duration).as_str())?;
        }
        if self.capacity != 0 {
            struct_ser.serialize_field("capacity", &self.capacity)?;
        }
        if self.slot_capacity != 0 {
            struct_ser.serialize_field("slotCapacity", &self.slot_capacity)?;
        }
        if !self.recurrence_rule.is_empty() {
            struct_ser.serialize_field("recurrenceRule", &self.recurrence_rule)?;
        }
        if self.event_type != 0 {
            let v = EventType::try_from(self.event_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.event_type)))?;
            struct_ser.serialize_field("eventType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateEventRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "start",
            "end",
            "timezone",
            "organizer_key",
            "organizerKey",
            "slot_duration",
            "slotDuration",
            "capacity",
            "slot_capacity",
            "slotCapacity",
            "recurrence_rule",
            "recurrenceRule",
            "event_type",
            "eventType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Start,
            End,
            Timezone,
            OrganizerKey,
            SlotDuration,
            Capacity,
            SlotCapacity,
            RecurrenceRule,
            EventType,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "start" => Ok(GeneratedField::Start),
                            "end" => Ok(GeneratedField::End),
                            "timezone" => Ok(GeneratedField::Timezone),
                            "organizerKey" | "organizer_key" => Ok(GeneratedField::OrganizerKey),
                            "slotDuration" | "slot_duration" => Ok(GeneratedField::SlotDuration),
                            "capacity" => Ok(GeneratedField::Capacity),
                            "slotCapacity" | "slot_capacity" => Ok(GeneratedField::SlotCapacity),
                            "recurrenceRule" | "recurrence_rule" => Ok(GeneratedField::RecurrenceRule),
                            "eventType" | "event_type" => Ok(GeneratedField::EventType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateEventRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct booking.v1.CreateEventRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateEventRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut start__ = None;
                let mut end__ = None;
                let mut timezone__ = None;
                let mut organizer_key__ = None;
                let mut slot_duration__ = None;
                let mut capacity__ = None;
                let mut slot_capacity__ = None;
                let mut recurrence_rule__ = None;
                let mut event_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Start => {
                            if start__.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            start__ = Some(map_.next_value()?);
                        }
                        GeneratedField::End => {
                            if end__.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            end__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Timezone => {
                            if timezone__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timezone"));
                            }
                            timezone__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrganizerKey => {
                            if organizer_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizerKey"));
                            }
                            organizer_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SlotDuration => {
                            if slot_duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slotDuration"));
                            }
                            slot_duration__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Capacity => {
                            if capacity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("capacity"));
                            }
                            capacity__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SlotCapacity => {
                            if slot_capacity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slotCapacity"));
                            }
                            slot_capacity__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RecurrenceRule => {
                            if recurrence_rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recurrenceRule"));
                            }
                            recurrence_rule__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EventType => {
                            if event_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventType"));
                            }
                            event_type__ = Some(map_.next_value::<EventType>()? as i32);
                        }
                    }
                }
                Ok(CreateEventRequest {
                    name: name__.unwrap_or_default(),
                    start: start__.unwrap_or_default(),
                    end: end__.unwrap_or_default(),
                    timezone: timezone__.unwrap_or_default(),
                    organizer_key: organizer_key__.unwrap_or_default(),
                    slot_duration: slot_duration__.unwrap_or_default(),
                    capacity: capacity__.unwrap_or_default(),
                    slot_capacity: slot_capacity__.unwrap_or_default(),
                    recurrence_rule: recurrence_rule__.unwrap_or_default(),
                    event_type: event_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("booking.v1.CreateEventRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateEventResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.event.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("booking.v1.CreateEventResponse", len)?;
        if let Some(v) = self.event.as_ref() {
            struct_ser.serialize_field("event", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateEventResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "event",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Event,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "event" => Ok(GeneratedField::Event),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateEventResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct booking.v1.CreateEventResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateEventResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut event__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Event => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("event"));
                            }
                            event__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateEventResponse {
                    event: event__,
                })
            }
        }
        deserializer.deserialize_struct("booking.v1.CreateEventResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Event {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.event_type != 0 {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if self.start.is_some() {
            len += 1;
        }
        if self.end.is_some() {
            len += 1;
        }
        if !self.recurrence_rule.is_empty() {
            len += 1;
        }
        if !self.organizer_key.is_empty() {
            len += 1;
        }
        if self.cancellation.is_some() {
            len += 1;
        }
        if !self.slots.is_empty() {
            len += 1;
        }
        if self.slot_duration != 0 {
            len += 1;
        }
        if self.capacity != 0 {
            len += 1;
        }
        if self.created_at != 0 {
            len += 1;
        }
        if self.updated_at != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("booking.v1.Event", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.event_type != 0 {
            let v = EventType::try_from(self.event_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.event_type)))?;
            struct_ser.serialize_field("eventType", &v)?;
        }
        if self.status != 0 {
            let v = EventStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if let Some(v) = self.start.as_ref() {
            struct_ser.serialize_field("start", v)?;
        }
        if let Some(v) = self.end.as_ref() {
            struct_ser.serialize_field("end", v)?;
        }
        if !self.recurrence_rule.is_empty() {
            struct_ser.serialize_field("recurrenceRule", &self.recurrence_rule)?;
        }
        if !self.organizer_key.is_empty() {
            struct_ser.serialize_field("organizerKey", &self.organizer_key)?;
        }
        if let Some(v) = self.cancellation.as_ref() {
            struct_ser.serialize_field("cancellation", v)?;
        }
        if !self.slots.is_empty() {
            struct_ser.serialize_field("slots", &self.slots)?;
        }
        if self.slot_duration != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("slotDuration", ToString::to_string(&self.slot_duration).as_str())?;
        }
        if self.capacity != 0 {
            struct_ser.serialize_field("capacity", &self.capacity)?;
        }
        if self.created_at != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("createdAt", ToString::to_string(&self.created_at).as_str())?;
        }
        if self.updated_at != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("updatedAt", ToString::to_string(&self.updated_at).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Event {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "event_type",
            "eventType",
            "status",
            "start",
            "end",
            "recurrence_rule",
            "recurrenceRule",
            "organizer_key",
            "organizerKey",
            "cancellation",
            "slots",
            "slot_duration",
            "slotDuration",
            "capacity",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            EventType,
            Status,
            Start,
            End,
            RecurrenceRule,
            OrganizerKey,
            Cancellation,
            Slots,
            SlotDuration,
            Capacity,
            CreatedAt,
            UpdatedAt,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "eventType" | "event_type" => Ok(GeneratedField::EventType),
                            "status" => Ok(GeneratedField::Status),
                            "start" => Ok(GeneratedField::Start),
                            "end" => Ok(GeneratedField::End),
                            "recurrenceRule" | "recurrence_rule" => Ok(GeneratedField::RecurrenceRule),
                            "organizerKey" | "organizer_key" => Ok(GeneratedField::OrganizerKey),
                            "cancellation" => Ok(GeneratedField::Cancellation),
                            "slots" => Ok(GeneratedField::Slots),
                            "slotDuration" | "slot_duration" => Ok(GeneratedField::SlotDuration),
                            "capacity" => Ok(GeneratedField::Capacity),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Event;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct booking.v1.Event")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Event, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut event_type__ = None;
                let mut status__ = None;
                let mut start__ = None;
                let mut end__ = None;
                let mut recurrence_rule__ = None;
                let mut organizer_key__ = None;
                let mut cancellation__ = None;
                let mut slots__ = None;
                let mut slot_duration__ = None;
                let mut capacity__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EventType => {
                            if event_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventType"));
                            }
                            event_type__ = Some(map_.next_value::<EventType>()? as i32);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<EventStatus>()? as i32);
                        }
                        GeneratedField::Start => {
                            if start__.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            start__ = map_.next_value()?;
                        }
                        GeneratedField::End => {
                            if end__.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            end__ = map_.next_value()?;
                        }
                        GeneratedField::RecurrenceRule => {
                            if recurrence_rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recurrenceRule"));
                            }
                            recurrence_rule__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrganizerKey => {
                            if organizer_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizerKey"));
                            }
                            organizer_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Cancellation => {
                            if cancellation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cancellation"));
                            }
                            cancellation__ = map_.next_value()?;
                        }
                        GeneratedField::Slots => {
                            if slots__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slots"));
                            }
                            slots__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SlotDuration => {
                            if slot_duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slotDuration"));
                            }
                            slot_duration__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Capacity => {
                            if capacity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("capacity"));
                            }
                            capacity__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Event {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    event_type: event_type__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    start: start__,
                    end: end__,
                    recurrence_rule: recurrence_rule__.unwrap_or_default(),
                    organizer_key: organizer_key__.unwrap_or_default(),
                    cancellation: cancellation__,
                    slots: slots__.unwrap_or_default(),
                    slot_duration: slot_duration__.unwrap_or_default(),
                    capacity: capacity__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("booking.v1.Event", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventInstances {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.event_type != 0 {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if !self.organizer_key.is_empty() {
            len += 1;
        }
        if self.cancellation.is_some() {
            len += 1;
        }
        if self.slot_duration != 0 {
            len += 1;
        }
        if self.capacity != 0 {
            len += 1;
        }
        if self.slot_capacity != 0 {
            len += 1;
        }
        if !self.items.is_empty() {
            len += 1;
        }
        if self.created_at != 0 {
            len += 1;
        }
        if self.updated_at != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("booking.v1.EventInstances", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.event_type != 0 {
            let v = EventType::try_from(self.event_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.event_type)))?;
            struct_ser.serialize_field("eventType", &v)?;
        }
        if self.status != 0 {
            let v = EventStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if !self.organizer_key.is_empty() {
            struct_ser.serialize_field("organizerKey", &self.organizer_key)?;
        }
        if let Some(v) = self.cancellation.as_ref() {
            struct_ser.serialize_field("cancellation", v)?;
        }
        if self.slot_duration != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("slotDuration", ToString::to_string(&self.slot_duration).as_str())?;
        }
        if self.capacity != 0 {
            struct_ser.serialize_field("capacity", &self.capacity)?;
        }
        if self.slot_capacity != 0 {
            struct_ser.serialize_field("slotCapacity", &self.slot_capacity)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        if self.created_at != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("createdAt", ToString::to_string(&self.created_at).as_str())?;
        }
        if self.updated_at != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("updatedAt", ToString::to_string(&self.updated_at).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventInstances {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "event_type",
            "eventType",
            "status",
            "organizer_key",
            "organizerKey",
            "cancellation",
            "slot_duration",
            "slotDuration",
            "capacity",
            "slot_capacity",
            "slotCapacity",
            "items",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            EventType,
            Status,
            OrganizerKey,
            Cancellation,
            SlotDuration,
            Capacity,
            SlotCapacity,
            Items,
            CreatedAt,
            UpdatedAt,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "eventType" | "event_type" => Ok(GeneratedField::EventType),
                            "status" => Ok(GeneratedField::Status),
                            "organizerKey" | "organizer_key" => Ok(GeneratedField::OrganizerKey),
                            "cancellation" => Ok(GeneratedField::Cancellation),
                            "slotDuration" | "slot_duration" => Ok(GeneratedField::SlotDuration),
                            "capacity" => Ok(GeneratedField::Capacity),
                            "slotCapacity" | "slot_capacity" => Ok(GeneratedField::SlotCapacity),
                            "items" => Ok(GeneratedField::Items),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventInstances;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct booking.v1.EventInstances")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventInstances, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut event_type__ = None;
                let mut status__ = None;
                let mut organizer_key__ = None;
                let mut cancellation__ = None;
                let mut slot_duration__ = None;
                let mut capacity__ = None;
                let mut slot_capacity__ = None;
                let mut items__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EventType => {
                            if event_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventType"));
                            }
                            event_type__ = Some(map_.next_value::<EventType>()? as i32);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<EventStatus>()? as i32);
                        }
                        GeneratedField::OrganizerKey => {
                            if organizer_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizerKey"));
                            }
                            organizer_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Cancellation => {
                            if cancellation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cancellation"));
                            }
                            cancellation__ = map_.next_value()?;
                        }
                        GeneratedField::SlotDuration => {
                            if slot_duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slotDuration"));
                            }
                            slot_duration__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Capacity => {
                            if capacity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("capacity"));
                            }
                            capacity__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SlotCapacity => {
                            if slot_capacity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slotCapacity"));
                            }
                            slot_capacity__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Items => {
                            if items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("items"));
                            }
                            items__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(EventInstances {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    event_type: event_type__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    organizer_key: organizer_key__.unwrap_or_default(),
                    cancellation: cancellation__,
                    slot_duration: slot_duration__.unwrap_or_default(),
                    capacity: capacity__.unwrap_or_default(),
                    slot_capacity: slot_capacity__.unwrap_or_default(),
                    items: items__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("booking.v1.EventInstances", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "EVENT_STATUS_UNSPECIFIED",
            Self::Active => "EVENT_STATUS_ACTIVE",
            Self::Canceled => "EVENT_STATUS_CANCELED",
            Self::Full => "EVENT_STATUS_FULL",
            Self::Disable => "EVENT_STATUS_DISABLE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for EventStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "EVENT_STATUS_UNSPECIFIED",
            "EVENT_STATUS_ACTIVE",
            "EVENT_STATUS_CANCELED",
            "EVENT_STATUS_FULL",
            "EVENT_STATUS_DISABLE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "EVENT_STATUS_UNSPECIFIED" => Ok(EventStatus::Unspecified),
                    "EVENT_STATUS_ACTIVE" => Ok(EventStatus::Active),
                    "EVENT_STATUS_CANCELED" => Ok(EventStatus::Canceled),
                    "EVENT_STATUS_FULL" => Ok(EventStatus::Full),
                    "EVENT_STATUS_DISABLE" => Ok(EventStatus::Disable),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for EventType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "EVENT_TYPE_UNSPECIFIED",
            Self::Event => "EVENT_TYPE_EVENT",
            Self::Task => "EVENT_TYPE_TASK",
            Self::Meeting => "EVENT_TYPE_MEETING",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for EventType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "EVENT_TYPE_UNSPECIFIED",
            "EVENT_TYPE_EVENT",
            "EVENT_TYPE_TASK",
            "EVENT_TYPE_MEETING",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "EVENT_TYPE_UNSPECIFIED" => Ok(EventType::Unspecified),
                    "EVENT_TYPE_EVENT" => Ok(EventType::Event),
                    "EVENT_TYPE_TASK" => Ok(EventType::Task),
                    "EVENT_TYPE_MEETING" => Ok(EventType::Meeting),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Filters {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.from.is_empty() {
            len += 1;
        }
        if !self.to.is_empty() {
            len += 1;
        }
        if !self.organizer_key.is_empty() {
            len += 1;
        }
        if self.limit != 0 {
            len += 1;
        }
        if self.offset != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("booking.v1.Filters", len)?;
        if !self.from.is_empty() {
            struct_ser.serialize_field("from", &self.from)?;
        }
        if !self.to.is_empty() {
            struct_ser.serialize_field("to", &self.to)?;
        }
        if !self.organizer_key.is_empty() {
            struct_ser.serialize_field("organizerKey", &self.organizer_key)?;
        }
        if self.limit != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("limit", ToString::to_string(&self.limit).as_str())?;
        }
        if self.offset != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("offset", ToString::to_string(&self.offset).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Filters {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "from",
            "to",
            "organizer_key",
            "organizerKey",
            "limit",
            "offset",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            From,
            To,
            OrganizerKey,
            Limit,
            Offset,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "from" => Ok(GeneratedField::From),
                            "to" => Ok(GeneratedField::To),
                            "organizerKey" | "organizer_key" => Ok(GeneratedField::OrganizerKey),
                            "limit" => Ok(GeneratedField::Limit),
                            "offset" => Ok(GeneratedField::Offset),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Filters;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct booking.v1.Filters")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Filters, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut from__ = None;
                let mut to__ = None;
                let mut organizer_key__ = None;
                let mut limit__ = None;
                let mut offset__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = Some(map_.next_value()?);
                        }
                        GeneratedField::To => {
                            if to__.is_some() {
                                return Err(serde::de::Error::duplicate_field("to"));
                            }
                            to__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrganizerKey => {
                            if organizer_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizerKey"));
                            }
                            organizer_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Limit => {
                            if limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            limit__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Offset => {
                            if offset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offset"));
                            }
                            offset__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Filters {
                    from: from__.unwrap_or_default(),
                    to: to__.unwrap_or_default(),
                    organizer_key: organizer_key__.unwrap_or_default(),
                    limit: limit__.unwrap_or_default(),
                    offset: offset__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("booking.v1.Filters", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetActiveEventsInstancesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.filters.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("booking.v1.GetActiveEventsInstancesRequest", len)?;
        if let Some(v) = self.filters.as_ref() {
            struct_ser.serialize_field("filters", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetActiveEventsInstancesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "filters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Filters,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "filters" => Ok(GeneratedField::Filters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetActiveEventsInstancesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct booking.v1.GetActiveEventsInstancesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetActiveEventsInstancesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filters__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Filters => {
                            if filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filters"));
                            }
                            filters__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetActiveEventsInstancesRequest {
                    filters: filters__,
                })
            }
        }
        deserializer.deserialize_struct("booking.v1.GetActiveEventsInstancesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetActiveEventsInstancesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.events.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("booking.v1.GetActiveEventsInstancesResponse", len)?;
        if !self.events.is_empty() {
            struct_ser.serialize_field("events", &self.events)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetActiveEventsInstancesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "events",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Events,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "events" => Ok(GeneratedField::Events),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetActiveEventsInstancesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct booking.v1.GetActiveEventsInstancesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetActiveEventsInstancesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut events__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Events => {
                            if events__.is_some() {
                                return Err(serde::de::Error::duplicate_field("events"));
                            }
                            events__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetActiveEventsInstancesResponse {
                    events: events__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("booking.v1.GetActiveEventsInstancesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetActiveEventsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.filters.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("booking.v1.GetActiveEventsRequest", len)?;
        if let Some(v) = self.filters.as_ref() {
            struct_ser.serialize_field("filters", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetActiveEventsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "filters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Filters,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "filters" => Ok(GeneratedField::Filters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetActiveEventsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct booking.v1.GetActiveEventsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetActiveEventsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filters__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Filters => {
                            if filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filters"));
                            }
                            filters__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetActiveEventsRequest {
                    filters: filters__,
                })
            }
        }
        deserializer.deserialize_struct("booking.v1.GetActiveEventsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetActiveEventsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.events.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("booking.v1.GetActiveEventsResponse", len)?;
        if !self.events.is_empty() {
            struct_ser.serialize_field("events", &self.events)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetActiveEventsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "events",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Events,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "events" => Ok(GeneratedField::Events),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetActiveEventsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct booking.v1.GetActiveEventsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetActiveEventsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut events__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Events => {
                            if events__.is_some() {
                                return Err(serde::de::Error::duplicate_field("events"));
                            }
                            events__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetActiveEventsResponse {
                    events: events__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("booking.v1.GetActiveEventsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetBookingRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("booking.v1.GetBookingRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetBookingRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetBookingRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct booking.v1.GetBookingRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetBookingRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetBookingRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("booking.v1.GetBookingRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetBookingResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.booking.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("booking.v1.GetBookingResponse", len)?;
        if let Some(v) = self.booking.as_ref() {
            struct_ser.serialize_field("booking", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetBookingResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "booking",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Booking,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "booking" => Ok(GeneratedField::Booking),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetBookingResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct booking.v1.GetBookingResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetBookingResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut booking__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Booking => {
                            if booking__.is_some() {
                                return Err(serde::de::Error::duplicate_field("booking"));
                            }
                            booking__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetBookingResponse {
                    booking: booking__,
                })
            }
        }
        deserializer.deserialize_struct("booking.v1.GetBookingResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetEventInstancesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.event_id.is_empty() {
            len += 1;
        }
        if self.filters.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("booking.v1.GetEventInstancesRequest", len)?;
        if !self.event_id.is_empty() {
            struct_ser.serialize_field("eventId", &self.event_id)?;
        }
        if let Some(v) = self.filters.as_ref() {
            struct_ser.serialize_field("filters", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetEventInstancesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "event_id",
            "eventId",
            "filters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EventId,
            Filters,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "eventId" | "event_id" => Ok(GeneratedField::EventId),
                            "filters" => Ok(GeneratedField::Filters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetEventInstancesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct booking.v1.GetEventInstancesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetEventInstancesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut event_id__ = None;
                let mut filters__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EventId => {
                            if event_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventId"));
                            }
                            event_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Filters => {
                            if filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filters"));
                            }
                            filters__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetEventInstancesRequest {
                    event_id: event_id__.unwrap_or_default(),
                    filters: filters__,
                })
            }
        }
        deserializer.deserialize_struct("booking.v1.GetEventInstancesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetEventInstancesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.event.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("booking.v1.GetEventInstancesResponse", len)?;
        if let Some(v) = self.event.as_ref() {
            struct_ser.serialize_field("event", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetEventInstancesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "event",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Event,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "event" => Ok(GeneratedField::Event),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetEventInstancesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct booking.v1.GetEventInstancesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetEventInstancesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut event__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Event => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("event"));
                            }
                            event__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetEventInstancesResponse {
                    event: event__,
                })
            }
        }
        deserializer.deserialize_struct("booking.v1.GetEventInstancesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetEventRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("booking.v1.GetEventRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetEventRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetEventRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct booking.v1.GetEventRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetEventRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetEventRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("booking.v1.GetEventRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetEventResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.event.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("booking.v1.GetEventResponse", len)?;
        if let Some(v) = self.event.as_ref() {
            struct_ser.serialize_field("event", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetEventResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "event",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Event,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "event" => Ok(GeneratedField::Event),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetEventResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct booking.v1.GetEventResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetEventResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut event__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Event => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("event"));
                            }
                            event__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetEventResponse {
                    event: event__,
                })
            }
        }
        deserializer.deserialize_struct("booking.v1.GetEventResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Slot {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.event_id.is_empty() {
            len += 1;
        }
        if self.start.is_some() {
            len += 1;
        }
        if self.end.is_some() {
            len += 1;
        }
        if self.capacity != 0 {
            len += 1;
        }
        if self.created_at != 0 {
            len += 1;
        }
        if self.updated_at != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("booking.v1.Slot", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.event_id.is_empty() {
            struct_ser.serialize_field("eventId", &self.event_id)?;
        }
        if let Some(v) = self.start.as_ref() {
            struct_ser.serialize_field("start", v)?;
        }
        if let Some(v) = self.end.as_ref() {
            struct_ser.serialize_field("end", v)?;
        }
        if self.capacity != 0 {
            struct_ser.serialize_field("capacity", &self.capacity)?;
        }
        if self.created_at != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("createdAt", ToString::to_string(&self.created_at).as_str())?;
        }
        if self.updated_at != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("updatedAt", ToString::to_string(&self.updated_at).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Slot {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "event_id",
            "eventId",
            "start",
            "end",
            "capacity",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            EventId,
            Start,
            End,
            Capacity,
            CreatedAt,
            UpdatedAt,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "eventId" | "event_id" => Ok(GeneratedField::EventId),
                            "start" => Ok(GeneratedField::Start),
                            "end" => Ok(GeneratedField::End),
                            "capacity" => Ok(GeneratedField::Capacity),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Slot;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct booking.v1.Slot")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Slot, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut event_id__ = None;
                let mut start__ = None;
                let mut end__ = None;
                let mut capacity__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EventId => {
                            if event_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventId"));
                            }
                            event_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Start => {
                            if start__.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            start__ = map_.next_value()?;
                        }
                        GeneratedField::End => {
                            if end__.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            end__ = map_.next_value()?;
                        }
                        GeneratedField::Capacity => {
                            if capacity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("capacity"));
                            }
                            capacity__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Slot {
                    id: id__.unwrap_or_default(),
                    event_id: event_id__.unwrap_or_default(),
                    start: start__,
                    end: end__,
                    capacity: capacity__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("booking.v1.Slot", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TimeData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.timezone.is_empty() {
            len += 1;
        }
        if !self.date_time.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("booking.v1.TimeData", len)?;
        if !self.timezone.is_empty() {
            struct_ser.serialize_field("timezone", &self.timezone)?;
        }
        if !self.date_time.is_empty() {
            struct_ser.serialize_field("dateTime", &self.date_time)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TimeData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "timezone",
            "date_time",
            "dateTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Timezone,
            DateTime,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "timezone" => Ok(GeneratedField::Timezone),
                            "dateTime" | "date_time" => Ok(GeneratedField::DateTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TimeData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct booking.v1.TimeData")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TimeData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut timezone__ = None;
                let mut date_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Timezone => {
                            if timezone__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timezone"));
                            }
                            timezone__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DateTime => {
                            if date_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dateTime"));
                            }
                            date_time__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TimeData {
                    timezone: timezone__.unwrap_or_default(),
                    date_time: date_time__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("booking.v1.TimeData", FIELDS, GeneratedVisitor)
    }
}
