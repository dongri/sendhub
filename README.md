# Sendhub
sendhub provides api to send emails and slack webhook.

![sendhub](https://github.com/dongri/images/blob/master/sendhub.png?raw=true "sendhub")

# Email Providers
* amazon ses
* mailgun
* sendgrid

# Change Email Primary Privider
```
curl -XPOST http://localhost:3000/api/email/settings \
-H 'Content-Type:application/json' \
-d '{"primary": "mailgun"}'
```

# Setting Load Balancing
```
curl -XPOST http://localhost:3000/api/email/settings \
-H 'Content-Type:application/json' \
-d '{"load_balancing": true}'
```

# Send Email And Slack Message
```
curl -XPOST http://localhost:3000/api/email/raw \
-H 'Content-Type:application/json' \
-d '{"from": "no-reply@hackerth.com", "to": "dongrify@gmail.com", "subject": "hello subject", "html": "<h1>mail</h1>"}'
```

```
curl -XPOST http://localhost:3000/api/email/template \
-H 'Content-Type:application/json' \
-d @- << EOF
{
  "from": "no-reply@hackerth.com",
  "to": "dongrify@gmail.com",
  "subject": "hello subject",
  "template": "hello",
  "values": {
    "message": "hello sendhub"
  }
}
EOF
```

```
curl -XPOST http://localhost:3000/api/slack/message \
-H 'Content-Type:application/json' \
-d '{"channel": "monitor", "text": "system error"}'
```

# Edit template
`/templates/` Edit Email templates
