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
        let mut struct_ser = serializer.serialize_struct("event.v1.Booking", len)?;
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
                formatter.write_str("struct event.v1.Booking")
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
        deserializer.deserialize_struct("event.v1.Booking", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CancelEventRequest {
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
        if !self.canceled_by.is_empty() {
            len += 1;
        }
        if !self.reason.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("event.v1.CancelEventRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.canceled_by.is_empty() {
            struct_ser.serialize_field("canceledBy", &self.canceled_by)?;
        }
        if !self.reason.is_empty() {
            struct_ser.serialize_field("reason", &self.reason)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CancelEventRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "canceled_by",
            "canceledBy",
            "reason",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            CanceledBy,
            Reason,
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
                            "canceledBy" | "canceled_by" => Ok(GeneratedField::CanceledBy),
                            "reason" => Ok(GeneratedField::Reason),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CancelEventRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct event.v1.CancelEventRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CancelEventRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut canceled_by__ = None;
                let mut reason__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
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
                    }
                }
                Ok(CancelEventRequest {
                    id: id__.unwrap_or_default(),
                    canceled_by: canceled_by__.unwrap_or_default(),
                    reason: reason__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("event.v1.CancelEventRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CancelEventResponse {
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
        let mut struct_ser = serializer.serialize_struct("event.v1.CancelEventResponse", len)?;
        if let Some(v) = self.event.as_ref() {
            struct_ser.serialize_field("event", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CancelEventResponse {
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
            type Value = CancelEventResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct event.v1.CancelEventResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CancelEventResponse, V::Error>
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
                Ok(CancelEventResponse {
                    event: event__,
                })
            }
        }
        deserializer.deserialize_struct("event.v1.CancelEventResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("event.v1.Cancellation", len)?;
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
                formatter.write_str("struct event.v1.Cancellation")
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
        deserializer.deserialize_struct("event.v1.Cancellation", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("event.v1.Closure", len)?;
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
                formatter.write_str("struct event.v1.Closure")
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
        deserializer.deserialize_struct("event.v1.Closure", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("event.v1.CreateBookingRequest", len)?;
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
                formatter.write_str("struct event.v1.CreateBookingRequest")
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
        deserializer.deserialize_struct("event.v1.CreateBookingRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("event.v1.CreateBookingResponse", len)?;
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
                formatter.write_str("struct event.v1.CreateBookingResponse")
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
        deserializer.deserialize_struct("event.v1.CreateBookingResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("event.v1.CreateClosureRequest", len)?;
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
                formatter.write_str("struct event.v1.CreateClosureRequest")
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
        deserializer.deserialize_struct("event.v1.CreateClosureRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("event.v1.CreateClosureResponse", len)?;
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
                formatter.write_str("struct event.v1.CreateClosureResponse")
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
        deserializer.deserialize_struct("event.v1.CreateClosureResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("event.v1.CreateEventRequest", len)?;
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
                formatter.write_str("struct event.v1.CreateEventRequest")
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
        deserializer.deserialize_struct("event.v1.CreateEventRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("event.v1.CreateEventResponse", len)?;
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
                formatter.write_str("struct event.v1.CreateEventResponse")
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
        deserializer.deserialize_struct("event.v1.CreateEventResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteBookingRequest {
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
        let mut struct_ser = serializer.serialize_struct("event.v1.DeleteBookingRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteBookingRequest {
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
            type Value = DeleteBookingRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct event.v1.DeleteBookingRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteBookingRequest, V::Error>
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
                Ok(DeleteBookingRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("event.v1.DeleteBookingRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteBookingResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("event.v1.DeleteBookingResponse", len)?;
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteBookingResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Message,
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
                            "message" => Ok(GeneratedField::Message),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteBookingResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct event.v1.DeleteBookingResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteBookingResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteBookingResponse {
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("event.v1.DeleteBookingResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteClosureRequest {
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
        let mut struct_ser = serializer.serialize_struct("event.v1.DeleteClosureRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteClosureRequest {
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
            type Value = DeleteClosureRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct event.v1.DeleteClosureRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteClosureRequest, V::Error>
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
                Ok(DeleteClosureRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("event.v1.DeleteClosureRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteClosureResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("event.v1.DeleteClosureResponse", len)?;
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteClosureResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Message,
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
                            "message" => Ok(GeneratedField::Message),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteClosureResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct event.v1.DeleteClosureResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteClosureResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteClosureResponse {
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("event.v1.DeleteClosureResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteEventRequest {
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
        let mut struct_ser = serializer.serialize_struct("event.v1.DeleteEventRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteEventRequest {
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
            type Value = DeleteEventRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct event.v1.DeleteEventRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteEventRequest, V::Error>
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
                Ok(DeleteEventRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("event.v1.DeleteEventRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteEventResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("event.v1.DeleteEventResponse", len)?;
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteEventResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Message,
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
                            "message" => Ok(GeneratedField::Message),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteEventResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct event.v1.DeleteEventResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteEventResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteEventResponse {
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("event.v1.DeleteEventResponse", FIELDS, GeneratedVisitor)
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
        if self.overlap {
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
        let mut struct_ser = serializer.serialize_struct("event.v1.Event", len)?;
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
        if self.overlap {
            struct_ser.serialize_field("overlap", &self.overlap)?;
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
            "overlap",
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
            Overlap,
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
                            "overlap" => Ok(GeneratedField::Overlap),
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
                formatter.write_str("struct event.v1.Event")
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
                let mut overlap__ = None;
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
                        GeneratedField::Overlap => {
                            if overlap__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overlap"));
                            }
                            overlap__ = Some(map_.next_value()?);
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
                    overlap: overlap__.unwrap_or_default(),
                    slots: slots__.unwrap_or_default(),
                    slot_duration: slot_duration__.unwrap_or_default(),
                    capacity: capacity__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("event.v1.Event", FIELDS, GeneratedVisitor)
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
            Self::Closed => "EVENT_STATUS_CLOSED",
            Self::Full => "EVENT_STATUS_FULL",
            Self::Disabled => "EVENT_STATUS_DISABLED",
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
            "EVENT_STATUS_CLOSED",
            "EVENT_STATUS_FULL",
            "EVENT_STATUS_DISABLED",
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
                    "EVENT_STATUS_CLOSED" => Ok(EventStatus::Closed),
                    "EVENT_STATUS_FULL" => Ok(EventStatus::Full),
                    "EVENT_STATUS_DISABLED" => Ok(EventStatus::Disabled),
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
        if self.status != 0 {
            len += 1;
        }
        if self.event_type != 0 {
            len += 1;
        }
        if !self.booking_holder_key.is_empty() {
            len += 1;
        }
        if !self.slot_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("event.v1.Filters", len)?;
        if !self.from.is_empty() {
            struct_ser.serialize_field("from", &self.from)?;
        }
        if !self.to.is_empty() {
            struct_ser.serialize_field("to", &self.to)?;
        }
        if !self.organizer_key.is_empty() {
            struct_ser.serialize_field("organizerKey", &self.organizer_key)?;
        }
        if self.status != 0 {
            let v = EventStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if self.event_type != 0 {
            let v = EventType::try_from(self.event_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.event_type)))?;
            struct_ser.serialize_field("eventType", &v)?;
        }
        if !self.booking_holder_key.is_empty() {
            struct_ser.serialize_field("bookingHolderKey", &self.booking_holder_key)?;
        }
        if !self.slot_id.is_empty() {
            struct_ser.serialize_field("slotId", &self.slot_id)?;
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
            "status",
            "event_type",
            "eventType",
            "booking_holder_key",
            "bookingHolderKey",
            "slot_id",
            "slotId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            From,
            To,
            OrganizerKey,
            Status,
            EventType,
            BookingHolderKey,
            SlotId,
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
                            "status" => Ok(GeneratedField::Status),
                            "eventType" | "event_type" => Ok(GeneratedField::EventType),
                            "bookingHolderKey" | "booking_holder_key" => Ok(GeneratedField::BookingHolderKey),
                            "slotId" | "slot_id" => Ok(GeneratedField::SlotId),
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
                formatter.write_str("struct event.v1.Filters")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Filters, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut from__ = None;
                let mut to__ = None;
                let mut organizer_key__ = None;
                let mut status__ = None;
                let mut event_type__ = None;
                let mut booking_holder_key__ = None;
                let mut slot_id__ = None;
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
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<EventStatus>()? as i32);
                        }
                        GeneratedField::EventType => {
                            if event_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventType"));
                            }
                            event_type__ = Some(map_.next_value::<EventType>()? as i32);
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
                    }
                }
                Ok(Filters {
                    from: from__.unwrap_or_default(),
                    to: to__.unwrap_or_default(),
                    organizer_key: organizer_key__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    event_type: event_type__.unwrap_or_default(),
                    booking_holder_key: booking_holder_key__.unwrap_or_default(),
                    slot_id: slot_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("event.v1.Filters", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("event.v1.GetBookingRequest", len)?;
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
                formatter.write_str("struct event.v1.GetBookingRequest")
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
        deserializer.deserialize_struct("event.v1.GetBookingRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("event.v1.GetBookingResponse", len)?;
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
                formatter.write_str("struct event.v1.GetBookingResponse")
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
        deserializer.deserialize_struct("event.v1.GetBookingResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("event.v1.GetEventRequest", len)?;
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
                formatter.write_str("struct event.v1.GetEventRequest")
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
        deserializer.deserialize_struct("event.v1.GetEventRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("event.v1.GetEventResponse", len)?;
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
                formatter.write_str("struct event.v1.GetEventResponse")
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
        deserializer.deserialize_struct("event.v1.GetEventResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetListEventsRequest {
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
        let mut struct_ser = serializer.serialize_struct("event.v1.GetListEventsRequest", len)?;
        if let Some(v) = self.filters.as_ref() {
            struct_ser.serialize_field("filters", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetListEventsRequest {
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
            type Value = GetListEventsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct event.v1.GetListEventsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetListEventsRequest, V::Error>
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
                Ok(GetListEventsRequest {
                    filters: filters__,
                })
            }
        }
        deserializer.deserialize_struct("event.v1.GetListEventsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetListEventsResponse {
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
        let mut struct_ser = serializer.serialize_struct("event.v1.GetListEventsResponse", len)?;
        if !self.events.is_empty() {
            struct_ser.serialize_field("events", &self.events)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetListEventsResponse {
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
            type Value = GetListEventsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct event.v1.GetListEventsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetListEventsResponse, V::Error>
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
                Ok(GetListEventsResponse {
                    events: events__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("event.v1.GetListEventsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTimelineRequest {
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
        let mut struct_ser = serializer.serialize_struct("event.v1.GetTimelineRequest", len)?;
        if let Some(v) = self.filters.as_ref() {
            struct_ser.serialize_field("filters", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTimelineRequest {
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
            type Value = GetTimelineRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct event.v1.GetTimelineRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetTimelineRequest, V::Error>
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
                Ok(GetTimelineRequest {
                    filters: filters__,
                })
            }
        }
        deserializer.deserialize_struct("event.v1.GetTimelineRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTimelineResponse {
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
        let mut struct_ser = serializer.serialize_struct("event.v1.GetTimelineResponse", len)?;
        if !self.events.is_empty() {
            struct_ser.serialize_field("events", &self.events)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTimelineResponse {
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
            type Value = GetTimelineResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct event.v1.GetTimelineResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetTimelineResponse, V::Error>
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
                Ok(GetTimelineResponse {
                    events: events__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("event.v1.GetTimelineResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListBookingsRequest {
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
        let mut struct_ser = serializer.serialize_struct("event.v1.ListBookingsRequest", len)?;
        if let Some(v) = self.filters.as_ref() {
            struct_ser.serialize_field("filters", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListBookingsRequest {
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
            type Value = ListBookingsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct event.v1.ListBookingsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListBookingsRequest, V::Error>
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
                Ok(ListBookingsRequest {
                    filters: filters__,
                })
            }
        }
        deserializer.deserialize_struct("event.v1.ListBookingsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListBookingsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.bookings.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("event.v1.ListBookingsResponse", len)?;
        if !self.bookings.is_empty() {
            struct_ser.serialize_field("bookings", &self.bookings)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListBookingsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bookings",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Bookings,
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
                            "bookings" => Ok(GeneratedField::Bookings),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListBookingsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct event.v1.ListBookingsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListBookingsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bookings__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Bookings => {
                            if bookings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bookings"));
                            }
                            bookings__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListBookingsResponse {
                    bookings: bookings__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("event.v1.ListBookingsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListClosuresRequest {
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
        let mut struct_ser = serializer.serialize_struct("event.v1.ListClosuresRequest", len)?;
        if let Some(v) = self.filters.as_ref() {
            struct_ser.serialize_field("filters", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListClosuresRequest {
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
            type Value = ListClosuresRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct event.v1.ListClosuresRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListClosuresRequest, V::Error>
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
                Ok(ListClosuresRequest {
                    filters: filters__,
                })
            }
        }
        deserializer.deserialize_struct("event.v1.ListClosuresRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListClosuresResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.closures.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("event.v1.ListClosuresResponse", len)?;
        if !self.closures.is_empty() {
            struct_ser.serialize_field("closures", &self.closures)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListClosuresResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "closures",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Closures,
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
                            "closures" => Ok(GeneratedField::Closures),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListClosuresResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct event.v1.ListClosuresResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListClosuresResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut closures__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Closures => {
                            if closures__.is_some() {
                                return Err(serde::de::Error::duplicate_field("closures"));
                            }
                            closures__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListClosuresResponse {
                    closures: closures__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("event.v1.ListClosuresResponse", FIELDS, GeneratedVisitor)
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
        if self.status != 0 {
            len += 1;
        }
        if self.created_at != 0 {
            len += 1;
        }
        if self.updated_at != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("event.v1.Slot", len)?;
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
        if self.status != 0 {
            let v = SlotStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
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
            "status",
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
            Status,
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
                            "status" => Ok(GeneratedField::Status),
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
                formatter.write_str("struct event.v1.Slot")
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
                let mut status__ = None;
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
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<SlotStatus>()? as i32);
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
                    status: status__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("event.v1.Slot", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SlotStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SLOT_STATUS_UNSPECIFIED",
            Self::Available => "SLOT_STATUS_AVAILABLE",
            Self::Full => "SLOT_STATUS_FULL",
            Self::Closed => "SLOT_STATUS_CLOSED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SlotStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SLOT_STATUS_UNSPECIFIED",
            "SLOT_STATUS_AVAILABLE",
            "SLOT_STATUS_FULL",
            "SLOT_STATUS_CLOSED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SlotStatus;

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
                    "SLOT_STATUS_UNSPECIFIED" => Ok(SlotStatus::Unspecified),
                    "SLOT_STATUS_AVAILABLE" => Ok(SlotStatus::Available),
                    "SLOT_STATUS_FULL" => Ok(SlotStatus::Full),
                    "SLOT_STATUS_CLOSED" => Ok(SlotStatus::Closed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("event.v1.TimeData", len)?;
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
                formatter.write_str("struct event.v1.TimeData")
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
        deserializer.deserialize_struct("event.v1.TimeData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateClosureRequest {
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
        if !self.closing_from.is_empty() {
            len += 1;
        }
        if !self.closing_to.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("event.v1.UpdateClosureRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.closing_from.is_empty() {
            struct_ser.serialize_field("closingFrom", &self.closing_from)?;
        }
        if !self.closing_to.is_empty() {
            struct_ser.serialize_field("closingTo", &self.closing_to)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateClosureRequest {
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
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            ClosingFrom,
            ClosingTo,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateClosureRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct event.v1.UpdateClosureRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateClosureRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut closing_from__ = None;
                let mut closing_to__ = None;
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
                            closing_from__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClosingTo => {
                            if closing_to__.is_some() {
                                return Err(serde::de::Error::duplicate_field("closingTo"));
                            }
                            closing_to__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateClosureRequest {
                    id: id__.unwrap_or_default(),
                    closing_from: closing_from__.unwrap_or_default(),
                    closing_to: closing_to__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("event.v1.UpdateClosureRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateClosureResponse {
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
        let mut struct_ser = serializer.serialize_struct("event.v1.UpdateClosureResponse", len)?;
        if let Some(v) = self.closure.as_ref() {
            struct_ser.serialize_field("closure", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateClosureResponse {
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
            type Value = UpdateClosureResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct event.v1.UpdateClosureResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateClosureResponse, V::Error>
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
                Ok(UpdateClosureResponse {
                    closure: closure__,
                })
            }
        }
        deserializer.deserialize_struct("event.v1.UpdateClosureResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateEventRequest {
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
        if !self.start.is_empty() {
            len += 1;
        }
        if !self.end.is_empty() {
            len += 1;
        }
        if !self.timezone.is_empty() {
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
        let mut struct_ser = serializer.serialize_struct("event.v1.UpdateEventRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
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
        if self.capacity != 0 {
            struct_ser.serialize_field("capacity", &self.capacity)?;
        }
        if self.slot_capacity != 0 {
            struct_ser.serialize_field("slotCapacity", &self.slot_capacity)?;
        }
        if !self.recurrence_rule.is_empty() {
            struct_ser.serialize_field("recurrenceRule", &self.recurrence_rule)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateEventRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "start",
            "end",
            "timezone",
            "capacity",
            "slot_capacity",
            "slotCapacity",
            "recurrence_rule",
            "recurrenceRule",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Start,
            End,
            Timezone,
            Capacity,
            SlotCapacity,
            RecurrenceRule,
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
                            "start" => Ok(GeneratedField::Start),
                            "end" => Ok(GeneratedField::End),
                            "timezone" => Ok(GeneratedField::Timezone),
                            "capacity" => Ok(GeneratedField::Capacity),
                            "slotCapacity" | "slot_capacity" => Ok(GeneratedField::SlotCapacity),
                            "recurrenceRule" | "recurrence_rule" => Ok(GeneratedField::RecurrenceRule),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateEventRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct event.v1.UpdateEventRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateEventRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut start__ = None;
                let mut end__ = None;
                let mut timezone__ = None;
                let mut capacity__ = None;
                let mut slot_capacity__ = None;
                let mut recurrence_rule__ = None;
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
                    }
                }
                Ok(UpdateEventRequest {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    start: start__.unwrap_or_default(),
                    end: end__.unwrap_or_default(),
                    timezone: timezone__.unwrap_or_default(),
                    capacity: capacity__.unwrap_or_default(),
                    slot_capacity: slot_capacity__.unwrap_or_default(),
                    recurrence_rule: recurrence_rule__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("event.v1.UpdateEventRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateEventResponse {
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
        let mut struct_ser = serializer.serialize_struct("event.v1.UpdateEventResponse", len)?;
        if let Some(v) = self.event.as_ref() {
            struct_ser.serialize_field("event", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateEventResponse {
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
            type Value = UpdateEventResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct event.v1.UpdateEventResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateEventResponse, V::Error>
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
                Ok(UpdateEventResponse {
                    event: event__,
                })
            }
        }
        deserializer.deserialize_struct("event.v1.UpdateEventResponse", FIELDS, GeneratedVisitor)
    }
}
