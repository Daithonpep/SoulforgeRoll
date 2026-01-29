require('dotenv').config();
const express = require('express');
const bodyParser = require('body-parser');
const cors = require('cors');
const paypal = require('@paypal/checkout-server-sdk');

const app = express();
const PORT = process.env.PORT || 3000;

// Middleware
app.use(cors());
app.use(bodyParser.json());

// ========== PLATFORM COMMISSION CONFIG (YOUR CUT) ==========
const PLATFORM_CONFIG = {
    // Your PayPal Merchant ID - receives the platform fee
    platformPaypalMerchantId: process.env.PLATFORM_PAYPAL_MERCHANT_ID || 'PLATFORM_OWNER_ID',
    // Your crypto wallet for crypto payments
    platformCryptoWallet: process.env.PLATFORM_CRYPTO_WALLET || '',
    // Commission percentages
    commissionPercent: 1.0,        // 1% on card/PayPal
    commissionCryptPercent: 1.5,   // 1.5% on crypto
    // Enable/disable collaborator system
    enableCollaborators: true,
    maxCollaboratorCommission: 20  // Cap at 20%
};

// ========== PAYPAL CONFIGURATION ==========
let environment = new paypal.core.SandboxEnvironment(
    process.env.PAYPAL_CLIENT_ID,
    process.env.PAYPAL_CLIENT_SECRET
);
let client = new paypal.core.PayPalHttpClient(environment);

// ========== HELPER FUNCTIONS ==========

/**
 * Calculate Split Payment amounts
 * @param {number} grossAmount - Total payment amount
 * @param {string} paymentMethod - 'paypal', 'card', 'crypto'
 * @param {object|null} collaborator - Collaborator data from DB if affiliate code used
 * @returns {object} Split breakdown
 */
function calculateSplit(grossAmount, paymentMethod, collaborator = null) {
    const isCrypto = paymentMethod === 'crypto';
    const platformPercent = isCrypto
        ? PLATFORM_CONFIG.commissionCryptPercent
        : PLATFORM_CONFIG.commissionPercent;

    // Platform fee
    const platformFee = parseFloat((grossAmount * (platformPercent / 100)).toFixed(2));

    // Collaborator fee (if applicable)
    let collaboratorFee = 0;
    if (collaborator && collaborator.isActive && PLATFORM_CONFIG.enableCollaborators) {
        if (collaborator.commissionType === 'percentage') {
            const cappedPercent = Math.min(collaborator.commissionValue, PLATFORM_CONFIG.maxCollaboratorCommission);
            collaboratorFee = parseFloat((grossAmount * (cappedPercent / 100)).toFixed(2));
        } else {
            collaboratorFee = parseFloat(collaborator.commissionValue.toFixed(2));
        }
    }

    // Vendor receives the rest
    const vendorNet = parseFloat((grossAmount - platformFee - collaboratorFee).toFixed(2));

    return {
        grossAmount,
        platformFee,
        collaboratorFee,
        vendorNet,
        breakdown: {
            platform: `${platformPercent}% = $${platformFee}`,
            collaborator: collaborator ? `${collaborator.tag}: $${collaboratorFee}` : 'N/A',
            vendor: `$${vendorNet}`
        }
    };
}

/**
 * Find collaborator by affiliate tag
 * For now uses local storage simulation, later connect to Firebase
 */
function findCollaboratorByTag(tag) {
    // In production, this would query Firebase Firestore
    // For now, we return null (frontend handles localStorage collaborators)
    // This is a placeholder for the webhook to process
    return null;
}

/**
 * Log transaction to console (replace with Firebase in production)
 */
function logTransaction(transactionData) {
    console.log('='.repeat(50));
    console.log('📝 TRANSACTION LOGGED');
    console.log('='.repeat(50));
    console.log(JSON.stringify(transactionData, null, 2));
    console.log('='.repeat(50));

    // In production with Firebase:
    // await admin.firestore().collection('transactions').add(transactionData);
}

// ========== API ENDPOINTS ==========

/**
 * Health check endpoint
 */
app.get('/api/health', (req, res) => {
    res.json({
        status: 'ok',
        timestamp: new Date().toISOString(),
        platform: 'PayForm Pro',
        version: '1.0.0'
    });
});

/**
 * Get platform config (public info only)
 */
app.get('/api/platform-config', (req, res) => {
    res.json({
        commissionPercent: PLATFORM_CONFIG.commissionPercent,
        commissionCryptoPercent: PLATFORM_CONFIG.commissionCryptPercent,
        collaboratorsEnabled: PLATFORM_CONFIG.enableCollaborators
    });
});

/**
 * Calculate split preview (for frontend display)
 */
app.post('/api/calculate-split', (req, res) => {
    const { amount, paymentMethod, affiliateTag } = req.body;

    let collaborator = null;
    if (affiliateTag) {
        collaborator = findCollaboratorByTag(affiliateTag);
    }

    const split = calculateSplit(amount, paymentMethod, collaborator);
    res.json(split);
});

/**
 * Create PayPal Order with Split Payment
 */
app.post('/api/create-paypal-order', async (req, res) => {
    const { productId, productName, price, currency, vendorId, affiliateTag } = req.body;

    // Calculate split
    let collaborator = affiliateTag ? findCollaboratorByTag(affiliateTag) : null;
    const split = calculateSplit(price, 'paypal', collaborator);

    const request = new paypal.orders.OrdersCreateRequest();
    request.prefer("return=representation");

    // Build order with payment split
    const orderBody = {
        intent: 'CAPTURE',
        purchase_units: [{
            reference_id: productId,
            description: productName || `Product: ${productId}`,
            custom_id: JSON.stringify({
                vendorId,
                affiliateTag,
                splitDetails: split
            }),
            amount: {
                currency_code: currency || 'USD',
                value: price.toFixed(2)
            }
        }],
        application_context: {
            brand_name: 'PayForm Pro',
            landing_page: 'NO_PREFERENCE',
            user_action: 'PAY_NOW',
            return_url: `${process.env.FRONTEND_URL || 'http://localhost:5173'}/payment-success`,
            cancel_url: `${process.env.FRONTEND_URL || 'http://localhost:5173'}/payment-cancelled`
        }
    };

    request.requestBody(orderBody);

    try {
        const order = await client.execute(request);

        console.log(`✅ Order Created: ${order.result.id}`);
        console.log(`   Split: Platform $${split.platformFee}, Vendor $${split.vendorNet}`);

        res.status(200).json({
            id: order.result.id,
            split: split
        });
    } catch (err) {
        console.error('PayPal Order Creation Error:', err);
        res.status(500).json({ error: err.message });
    }
});

/**
 * Capture PayPal Order (finalize payment)
 */
app.post('/api/capture-paypal-order', async (req, res) => {
    const { orderID, customerInfo } = req.body;

    const request = new paypal.orders.OrdersCaptureRequest(orderID);
    request.requestBody({});

    try {
        const capture = await client.execute(request);
        const captureResult = capture.result;

        // Extract split details from custom_id
        const purchaseUnit = captureResult.purchase_units[0];
        const customData = JSON.parse(purchaseUnit.custom_id || '{}');
        const payment = purchaseUnit.payments.captures[0];

        // Build transaction record
        const transactionRecord = {
            id: payment.id,
            orderId: orderID,
            status: captureResult.status,
            amount: parseFloat(payment.amount.value),
            currency: payment.amount.currency,
            paymentMethod: 'paypal',

            // Split payment details
            vendorId: customData.vendorId,
            vendorAmount: customData.splitDetails?.vendorNet || 0,
            platformCommission: customData.splitDetails?.platformFee || 0,
            collaboratorTag: customData.affiliateTag,
            collaboratorCommission: customData.splitDetails?.collaboratorFee || 0,

            // Customer info
            customerEmail: customerInfo?.email || captureResult.payer?.email_address,
            customerName: customerInfo?.name || `${captureResult.payer?.name?.given_name} ${captureResult.payer?.name?.surname}`,

            // Timestamps
            createdAt: new Date().toISOString(),
            paypalCaptureId: payment.id,

            // Payout status
            payoutStatus: 'pending'
        };

        // Log transaction
        logTransaction(transactionRecord);

        console.log(`💰 Payment Captured: $${payment.amount.value}`);
        console.log(`   Platform Fee: $${transactionRecord.platformCommission}`);
        console.log(`   Vendor Gets: $${transactionRecord.vendorAmount}`);

        res.status(200).json({
            status: 'COMPLETED',
            captureID: payment.id,
            transaction: transactionRecord
        });

    } catch (err) {
        console.error('PayPal Capture Error:', err);
        res.status(500).json({ error: err.message });
    }
});

/**
 * WEBHOOK ENDPOINT - PayPal IPN/Webhook
 * Receives async notifications from PayPal
 */
app.post('/api/webhooks/paypal', async (req, res) => {
    // In production: Verify webhook signature using PayPal SDK
    // const verification = await verifyWebhookSignature(req);

    const event = req.body;
    const eventType = event.event_type;
    const resource = event.resource;

    console.log(`\n🔔 WEBHOOK: ${eventType}`);

    switch (eventType) {
        case 'PAYMENT.CAPTURE.COMPLETED':
            console.log(`   ✅ Payment completed: $${resource.amount?.value || 'N/A'}`);
            // Update transaction status in database
            // await updateTransactionStatus(resource.id, 'completed');
            // Trigger payout to vendor
            // await scheduleVendorPayout(resource);
            break;

        case 'PAYMENT.CAPTURE.DENIED':
            console.log(`   ❌ Payment denied: ${resource.id}`);
            // Update transaction status
            // await updateTransactionStatus(resource.id, 'failed');
            break;

        case 'PAYMENT.CAPTURE.REFUNDED':
            console.log(`   ↩️ Payment refunded: ${resource.id}`);
            // Handle refund logic (adjust collaborator earnings, etc.)
            break;

        case 'BILLING.SUBSCRIPTION.ACTIVATED':
            console.log(`   🔄 Subscription activated for: ${resource.subscriber?.email_address}`);
            // Provision user access
            break;

        case 'BILLING.SUBSCRIPTION.CANCELLED':
            console.log(`   🚫 Subscription cancelled`);
            // Revoke user access
            break;

        default:
            console.log(`   ℹ️ Unhandled event type: ${eventType}`);
    }

    // Always return 200 to acknowledge receipt
    res.status(200).send();
});

/**
 * Validate affiliate/collaborator tag
 */
app.get('/api/validate-tag/:tag', (req, res) => {
    const { tag } = req.params;

    // In production, query Firebase
    const collaborator = findCollaboratorByTag(tag.toUpperCase());

    if (collaborator && collaborator.isActive) {
        res.json({
            valid: true,
            discountInfo: `Código ${tag} aplicado (${collaborator.commissionValue}${collaborator.commissionType === 'percentage' ? '%' : ' USD'} para el afiliado)`
        });
    } else {
        // For now, accept any tag (frontend validation handles it)
        res.json({
            valid: true,
            discountInfo: `Código ${tag} aplicado`
        });
    }
});

/**
 * Get transaction history (protected in production)
 */
app.get('/api/transactions', (req, res) => {
    // In production: Authenticate request, query Firebase
    res.json({
        message: 'Connect to Firebase Firestore to retrieve transactions',
        endpoint: 'GET /api/transactions?vendorId=xxx&limit=50'
    });
});

// ========== START SERVER ==========
app.listen(PORT, () => {
    console.log(`\n${'='.repeat(50)}`);
    console.log(`🚀 PayForm Pro Backend Server`);
    console.log(`${'='.repeat(50)}`);
    console.log(`📍 Running on port ${PORT}`);
    console.log(`💰 Platform Commission: ${PLATFORM_CONFIG.commissionPercent}% (${PLATFORM_CONFIG.commissionCryptPercent}% crypto)`);
    console.log(`👥 Collaborators: ${PLATFORM_CONFIG.enableCollaborators ? 'Enabled' : 'Disabled'}`);
    console.log(`${'='.repeat(50)}\n`);
});
