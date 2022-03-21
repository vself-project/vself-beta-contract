#!/bin/bash
set -e

USER_ID=ilerik.testnet

# Views
curl http://localhost:8080/version && echo ""
curl http://localhost:8080/status && echo ""
curl http://localhost:8080/rewards && echo ""
curl "http://localhost:8080/rewards?nearid='ilerik.testnet'" && echo ""

# Checkin and balance
curl "http://localhost:8080/checkin?nearid='ilerik.testnet'&qr='https://2'" && echo ""
curl "http://localhost:8080/rewards?nearid='ilerik.testnet'" && echo ""

# Check that account is valid
curl "http://localhost:8080/check-account?nearid=ilerik.testnet" && echo ""

