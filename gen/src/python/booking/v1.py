# Generated by the protocol buffer compiler.  DO NOT EDIT!
# sources: booking/v1/event.proto
# plugin: python-betterproto
from dataclasses import dataclass
from typing import List, Optional

import betterproto
import grpclib


class EventStatus(betterproto.Enum):
    EVENT_STATUS_UNSPECIFIED = 0
    EVENT_STATUS_ACTIVE = 1
    EVENT_STATUS_CANCELED = 2
    EVENT_STATUS_FULL = 3
    EVENT_STATUS_DISABLE = 4


class EventType(betterproto.Enum):
    EVENT_TYPE_UNSPECIFIED = 0
    EVENT_TYPE_EVENT = 1
    EVENT_TYPE_TASK = 2
    EVENT_TYPE_MEETING = 3


@dataclass
class TimeData(betterproto.Message):
    timezone: str = betterproto.string_field(1)
    date_time: str = betterproto.string_field(2)


@dataclass
class Event(betterproto.Message):
    id: str = betterproto.string_field(1)
    name: str = betterproto.string_field(2)
    event_type: "EventType" = betterproto.enum_field(3)
    status: "EventStatus" = betterproto.enum_field(4)
    start: "TimeData" = betterproto.message_field(5)
    end: "TimeData" = betterproto.message_field(6)
    recurrence_rule: str = betterproto.string_field(7)
    organizer_key: str = betterproto.string_field(8)
    cancellation: "Cancellation" = betterproto.message_field(9)
    slots: List["Slot"] = betterproto.message_field(10)
    slot_duration: int = betterproto.int64_field(11)
    capacity: int = betterproto.int32_field(12)
    created_at: int = betterproto.int64_field(13)
    updated_at: int = betterproto.int64_field(14)


@dataclass
class Cancellation(betterproto.Message):
    canceled_by: str = betterproto.string_field(1)
    reason: str = betterproto.string_field(2)
    created_at: "TimeData" = betterproto.message_field(3)


@dataclass
class Slot(betterproto.Message):
    id: str = betterproto.string_field(1)
    event_id: str = betterproto.string_field(2)
    start: "TimeData" = betterproto.message_field(3)
    end: "TimeData" = betterproto.message_field(4)
    capacity: int = betterproto.int32_field(5)
    created_at: int = betterproto.int64_field(6)
    updated_at: int = betterproto.int64_field(7)


@dataclass
class Booking(betterproto.Message):
    id: str = betterproto.string_field(1)
    booking_holder_key: str = betterproto.string_field(2)
    slot_id: str = betterproto.string_field(3)
    slot: "Slot" = betterproto.message_field(4)
    date_time: "TimeData" = betterproto.message_field(5)
    persons: int = betterproto.int32_field(6)
    created_at: int = betterproto.int64_field(7)
    updated_at: int = betterproto.int64_field(8)


@dataclass
class Closure(betterproto.Message):
    id: str = betterproto.string_field(1)
    closing_from: "TimeData" = betterproto.message_field(2)
    closing_to: "TimeData" = betterproto.message_field(3)
    organizer_key: str = betterproto.string_field(5)
    created_at: int = betterproto.int64_field(6)
    updated_at: int = betterproto.int64_field(7)


@dataclass
class Filters(betterproto.Message):
    from_: str = betterproto.string_field(1)
    to: str = betterproto.string_field(2)
    organizer_key: str = betterproto.string_field(3)
    status: "EventStatus" = betterproto.enum_field(4)
    event_type: "EventType" = betterproto.enum_field(5)
    limit: int = betterproto.int64_field(6)
    offset: int = betterproto.int64_field(7)


@dataclass
class CreateEventRequest(betterproto.Message):
    name: str = betterproto.string_field(1)
    start: str = betterproto.string_field(2)
    end: str = betterproto.string_field(3)
    timezone: str = betterproto.string_field(4)
    organizer_key: str = betterproto.string_field(5)
    slot_duration: int = betterproto.int64_field(6)
    capacity: int = betterproto.int32_field(7)
    slot_capacity: int = betterproto.int32_field(8)
    recurrence_rule: str = betterproto.string_field(9)
    event_type: "EventType" = betterproto.enum_field(10)


@dataclass
class CreateEventResponse(betterproto.Message):
    event: "Event" = betterproto.message_field(1)


@dataclass
class GetEventRequest(betterproto.Message):
    id: str = betterproto.string_field(1)


@dataclass
class GetEventResponse(betterproto.Message):
    event: "Event" = betterproto.message_field(1)


@dataclass
class ListEventsRequest(betterproto.Message):
    filters: "Filters" = betterproto.message_field(1)


@dataclass
class ListEventsResponse(betterproto.Message):
    events: List["Event"] = betterproto.message_field(1)


@dataclass
class CreateBookingRequest(betterproto.Message):
    booking_holder_key: str = betterproto.string_field(1)
    slot_id: str = betterproto.string_field(2)
    date_time: str = betterproto.string_field(3)
    persons: int = betterproto.int32_field(4)


@dataclass
class CreateBookingResponse(betterproto.Message):
    booking: "Booking" = betterproto.message_field(1)


@dataclass
class GetBookingRequest(betterproto.Message):
    id: str = betterproto.string_field(1)


@dataclass
class GetBookingResponse(betterproto.Message):
    booking: "Booking" = betterproto.message_field(1)


@dataclass
class CreateClosureRequest(betterproto.Message):
    closing_from: str = betterproto.string_field(1)
    closing_to: str = betterproto.string_field(2)
    organizer_key: str = betterproto.string_field(3)


@dataclass
class CreateClosureResponse(betterproto.Message):
    closure: "Closure" = betterproto.message_field(1)


@dataclass
class GetTimelineRequest(betterproto.Message):
    organizer_key: str = betterproto.string_field(1)
    from_: str = betterproto.string_field(2)
    to: str = betterproto.string_field(3)


@dataclass
class GetTimelineResponse(betterproto.Message):
    events: List["Event"] = betterproto.message_field(1)
    closures: List["Closure"] = betterproto.message_field(2)


class BookingServiceStub(betterproto.ServiceStub):
    async def create_event(
        self,
        *,
        name: str = "",
        start: str = "",
        end: str = "",
        timezone: str = "",
        organizer_key: str = "",
        slot_duration: int = 0,
        capacity: int = 0,
        slot_capacity: int = 0,
        recurrence_rule: str = "",
        event_type: "EventType" = 0,
    ) -> CreateEventResponse:
        request = CreateEventRequest()
        request.name = name
        request.start = start
        request.end = end
        request.timezone = timezone
        request.organizer_key = organizer_key
        request.slot_duration = slot_duration
        request.capacity = capacity
        request.slot_capacity = slot_capacity
        request.recurrence_rule = recurrence_rule
        request.event_type = event_type

        return await self._unary_unary(
            "/booking.v1.BookingService/CreateEvent",
            request,
            CreateEventResponse,
        )

    async def get_event(self, *, id: str = "") -> GetEventResponse:
        request = GetEventRequest()
        request.id = id

        return await self._unary_unary(
            "/booking.v1.BookingService/GetEvent",
            request,
            GetEventResponse,
        )

    async def list_events(
        self, *, filters: Optional["Filters"] = None
    ) -> ListEventsResponse:
        request = ListEventsRequest()
        if filters is not None:
            request.filters = filters

        return await self._unary_unary(
            "/booking.v1.BookingService/ListEvents",
            request,
            ListEventsResponse,
        )

    async def create_booking(
        self,
        *,
        booking_holder_key: str = "",
        slot_id: str = "",
        date_time: str = "",
        persons: int = 0,
    ) -> CreateBookingResponse:
        request = CreateBookingRequest()
        request.booking_holder_key = booking_holder_key
        request.slot_id = slot_id
        request.date_time = date_time
        request.persons = persons

        return await self._unary_unary(
            "/booking.v1.BookingService/CreateBooking",
            request,
            CreateBookingResponse,
        )

    async def get_booking(self, *, id: str = "") -> GetBookingResponse:
        request = GetBookingRequest()
        request.id = id

        return await self._unary_unary(
            "/booking.v1.BookingService/GetBooking",
            request,
            GetBookingResponse,
        )

    async def create_closure(
        self, *, closing_from: str = "", closing_to: str = "", organizer_key: str = ""
    ) -> CreateClosureResponse:
        request = CreateClosureRequest()
        request.closing_from = closing_from
        request.closing_to = closing_to
        request.organizer_key = organizer_key

        return await self._unary_unary(
            "/booking.v1.BookingService/CreateClosure",
            request,
            CreateClosureResponse,
        )

    async def get_timeline(
        self, *, organizer_key: str = "", from_: str = "", to: str = ""
    ) -> GetTimelineResponse:
        request = GetTimelineRequest()
        request.organizer_key = organizer_key
        request.from_ = from_
        request.to = to

        return await self._unary_unary(
            "/booking.v1.BookingService/GetTimeline",
            request,
            GetTimelineResponse,
        )
