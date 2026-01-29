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

export type TabType = 'dashboard' | 'products' | 'discounts' | 'settings' | 'form-builder' | 'preview' | 'transactions' | 'integration' | 'profile';

export interface IntegrationConfig {
  embedId: string;
  baseUrl: string;
  selectedProductIds: string[];
  widgetPosition: 'bottom-right' | 'bottom-left' | 'center';
  widgetButtonText: string;
  widgetButtonColor: string;
}
