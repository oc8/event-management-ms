# Generated by the protocol buffer compiler.  DO NOT EDIT!
# sources: booking/v1/event.proto
# plugin: python-betterproto
from dataclasses import dataclass
from typing import List

import betterproto
import grpclib


class EventStatus(betterproto.Enum):
    EVENT_STATUS_UNSPECIFIED = 0
    EVENT_STATUS_ACTIVE = 1
    EVENT_STATUS_CANCELED = 2
    EVENT_STATUS_FULL = 3


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
    max_guests: int = betterproto.int32_field(12)
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
    max_guests: int = betterproto.int32_field(5)
    created_at: int = betterproto.int64_field(6)
    updated_at: int = betterproto.int64_field(7)


@dataclass
class Booking(betterproto.Message):
    id: str = betterproto.string_field(1)
    first_name: str = betterproto.string_field(2)
    last_name: str = betterproto.string_field(3)
    booking_holder_key: str = betterproto.string_field(4)
    slot_id: str = betterproto.string_field(5)
    slot: "Slot" = betterproto.message_field(6)
    number_of_people: int = betterproto.int32_field(7)
    message: str = betterproto.string_field(8)
    created_at: int = betterproto.int64_field(9)
    updated_at: int = betterproto.int64_field(10)


@dataclass
class ClosingException(betterproto.Message):
    id: str = betterproto.string_field(1)
    closing_from: "TimeData" = betterproto.message_field(4)
    closing_to: "TimeData" = betterproto.message_field(5)
    reason: str = betterproto.string_field(6)
    created_at: int = betterproto.int64_field(7)
    updated_at: int = betterproto.int64_field(8)


@dataclass
class CreateEventRequest(betterproto.Message):
    name: str = betterproto.string_field(1)
    start: str = betterproto.string_field(2)
    end: str = betterproto.string_field(3)
    timezone: str = betterproto.string_field(4)
    organizer_key: str = betterproto.string_field(5)
    slot_duration: int = betterproto.int64_field(6)
    max_guests: int = betterproto.int32_field(7)
    recurrence_rule: str = betterproto.string_field(8)
    event_type: "EventType" = betterproto.enum_field(9)


@dataclass
class CreateEventResponse(betterproto.Message):
    event: "Event" = betterproto.message_field(1)


@dataclass
class CreateEventsRequest(betterproto.Message):
    events: List["CreateEventRequest"] = betterproto.message_field(1)


@dataclass
class CreateEventsResponse(betterproto.Message):
    events: List["Event"] = betterproto.message_field(1)


@dataclass
class GetEventRequest(betterproto.Message):
    id: str = betterproto.string_field(1)


@dataclass
class GetEventResponse(betterproto.Message):
    event: "Event" = betterproto.message_field(1)


@dataclass
class GetActiveEventsRequest(betterproto.Message):
    organizer_key: str = betterproto.string_field(1)


@dataclass
class GetActiveEventsResponse(betterproto.Message):
    events: List["Event"] = betterproto.message_field(1)


@dataclass
class CreateBookingRequest(betterproto.Message):
    first_name: str = betterproto.string_field(1)
    last_name: str = betterproto.string_field(2)
    booking_holder_key: str = betterproto.string_field(3)
    slot_id: str = betterproto.string_field(4)
    number_of_people: int = betterproto.int32_field(5)
    message: str = betterproto.string_field(6)


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
class CreateClosingExceptionRequest(betterproto.Message):
    closing_from: str = betterproto.string_field(1)
    closing_to: str = betterproto.string_field(2)
    reason: str = betterproto.string_field(3)


@dataclass
class CreateClosingExceptionResponse(betterproto.Message):
    exception: "ClosingException" = betterproto.message_field(1)


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
        max_guests: int = 0,
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
        request.max_guests = max_guests
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

    async def get_active_events(
        self, *, organizer_key: str = ""
    ) -> GetActiveEventsResponse:
        request = GetActiveEventsRequest()
        request.organizer_key = organizer_key

        return await self._unary_unary(
            "/booking.v1.BookingService/GetActiveEvents",
            request,
            GetActiveEventsResponse,
        )

    async def create_booking(
        self,
        *,
        first_name: str = "",
        last_name: str = "",
        booking_holder_key: str = "",
        slot_id: str = "",
        number_of_people: int = 0,
        message: str = "",
    ) -> CreateBookingResponse:
        request = CreateBookingRequest()
        request.first_name = first_name
        request.last_name = last_name
        request.booking_holder_key = booking_holder_key
        request.slot_id = slot_id
        request.number_of_people = number_of_people
        request.message = message

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

    async def create_closing_exception(
        self, *, closing_from: str = "", closing_to: str = "", reason: str = ""
    ) -> CreateClosingExceptionResponse:
        request = CreateClosingExceptionRequest()
        request.closing_from = closing_from
        request.closing_to = closing_to
        request.reason = reason

        return await self._unary_unary(
            "/booking.v1.BookingService/CreateClosingException",
            request,
            CreateClosingExceptionResponse,
        )
