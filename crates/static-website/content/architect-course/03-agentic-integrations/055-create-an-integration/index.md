# Creating your Own Integration

Run the following prompt in the console. 

```
Using the example below, create an Open API specification in JSON format for this API.

Make sure every operation has an operationid.

curl -X GET https://api.blockchain.com/v3/exchange/tickers \
  -H 'Accept: application/json' \
  -H 'X-API-Token: API_KEY'

Example Response

[
  {
    "symbol": "BTC-USD",
    "price_24h": "4998.0",
    "volume_24h": "0.3015",
    "last_trade_price": "5000.0"
  }
]
```

You'll get something like the below.

![alt text](generate-spec.png)

We can now add this specification to Bionics 

Then go to `Integrations > Select Integration > Add Custom` and cut and paste your Open API specification into the textarea.

![alt text](add-spec.png)

When you click `Submit` you should see a new Integration.

![alt text](show-integration.png)

And by connecting your integration similar to what we did with postgres you will be able to talk to that system.
