#!/bin/bash
set -e

USER_ID=ilerik.testnet

# Views
curl -v http://localhost:8080/version
curl -v http://localhost:8080/status
curl -v http://localhost:8080/rewards?nearid=ilerik.testnet

# Checkin and balance
curl -v "http://localhost:8080/checkin?nearid='ilerik.testnet'&qr='http://2'"
curl -v http://localhost:8080/rewards?nearid=ilerik.testnet

