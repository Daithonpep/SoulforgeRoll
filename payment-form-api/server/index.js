require('dotenv').config();
const express = require('express');
const bodyParser = require('body-parser');
const cors = require('cors');
const paypal = require('@paypal/checkout-server-sdk');

const app = express();
const PORT = process.env.PORT || 3000;

// Middleware
app.use(cors());
// PayPal webhooks send JSON
app.use(bodyParser.json());

// PayPal Environment Configuration
let environment = new paypal.core.SandboxEnvironment(
    process.env.PAYPAL_CLIENT_ID,
    process.env.PAYPAL_CLIENT_SECRET
);
let client = new paypal.core.PayPalHttpClient(environment);

/**
 * Endpoint to Create an Order
 * Frontend calls this to get an orderID
 */
app.post('/api/create-paypal-order', async (req, res) => {
    const { productId, price, currency } = req.body;

    const request = new paypal.orders.OrdersCreateRequest();
    request.prefer("return=representation");
    request.requestBody({
        intent: 'CAPTURE',
        purchase_units: [{
            amount: {
                currency_code: currency || 'USD',
                value: price
            },
            description: `Product ID: ${productId}`
        }]
    });

    try {
        const order = await client.execute(request);
        res.status(200).json({
            id: order.result.id
        });
    } catch (err) {
        console.error(err);
        res.status(500).send(err.message);
    }
});

/**
 * Endpoint to Capture an Order
 * Frontend calls this after user approves payment
 */
app.post('/api/capture-paypal-order', async (req, res) => {
    const { orderID } = req.body;

    const request = new paypal.orders.OrdersCaptureRequest(orderID);
    request.requestBody({});

    try {
        const capture = await client.execute(request);
        // Here you would:
        // 1. Verify the payment amount matches your database
        // 2. Provision the user's account (e.g. set plan='premium')
        // 3. Send confirmation email via SendGrid, etc.

        const captureID = capture.result.purchase_units[0].payments.captures[0].id;
        console.log(`Order ${orderID} captured successfully. Capture ID: ${captureID}`);

        res.status(200).json({
            status: 'COMPLETED',
            captureID: captureID,
            result: capture.result
        });
    } catch (err) {
        console.error(err);
        res.status(500).send(err.message);
    }
});

/**
 * WEBHOOK ENDPOINT
 * PayPal calls this automatically when events happen (async)
 */
app.post('/api/webhooks/paypal', async (req, res) => {
    // 1. Verify webhook signature (Crucial for security in prod)
    // For MVP/Sandbox we will log the event.

    const event = req.body;
    const eventType = event.event_type;

    console.log(`Received Webhook Event: ${eventType}`);

    // Handle specific events
    switch (eventType) {
        case 'PAYMENT.CAPTURE.COMPLETED':
            // Payment was successful
            const resource = event.resource;
            const amount = resource.amount.value;
            const customId = resource.custom_id; // Could be userId
            console.log(`💰 Payment Received: $${amount} USD`);
            // TODO: Update user DB status to 'paid'
            break;

        case 'BILLING.SUBSCRIPTION.CANCELLED':
            console.log('User cancelled subscription');
            // TODO: Downgrade user
            break;

        case 'PAYMENT.CAPTURE.DENIED':
            console.log('Payment denied');
            break;
    }

    // Always return 200 OK to PayPal so they stop retrying
    res.status(200).send();
});

app.listen(PORT, () => {
    console.log(`Server running on port ${PORT}`);
});
