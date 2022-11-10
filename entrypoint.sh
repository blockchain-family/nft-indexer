#!/bin/bash

sqlx migrate && /app/application $1
