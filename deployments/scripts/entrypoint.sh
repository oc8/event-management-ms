#!/bin/bash
sleep 5
sqlx migrate run
./event-ms