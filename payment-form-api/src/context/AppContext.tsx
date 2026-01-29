import { createContext, useContext, useState, ReactNode, useEffect } from 'react';
import { v4 as uuidv4 } from 'uuid';
import { Product, DiscountCode, PaymentConfig, FormStyle, FormField, FormConfig, Transaction, TabType } from '../types';

interface AppContextType {
  activeTab: TabType;
  setActiveTab: (tab: TabType) => void;
  products: Product[];
  addProduct: (product: Omit<Product, 'id' | 'createdAt'>) => void;
  updateProduct: (id: string, product: Partial<Product>) => void;
  deleteProduct: (id: string) => void;
  discountCodes: DiscountCode[];
  addDiscountCode: (code: Omit<DiscountCode, 'id' | 'createdAt' | 'currentUses'>) => void;
  updateDiscountCode: (id: string, code: Partial<DiscountCode>) => void;
  deleteDiscountCode: (id: string) => void;
  paymentConfig: PaymentConfig;
  updatePaymentConfig: (config: Partial<PaymentConfig>) => void;
  formStyle: FormStyle;
  updateFormStyle: (style: Partial<FormStyle>) => void;
  formConfig: FormConfig;
  updateFormConfig: (config: Partial<FormConfig>) => void;
  updateFormField: (fieldId: string, updates: Partial<FormField>) => void;
  reorderFields: (startIndex: number, endIndex: number) => void;
  transactions: Transaction[];
  addTransaction: (transaction: Omit<Transaction, 'id' | 'createdAt'>) => void;
  selectedProduct: Product | null;
  setSelectedProduct: (product: Product | null) => void;
  validateDiscountCode: (code: string, productId: string) => DiscountCode | null;
  darkMode: boolean;
  toggleDarkMode: () => void;
}

const AppContext = createContext<AppContextType | undefined>(undefined);

const defaultPaymentConfig: PaymentConfig = {
  stripeEnabled: false,
  stripePublicKey: '',
  stripeSecretKey: '',
  paypalEnabled: false,
  paypalClientId: '',
  paypalClientSecret: '',
  paypalSandbox: true,
};

const defaultFormStyle: FormStyle = {
  primaryColor: '#6366f1',
  backgroundColor: '#ffffff',
  textColor: '#1f2937',
  borderRadius: 'lg',
  buttonStyle: 'solid',
  fontFamily: 'Inter',
  showLogo: true,
  logoUrl: '',
  companyName: 'Mi Empresa',
  showStripeButton: true,
  showPaypalButton: true,
  showCardButton: true,
  stripeButtonText: 'Pagar con Stripe',
  paypalButtonText: 'Pagar con PayPal',
  cardButtonText: 'Pagar con Tarjeta',
  sendInvoiceEmail: true,
  invoiceFromName: 'Mi Empresa',
  invoiceFromEmail: 'pagos@miempresa.com',
};

const defaultFormFields: FormField[] = [
  { id: 'name', type: 'text', label: 'Nombre completo', placeholder: 'Tu nombre', required: true, enabled: true, width: 'full', order: 0 },
  { id: 'email', type: 'email', label: 'Email', placeholder: 'tu@email.com', required: true, enabled: true, width: 'full', order: 1 },
  { id: 'phone', type: 'phone', label: 'Teléfono', placeholder: '+1 234 567 8900', required: false, enabled: false, width: 'half', order: 2 },
  { id: 'company', type: 'company', label: 'Empresa', placeholder: 'Nombre de tu empresa', required: false, enabled: false, width: 'half', order: 3 },
  { id: 'address', type: 'address', label: 'Dirección', placeholder: 'Calle y número', required: false, enabled: false, width: 'full', order: 4 },
  { id: 'city', type: 'city', label: 'Ciudad', placeholder: 'Tu ciudad', required: false, enabled: false, width: 'half', order: 5 },
  { id: 'country', type: 'country', label: 'País', placeholder: 'Tu país', required: false, enabled: false, width: 'half', order: 6 },
  { id: 'zip', type: 'zip', label: 'Código Postal', placeholder: '12345', required: false, enabled: false, width: 'half', order: 7 },
  { id: 'notes', type: 'notes', label: 'Notas adicionales', placeholder: 'Comentarios...', required: false, enabled: false, width: 'full', order: 8 },
];

const sampleProducts: Product[] = [];

const sampleDiscountCodes: DiscountCode[] = [];

const generateInvoiceId = () => `INV-${Date.now()}-${Math.random().toString(36).substr(2, 9).toUpperCase()}`;

// Helper to safely parse dates from JSON
const parseJSON = (key: string, value: any) => {
  if (value && typeof value === 'string' && (key.endsWith('At') || key === 'createdAt' || key === 'expiresAt')) {
    const date = new Date(value);
    if (!isNaN(date.getTime())) return date;
  }
  return value;
};

const loadState = <T,>(key: string, defaultValue: T): T => {
  if (typeof window === 'undefined') return defaultValue;
  try {
    const item = localStorage.getItem(`payform_${key}`);
    return item ? JSON.parse(item, parseJSON) : defaultValue;
  } catch (error) {
    console.error(`Error loading ${key} from localStorage`, error);
    return defaultValue;
  }
};


const sampleTransactions: Transaction[] = [];

export function AppProvider({ children }: { children: ReactNode }) {
  const [darkMode, setDarkMode] = useState(() => {
    if (typeof window !== 'undefined') {
      return localStorage.getItem('payform_darkMode') === 'true';
    }
    return false;
  });
  const [activeTab, setActiveTab] = useState<TabType>('dashboard');

  const [products, setProducts] = useState<Product[]>(() => {
    const loaded = loadState('products', sampleProducts);
    return Array.isArray(loaded) ? loaded : sampleProducts;
  });

  const [discountCodes, setDiscountCodes] = useState<DiscountCode[]>(() => {
    const loaded = loadState('discountCodes', sampleDiscountCodes);
    return Array.isArray(loaded) ? loaded : sampleDiscountCodes;
  });

  const [paymentConfig, setPaymentConfig] = useState<PaymentConfig>(() => {
    const loaded = loadState('paymentConfig', defaultPaymentConfig);
    return loaded && typeof loaded === 'object' ? loaded : defaultPaymentConfig;
  });

  const [formStyle, setFormStyle] = useState<FormStyle>(() => {
    const loaded = loadState('formStyle', defaultFormStyle);
    return loaded && typeof loaded === 'object' ? loaded : defaultFormStyle;
  });

  const [formConfig, setFormConfig] = useState<FormConfig>(() => {
    const loaded = loadState('formConfig', { fields: defaultFormFields });
    // Minimal validation to ensure fields array exists
    return loaded && typeof loaded === 'object' && Array.isArray((loaded as any).fields)
      ? loaded
      : { fields: defaultFormFields };
  });

  const [transactions, setTransactions] = useState<Transaction[]>(() => {
    const loaded = loadState('transactions', sampleTransactions);
    return Array.isArray(loaded) ? loaded : sampleTransactions;
  });

  const [selectedProduct, setSelectedProduct] = useState<Product | null>(null);

  useEffect(() => {
    localStorage.setItem('payform_products', JSON.stringify(products));
  }, [products]);

  useEffect(() => {
    localStorage.setItem('payform_discountCodes', JSON.stringify(discountCodes));
  }, [discountCodes]);

  useEffect(() => {
    localStorage.setItem('payform_paymentConfig', JSON.stringify(paymentConfig));
  }, [paymentConfig]);

  useEffect(() => {
    localStorage.setItem('payform_formStyle', JSON.stringify(formStyle));
  }, [formStyle]);

  useEffect(() => {
    localStorage.setItem('payform_formConfig', JSON.stringify(formConfig));
  }, [formConfig]);

  useEffect(() => {
    localStorage.setItem('payform_transactions', JSON.stringify(transactions));
  }, [transactions]);


  useEffect(() => {
    if (darkMode) {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
    localStorage.setItem('payform_darkMode', String(darkMode));
  }, [darkMode]);

  const toggleDarkMode = () => setDarkMode(!darkMode);

  const addProduct = (product: Omit<Product, 'id' | 'createdAt'>) => {
    const newProduct: Product = {
      ...product,
      id: uuidv4(),
      createdAt: new Date(),
    };
    setProducts([...products, newProduct]);
  };

  const updateProduct = (id: string, updates: Partial<Product>) => {
    setProducts(products.map(p => p.id === id ? { ...p, ...updates } : p));
  };

  const deleteProduct = (id: string) => {
    setProducts(products.filter(p => p.id !== id));
  };

  const addDiscountCode = (code: Omit<DiscountCode, 'id' | 'createdAt' | 'currentUses'>) => {
    const newCode: DiscountCode = {
      ...code,
      id: uuidv4(),
      currentUses: 0,
      createdAt: new Date(),
    };
    setDiscountCodes([...discountCodes, newCode]);
  };

  const updateDiscountCode = (id: string, updates: Partial<DiscountCode>) => {
    setDiscountCodes(discountCodes.map(c => c.id === id ? { ...c, ...updates } : c));
  };

  const deleteDiscountCode = (id: string) => {
    setDiscountCodes(discountCodes.filter(c => c.id !== id));
  };

  const updatePaymentConfig = (config: Partial<PaymentConfig>) => {
    setPaymentConfig({ ...paymentConfig, ...config });
  };

  const updateFormStyle = (style: Partial<FormStyle>) => {
    setFormStyle({ ...formStyle, ...style });
  };

  const updateFormConfig = (config: Partial<FormConfig>) => {
    setFormConfig({ ...formConfig, ...config });
  };

  const updateFormField = (fieldId: string, updates: Partial<FormField>) => {
    setFormConfig({
      ...formConfig,
      fields: formConfig.fields.map(f => f.id === fieldId ? { ...f, ...updates } : f)
    });
  };

  const reorderFields = (startIndex: number, endIndex: number) => {
    const result = Array.from(formConfig.fields);
    const [removed] = result.splice(startIndex, 1);
    result.splice(endIndex, 0, removed);

    const reorderedFields = result.map((field, index) => ({ ...field, order: index }));
    setFormConfig({ ...formConfig, fields: reorderedFields });
  };

  const addTransaction = (transaction: Omit<Transaction, 'id' | 'createdAt'>) => {
    const newTransaction: Transaction = {
      ...transaction,
      id: uuidv4(),
      createdAt: new Date(),
    };
    setTransactions([newTransaction, ...transactions]);
  };

  const validateDiscountCode = (code: string, productId: string): DiscountCode | null => {
    const discount = discountCodes.find(
      d => d.code.toLowerCase() === code.toLowerCase() && d.active
    );

    if (!discount) return null;

    if (discount.maxUses !== null && discount.currentUses >= discount.maxUses) return null;

    if (discount.expiresAt && new Date() > discount.expiresAt) return null;

    if (discount.applicableProducts.length > 0 && !discount.applicableProducts.includes(productId)) {
      return null;
    }

    return discount;
  };

  return (
    <AppContext.Provider value={{
      activeTab,
      setActiveTab,
      products,
      addProduct,
      updateProduct,
      deleteProduct,
      discountCodes,
      addDiscountCode,
      updateDiscountCode,
      deleteDiscountCode,
      paymentConfig,
      updatePaymentConfig,
      formStyle,
      updateFormStyle,
      formConfig,
      updateFormConfig,
      updateFormField,
      reorderFields,
      transactions,
      addTransaction,
      selectedProduct,
      setSelectedProduct,
      validateDiscountCode,
      darkMode,
      toggleDarkMode,
    }}>
      {children}
    </AppContext.Provider>
  );
}

export function useApp() {
  const context = useContext(AppContext);
  if (!context) {
    throw new Error('useApp must be used within AppProvider');
  }
  return context;
}
