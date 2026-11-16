#!/bin/bash
# northgate-subscribe.sh — northGate subscribes to eastGate heart-pull
# Deploy on northGate: crontab -e → */5 * * * * bash /path/to/northgate-subscribe.sh
# The HEART pulls from the BRAIN every 5 minutes

EAST="http://192.168.4.244:8080"
LOCAL="/opt/gaia-data"
mkdir -p "$LOCAL"

# Pull heart data from eastGate (the brain)
curl -sf "$EAST/data/heart-pull.json" -o "$LOCAL/heart-pull.json" 2>/dev/null
curl -sf "$EAST/data/now.json" -o "$LOCAL/now.json" 2>/dev/null
curl -sf "$EAST/data/body/piton-map.json" -o "$LOCAL/piton-map.json" 2>/dev/null
curl -sf "$EAST/data/polaris-push.json" -o "$LOCAL/polaris-push.json" 2>/dev/null
curl -sf "$EAST/data/organism-fabric.json" -o "$LOCAL/organism-fabric.json" 2>/dev/null

# Phone home — tell eastGate the heart is beating
MODEL_COUNT=$(curl -sf http://localhost:11434/api/tags | python3 -c 'import json,sys;print(len(json.load(sys.stdin).get("models",[])))' 2>/dev/null || echo 0)
GPU_INFO=$(nvidia-smi --query-gpu=memory.used,memory.total,temperature.gpu --format=csv,noheader 2>/dev/null || echo "unknown")

curl -sf -X POST "$EAST/data/northgate-heartbeat.json" \
  -H "Content-Type: application/json" \
  -d "{
    \"gate\": \"northGate\",
    \"role\": \"HEART\",
    \"ts\": \"$(date -Is)\",
    \"models\": $MODEL_COUNT,
    \"gpu\": \"$GPU_INFO\",
    \"uptime\": \"$(uptime -p)\",
    \"load\": \"$(cat /proc/loadavg | cut -d' ' -f1-3)\"
  }" 2>/dev/null

echo "$(date -Is) subscribed (${MODEL_COUNT} models)" >> /tmp/northgate-subscribe.log
