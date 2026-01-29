export interface Product {
  id: string;
  name: string;
  description: string;
  price: number;
  currency: string;
  type: 'one-time' | 'subscription';
  interval?: 'monthly' | 'yearly' | 'weekly';
  active: boolean;
  createdAt: Date;
}

export interface DiscountCode {
  id: string;
  code: string;
  creatorName: string;
  discountType: 'percentage' | 'fixed';
  discountValue: number;
  maxUses: number | null;
  currentUses: number;
  expiresAt: Date | null;
  active: boolean;
  applicableProducts: string[];
  createdAt: Date;
}

export interface PaymentConfig {
  stripeEnabled: boolean;
  stripePublicKey: string;
  stripeSecretKey: string;
  paypalEnabled: boolean;
  paypalClientId: string;
  paypalClientSecret: string;
  paypalSandbox: boolean;
}

export interface FormField {
  id: string;
  type: 'text' | 'email' | 'phone' | 'address' | 'city' | 'country' | 'zip' | 'company' | 'notes' | 'custom';
  label: string;
  placeholder: string;
  required: boolean;
  enabled: boolean;
  width: 'full' | 'half';
  order: number;
}

export interface FormStyle {
  primaryColor: string;
  backgroundColor: string;
  textColor: string;
  borderRadius: 'none' | 'sm' | 'md' | 'lg' | 'xl' | 'full';
  buttonStyle: 'solid' | 'outline' | 'gradient';
  fontFamily: string;
  showLogo: boolean;
  logoUrl: string;
  companyName: string;
  showStripeButton: boolean;
  showPaypalButton: boolean;
  showCardButton: boolean;
  stripeButtonText: string;
  paypalButtonText: string;
  cardButtonText: string;
  sendInvoiceEmail: boolean;
  invoiceFromName: string;
  invoiceFromEmail: string;
}

export interface FormConfig {
  fields: FormField[];
}

export interface Transaction {
  id: string;
  productId: string;
  productName: string;
  amount: number;
  originalAmount: number;
  currency: string;
  discountCode?: string;
  discountAmount?: number;
  paymentMethod: 'stripe' | 'paypal' | 'card';
  status: 'completed' | 'pending' | 'failed' | 'refunded';
  customerEmail: string;
  customerName: string;
  customerPhone?: string;
  customerAddress?: string;
  customerCity?: string;
  customerCountry?: string;
  customerZip?: string;
  customerCompany?: string;
  customerNotes?: string;
  invoiceId: string;
  createdAt: Date;
}

export type TabType = 'dashboard' | 'products' | 'discounts' | 'settings' | 'form-builder' | 'preview' | 'transactions' | 'integration' | 'profile' | 'vendor-settings' | 'collaborators';

export interface IntegrationConfig {
  embedId: string;
  baseUrl: string;
  selectedProductIds: string[];
  widgetPosition: 'bottom-right' | 'bottom-left' | 'center';
  widgetButtonText: string;
  widgetButtonColor: string;
}

// ========== SPLIT PAYMENTS & VENDOR SYSTEM ==========

export interface BankAccount {
  id: string;
  bankName: string;
  accountNumber: string;
  accountHolder: string;
  routingNumber?: string; // For US banks
  clabe?: string; // For Mexican banks
  swiftCode?: string; // For international
  country: string;
  currency: string;
  isPrimary: boolean;
}

export interface CryptoWallet {
  id: string;
  type: 'binance_pay' | 'coinbase' | 'usdt_trc20' | 'usdt_erc20' | 'btc' | 'eth' | 'other';
  label: string;
  address: string;
  network?: string; // e.g., "TRC20", "ERC20", "BEP20"
  binancePayId?: string;
  coinbaseCommerceApiKey?: string;
  isPrimary: boolean;
}

export interface VendorPaymentSettings {
  vendorId: string;
  vendorName: string;
  vendorEmail: string;
  paypalMerchantId?: string;
  stripeMerchantId?: string;
  bankAccounts: BankAccount[];
  cryptoWallets: CryptoWallet[];
  preferredPayoutMethod: 'paypal' | 'stripe' | 'bank' | 'crypto';
  minPayoutAmount: number;
  payoutCurrency: string;
}

export interface Collaborator {
  id: string;
  name: string;
  email: string;
  tag: string; // e.g., "JUAN20" - the affiliate code
  commissionType: 'percentage' | 'fixed';
  commissionValue: number; // e.g., 10 for 10% or 5 for $5
  totalEarnings: number;
  pendingPayout: number;
  payoutMethod: 'paypal' | 'bank' | 'crypto';
  paypalEmail?: string;
  bankAccount?: BankAccount;
  cryptoWallet?: CryptoWallet;
  isActive: boolean;
  createdAt: Date;
  referrals: number; // Total number of sales made with this tag
}

export interface SplitPaymentConfig {
  // Platform owner commission (your cut)
  platformCommissionPercent: number; // e.g., 1 for 1%
  platformCommissionCryptoPercent: number; // e.g., 1.5 for crypto
  platformPaypalMerchantId: string;
  platformStripeMerchantId: string;
  platformCryptoWallet: string;

  // Split settings
  enableCollaboratorSplit: boolean;
  maxCollaboratorCommission: number; // Cap for affiliate commission
}

// Extended Transaction with split payment info
export interface TransactionWithSplit extends Transaction {
  vendorId?: string;
  vendorAmount?: number;
  platformCommission?: number;
  collaboratorId?: string;
  collaboratorTag?: string;
  collaboratorCommission?: number;
  splitDetails?: {
    grossAmount: number;
    platformFee: number;
    collaboratorFee: number;
    vendorNet: number;
    payoutStatus: 'pending' | 'processing' | 'completed' | 'failed';
  };
}
