import { useState, useEffect } from 'react';
import {
    Building2, CreditCard, Wallet, Plus, Trash2, Save,
    CheckCircle, AlertTriangle, Bitcoin, DollarSign
} from 'lucide-react';
import { useApp } from '../context/AppContext';
import { cn } from '../utils/cn';
import { BankAccount, CryptoWallet, VendorPaymentSettings } from '../types';

const CRYPTO_TYPES = [
    { value: 'binance_pay', label: 'Binance Pay', icon: '💰' },
    { value: 'coinbase', label: 'Coinbase Commerce', icon: '🪙' },
    { value: 'usdt_trc20', label: 'USDT (TRC20)', icon: '💵' },
    { value: 'usdt_erc20', label: 'USDT (ERC20)', icon: '💵' },
    { value: 'btc', label: 'Bitcoin', icon: '₿' },
    { value: 'eth', label: 'Ethereum', icon: 'Ξ' },
    { value: 'other', label: 'Otra', icon: '🔗' },
];

const COUNTRIES = [
    { code: 'MX', name: 'México', currency: 'MXN' },
    { code: 'US', name: 'Estados Unidos', currency: 'USD' },
    { code: 'ES', name: 'España', currency: 'EUR' },
    { code: 'CO', name: 'Colombia', currency: 'COP' },
    { code: 'AR', name: 'Argentina', currency: 'ARS' },
    { code: 'CL', name: 'Chile', currency: 'CLP' },
    { code: 'PE', name: 'Perú', currency: 'PEN' },
];

export function VendorSettings() {
    const { darkMode } = useApp();
    const [notification, setNotification] = useState<{ type: 'success' | 'error', message: string } | null>(null);

    // Load from localStorage or use defaults
    const [vendorSettings, setVendorSettings] = useState<VendorPaymentSettings>(() => {
        const saved = localStorage.getItem('payform_vendorSettings');
        return saved ? JSON.parse(saved) : {
            vendorId: crypto.randomUUID(),
            vendorName: '',
            vendorEmail: '',
            paypalMerchantId: '',
            stripeMerchantId: '',
            bankAccounts: [],
            cryptoWallets: [],
            preferredPayoutMethod: 'paypal',
            minPayoutAmount: 50,
            payoutCurrency: 'USD'
        };
    });

    const showNotification = (type: 'success' | 'error', message: string) => {
        setNotification({ type, message });
        setTimeout(() => setNotification(null), 3000);
    };

    const saveSettings = () => {
        localStorage.setItem('payform_vendorSettings', JSON.stringify(vendorSettings));
        showNotification('success', 'Configuración de pagos guardada correctamente');
    };

    // Bank Account Management
    const addBankAccount = () => {
        const newAccount: BankAccount = {
            id: crypto.randomUUID(),
            bankName: '',
            accountNumber: '',
            accountHolder: '',
            country: 'MX',
            currency: 'MXN',
            isPrimary: vendorSettings.bankAccounts.length === 0
        };
        setVendorSettings({
            ...vendorSettings,
            bankAccounts: [...vendorSettings.bankAccounts, newAccount]
        });
    };

    const updateBankAccount = (id: string, field: keyof BankAccount, value: string | boolean) => {
        setVendorSettings({
            ...vendorSettings,
            bankAccounts: vendorSettings.bankAccounts.map(acc =>
                acc.id === id ? { ...acc, [field]: value } : acc
            )
        });
    };

    const removeBankAccount = (id: string) => {
        setVendorSettings({
            ...vendorSettings,
            bankAccounts: vendorSettings.bankAccounts.filter(acc => acc.id !== id)
        });
    };

    const setPrimaryBank = (id: string) => {
        setVendorSettings({
            ...vendorSettings,
            bankAccounts: vendorSettings.bankAccounts.map(acc => ({
                ...acc,
                isPrimary: acc.id === id
            }))
        });
    };

    // Crypto Wallet Management
    const addCryptoWallet = () => {
        const newWallet: CryptoWallet = {
            id: crypto.randomUUID(),
            type: 'usdt_trc20',
            label: '',
            address: '',
            isPrimary: vendorSettings.cryptoWallets.length === 0
        };
        setVendorSettings({
            ...vendorSettings,
            cryptoWallets: [...vendorSettings.cryptoWallets, newWallet]
        });
    };

    const updateCryptoWallet = (id: string, field: keyof CryptoWallet, value: string | boolean) => {
        setVendorSettings({
            ...vendorSettings,
            cryptoWallets: vendorSettings.cryptoWallets.map(wallet =>
                wallet.id === id ? { ...wallet, [field]: value } : wallet
            )
        });
    };

    const removeCryptoWallet = (id: string) => {
        setVendorSettings({
            ...vendorSettings,
            cryptoWallets: vendorSettings.cryptoWallets.filter(w => w.id !== id)
        });
    };

    return (
        <div className={cn("p-6 max-w-5xl mx-auto", darkMode && "text-white")}>
            <div className="mb-8">
                <h2 className={cn("text-2xl font-bold", darkMode ? "text-white" : "text-slate-800")}>
                    Configuración de Pagos del Vendedor
                </h2>
                <p className={cn(darkMode ? "text-slate-400" : "text-slate-500")}>
                    Configura tus cuentas bancarias y billeteras para recibir pagos
                </p>
            </div>

            {notification && (
                <div className={cn(
                    "mb-6 p-4 rounded-lg flex items-center gap-3",
                    notification.type === 'success'
                        ? "bg-green-100 text-green-700 border border-green-200"
                        : "bg-red-100 text-red-700 border border-red-200"
                )}>
                    {notification.type === 'success' ? <CheckCircle size={20} /> : <AlertTriangle size={20} />}
                    {notification.message}
                </div>
            )}

            {/* Basic Vendor Info */}
            <div className={cn(
                "rounded-xl p-6 shadow-sm border mb-6",
                darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100"
            )}>
                <div className="flex items-center gap-3 mb-6">
                    <div className="w-12 h-12 bg-indigo-100 rounded-full flex items-center justify-center">
                        <Building2 className="text-indigo-600" size={24} />
                    </div>
                    <div>
                        <h3 className={cn("text-lg font-semibold", darkMode ? "text-white" : "text-slate-800")}>
                            Información del Vendedor
                        </h3>
                        <p className={cn("text-sm", darkMode ? "text-slate-400" : "text-slate-500")}>
                            Datos básicos para identificarte en el sistema
                        </p>
                    </div>
                </div>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div>
                        <label className={cn("block text-sm font-medium mb-1", darkMode ? "text-slate-300" : "text-slate-700")}>
                            Nombre del Negocio
                        </label>
                        <input
                            type="text"
                            value={vendorSettings.vendorName}
                            onChange={(e) => setVendorSettings({ ...vendorSettings, vendorName: e.target.value })}
                            placeholder="Mi Tienda Online"
                            className={cn(
                                "w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-indigo-500 outline-none",
                                darkMode ? "bg-slate-700 border-slate-600 text-white" : "bg-white border-slate-200"
                            )}
                        />
                    </div>
                    <div>
                        <label className={cn("block text-sm font-medium mb-1", darkMode ? "text-slate-300" : "text-slate-700")}>
                            Email de Contacto
                        </label>
                        <input
                            type="email"
                            value={vendorSettings.vendorEmail}
                            onChange={(e) => setVendorSettings({ ...vendorSettings, vendorEmail: e.target.value })}
                            placeholder="pagos@mitienda.com"
                            className={cn(
                                "w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-indigo-500 outline-none",
                                darkMode ? "bg-slate-700 border-slate-600 text-white" : "bg-white border-slate-200"
                            )}
                        />
                    </div>
                    <div>
                        <label className={cn("block text-sm font-medium mb-1", darkMode ? "text-slate-300" : "text-slate-700")}>
                            PayPal Merchant ID (opcional)
                        </label>
                        <input
                            type="text"
                            value={vendorSettings.paypalMerchantId || ''}
                            onChange={(e) => setVendorSettings({ ...vendorSettings, paypalMerchantId: e.target.value })}
                            placeholder="ABCD1234EFGH5678"
                            className={cn(
                                "w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-indigo-500 outline-none",
                                darkMode ? "bg-slate-700 border-slate-600 text-white" : "bg-white border-slate-200"
                            )}
                        />
                    </div>
                    <div>
                        <label className={cn("block text-sm font-medium mb-1", darkMode ? "text-slate-300" : "text-slate-700")}>
                            Método de Pago Preferido
                        </label>
                        <select
                            value={vendorSettings.preferredPayoutMethod}
                            onChange={(e) => setVendorSettings({ ...vendorSettings, preferredPayoutMethod: e.target.value as any })}
                            className={cn(
                                "w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-indigo-500 outline-none",
                                darkMode ? "bg-slate-700 border-slate-600 text-white" : "bg-white border-slate-200"
                            )}
                        >
                            <option value="paypal">PayPal</option>
                            <option value="stripe">Stripe</option>
                            <option value="bank">Transferencia Bancaria</option>
                            <option value="crypto">Criptomonedas</option>
                        </select>
                    </div>
                </div>
            </div>

            {/* Bank Accounts Section */}
            <div className={cn(
                "rounded-xl p-6 shadow-sm border mb-6",
                darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100"
            )}>
                <div className="flex items-center justify-between mb-6">
                    <div className="flex items-center gap-3">
                        <div className="w-12 h-12 bg-green-100 rounded-full flex items-center justify-center">
                            <CreditCard className="text-green-600" size={24} />
                        </div>
                        <div>
                            <h3 className={cn("text-lg font-semibold", darkMode ? "text-white" : "text-slate-800")}>
                                Cuentas Bancarias
                            </h3>
                            <p className={cn("text-sm", darkMode ? "text-slate-400" : "text-slate-500")}>
                                Agrega tus cuentas para recibir transferencias
                            </p>
                        </div>
                    </div>
                    <button
                        onClick={addBankAccount}
                        className="flex items-center gap-2 px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors"
                    >
                        <Plus size={18} />
                        Agregar Cuenta
                    </button>
                </div>

                {vendorSettings.bankAccounts.length === 0 ? (
                    <div className={cn("text-center py-8 border-2 border-dashed rounded-lg", darkMode ? "border-slate-700" : "border-slate-200")}>
                        <CreditCard className="mx-auto mb-2 text-slate-400" size={32} />
                        <p className="text-slate-500">No hay cuentas bancarias registradas</p>
                        <p className="text-sm text-slate-400">Haz clic en "Agregar Cuenta" para comenzar</p>
                    </div>
                ) : (
                    <div className="space-y-4">
                        {vendorSettings.bankAccounts.map((account, index) => (
                            <div
                                key={account.id}
                                className={cn(
                                    "p-4 rounded-lg border",
                                    account.isPrimary
                                        ? "border-green-500 bg-green-50 dark:bg-green-900/20"
                                        : darkMode ? "border-slate-700 bg-slate-700/50" : "border-slate-200 bg-slate-50"
                                )}
                            >
                                <div className="flex items-center justify-between mb-3">
                                    <span className={cn("font-medium", darkMode ? "text-white" : "text-slate-800")}>
                                        Cuenta #{index + 1} {account.isPrimary && <span className="text-green-600 text-sm ml-2">★ Principal</span>}
                                    </span>
                                    <div className="flex gap-2">
                                        {!account.isPrimary && (
                                            <button
                                                onClick={() => setPrimaryBank(account.id)}
                                                className="text-sm text-indigo-500 hover:underline"
                                            >
                                                Hacer Principal
                                            </button>
                                        )}
                                        <button
                                            onClick={() => removeBankAccount(account.id)}
                                            className="text-red-500 hover:text-red-700"
                                        >
                                            <Trash2 size={18} />
                                        </button>
                                    </div>
                                </div>

                                <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                                    <div>
                                        <label className="block text-xs font-medium text-slate-500 mb-1">Banco</label>
                                        <input
                                            type="text"
                                            value={account.bankName}
                                            onChange={(e) => updateBankAccount(account.id, 'bankName', e.target.value)}
                                            placeholder="BBVA, Santander, etc."
                                            className={cn(
                                                "w-full px-3 py-2 border rounded-lg text-sm focus:ring-2 focus:ring-indigo-500",
                                                darkMode ? "bg-slate-800 border-slate-600 text-white" : "bg-white border-slate-200"
                                            )}
                                        />
                                    </div>
                                    <div>
                                        <label className="block text-xs font-medium text-slate-500 mb-1">Titular de la Cuenta</label>
                                        <input
                                            type="text"
                                            value={account.accountHolder}
                                            onChange={(e) => updateBankAccount(account.id, 'accountHolder', e.target.value)}
                                            placeholder="Nombre completo"
                                            className={cn(
                                                "w-full px-3 py-2 border rounded-lg text-sm focus:ring-2 focus:ring-indigo-500",
                                                darkMode ? "bg-slate-800 border-slate-600 text-white" : "bg-white border-slate-200"
                                            )}
                                        />
                                    </div>
                                    <div>
                                        <label className="block text-xs font-medium text-slate-500 mb-1">País</label>
                                        <select
                                            value={account.country}
                                            onChange={(e) => {
                                                const country = COUNTRIES.find(c => c.code === e.target.value);
                                                updateBankAccount(account.id, 'country', e.target.value);
                                                if (country) updateBankAccount(account.id, 'currency', country.currency);
                                            }}
                                            className={cn(
                                                "w-full px-3 py-2 border rounded-lg text-sm focus:ring-2 focus:ring-indigo-500",
                                                darkMode ? "bg-slate-800 border-slate-600 text-white" : "bg-white border-slate-200"
                                            )}
                                        >
                                            {COUNTRIES.map(c => (
                                                <option key={c.code} value={c.code}>{c.name} ({c.currency})</option>
                                            ))}
                                        </select>
                                    </div>
                                    <div className="md:col-span-2">
                                        <label className="block text-xs font-medium text-slate-500 mb-1">
                                            {account.country === 'MX' ? 'CLABE Interbancaria (18 dígitos)' : 'Número de Cuenta'}
                                        </label>
                                        <input
                                            type="text"
                                            value={account.country === 'MX' ? (account.clabe || '') : account.accountNumber}
                                            onChange={(e) => updateBankAccount(account.id, account.country === 'MX' ? 'clabe' : 'accountNumber', e.target.value)}
                                            placeholder={account.country === 'MX' ? '012345678901234567' : 'Número de cuenta'}
                                            className={cn(
                                                "w-full px-3 py-2 border rounded-lg text-sm font-mono focus:ring-2 focus:ring-indigo-500",
                                                darkMode ? "bg-slate-800 border-slate-600 text-white" : "bg-white border-slate-200"
                                            )}
                                        />
                                    </div>
                                    {account.country === 'US' && (
                                        <div>
                                            <label className="block text-xs font-medium text-slate-500 mb-1">Routing Number</label>
                                            <input
                                                type="text"
                                                value={account.routingNumber || ''}
                                                onChange={(e) => updateBankAccount(account.id, 'routingNumber', e.target.value)}
                                                placeholder="9 dígitos"
                                                className={cn(
                                                    "w-full px-3 py-2 border rounded-lg text-sm font-mono focus:ring-2 focus:ring-indigo-500",
                                                    darkMode ? "bg-slate-800 border-slate-600 text-white" : "bg-white border-slate-200"
                                                )}
                                            />
                                        </div>
                                    )}
                                </div>
                            </div>
                        ))}
                    </div>
                )}
            </div>

            {/* Crypto Wallets Section */}
            <div className={cn(
                "rounded-xl p-6 shadow-sm border mb-6",
                darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100"
            )}>
                <div className="flex items-center justify-between mb-6">
                    <div className="flex items-center gap-3">
                        <div className="w-12 h-12 bg-orange-100 rounded-full flex items-center justify-center">
                            <Bitcoin className="text-orange-600" size={24} />
                        </div>
                        <div>
                            <h3 className={cn("text-lg font-semibold", darkMode ? "text-white" : "text-slate-800")}>
                                Billeteras de Criptomonedas
                            </h3>
                            <p className={cn("text-sm", darkMode ? "text-slate-400" : "text-slate-500")}>
                                Recibe pagos en USDT, BTC, ETH y más
                            </p>
                        </div>
                    </div>
                    <button
                        onClick={addCryptoWallet}
                        className="flex items-center gap-2 px-4 py-2 bg-orange-600 text-white rounded-lg hover:bg-orange-700 transition-colors"
                    >
                        <Plus size={18} />
                        Agregar Billetera
                    </button>
                </div>

                {vendorSettings.cryptoWallets.length === 0 ? (
                    <div className={cn("text-center py-8 border-2 border-dashed rounded-lg", darkMode ? "border-slate-700" : "border-slate-200")}>
                        <Wallet className="mx-auto mb-2 text-slate-400" size={32} />
                        <p className="text-slate-500">No hay billeteras de criptomonedas</p>
                        <p className="text-sm text-slate-400">Acepta pagos en cripto agregando tu dirección</p>
                    </div>
                ) : (
                    <div className="space-y-4">
                        {vendorSettings.cryptoWallets.map((wallet, index) => (
                            <div
                                key={wallet.id}
                                className={cn(
                                    "p-4 rounded-lg border",
                                    wallet.isPrimary
                                        ? "border-orange-500 bg-orange-50 dark:bg-orange-900/20"
                                        : darkMode ? "border-slate-700 bg-slate-700/50" : "border-slate-200 bg-slate-50"
                                )}
                            >
                                <div className="flex items-center justify-between mb-3">
                                    <span className={cn("font-medium", darkMode ? "text-white" : "text-slate-800")}>
                                        Billetera #{index + 1} {wallet.isPrimary && <span className="text-orange-600 text-sm ml-2">★ Principal</span>}
                                    </span>
                                    <button
                                        onClick={() => removeCryptoWallet(wallet.id)}
                                        className="text-red-500 hover:text-red-700"
                                    >
                                        <Trash2 size={18} />
                                    </button>
                                </div>

                                <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                                    <div>
                                        <label className="block text-xs font-medium text-slate-500 mb-1">Tipo</label>
                                        <select
                                            value={wallet.type}
                                            onChange={(e) => updateCryptoWallet(wallet.id, 'type', e.target.value)}
                                            className={cn(
                                                "w-full px-3 py-2 border rounded-lg text-sm focus:ring-2 focus:ring-indigo-500",
                                                darkMode ? "bg-slate-800 border-slate-600 text-white" : "bg-white border-slate-200"
                                            )}
                                        >
                                            {CRYPTO_TYPES.map(type => (
                                                <option key={type.value} value={type.value}>{type.icon} {type.label}</option>
                                            ))}
                                        </select>
                                    </div>
                                    <div>
                                        <label className="block text-xs font-medium text-slate-500 mb-1">Etiqueta (opcional)</label>
                                        <input
                                            type="text"
                                            value={wallet.label}
                                            onChange={(e) => updateCryptoWallet(wallet.id, 'label', e.target.value)}
                                            placeholder="Mi billetera principal"
                                            className={cn(
                                                "w-full px-3 py-2 border rounded-lg text-sm focus:ring-2 focus:ring-indigo-500",
                                                darkMode ? "bg-slate-800 border-slate-600 text-white" : "bg-white border-slate-200"
                                            )}
                                        />
                                    </div>
                                    {wallet.type === 'binance_pay' && (
                                        <div>
                                            <label className="block text-xs font-medium text-slate-500 mb-1">Binance Pay ID</label>
                                            <input
                                                type="text"
                                                value={wallet.binancePayId || ''}
                                                onChange={(e) => updateCryptoWallet(wallet.id, 'binancePayId', e.target.value)}
                                                placeholder="Tu Binance Pay ID"
                                                className={cn(
                                                    "w-full px-3 py-2 border rounded-lg text-sm focus:ring-2 focus:ring-indigo-500",
                                                    darkMode ? "bg-slate-800 border-slate-600 text-white" : "bg-white border-slate-200"
                                                )}
                                            />
                                        </div>
                                    )}
                                    {wallet.type === 'coinbase' && (
                                        <div>
                                            <label className="block text-xs font-medium text-slate-500 mb-1">Coinbase Commerce API Key</label>
                                            <input
                                                type="password"
                                                value={wallet.coinbaseCommerceApiKey || ''}
                                                onChange={(e) => updateCryptoWallet(wallet.id, 'coinbaseCommerceApiKey', e.target.value)}
                                                placeholder="••••••••••••"
                                                className={cn(
                                                    "w-full px-3 py-2 border rounded-lg text-sm focus:ring-2 focus:ring-indigo-500",
                                                    darkMode ? "bg-slate-800 border-slate-600 text-white" : "bg-white border-slate-200"
                                                )}
                                            />
                                        </div>
                                    )}
                                    <div className={wallet.type === 'binance_pay' || wallet.type === 'coinbase' ? 'md:col-span-3' : 'md:col-span-2'}>
                                        <label className="block text-xs font-medium text-slate-500 mb-1">
                                            Dirección de la Billetera
                                        </label>
                                        <input
                                            type="text"
                                            value={wallet.address}
                                            onChange={(e) => updateCryptoWallet(wallet.id, 'address', e.target.value)}
                                            placeholder={wallet.type.includes('usdt') ? 'T... o 0x...' : 'Dirección'}
                                            className={cn(
                                                "w-full px-3 py-2 border rounded-lg text-sm font-mono focus:ring-2 focus:ring-indigo-500",
                                                darkMode ? "bg-slate-800 border-slate-600 text-white" : "bg-white border-slate-200"
                                            )}
                                        />
                                    </div>
                                </div>
                            </div>
                        ))}
                    </div>
                )}
            </div>

            {/* Save Button */}
            <div className="flex justify-end">
                <button
                    onClick={saveSettings}
                    className="flex items-center gap-2 px-6 py-3 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors font-medium"
                >
                    <Save size={20} />
                    Guardar Configuración de Pagos
                </button>
            </div>
        </div>
    );
}
