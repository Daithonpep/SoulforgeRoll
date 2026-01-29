import { useState, useRef, useEffect } from 'react';
import { PayPalScriptProvider, PayPalButtons } from "@paypal/react-paypal-js";
import { Check, AlertCircle, CreditCard, Lock, ChevronDown, X, FileText, Mail, Download } from 'lucide-react';
import { useApp } from '../context/AppContext';
import { cn } from '../utils/cn';

export function Preview() {
  const {
    products, formStyle, formConfig, validateDiscountCode, addTransaction,
    discountCodes, updateDiscountCode, darkMode, paymentConfig
  } = useApp();

  const [selectedProductId, setSelectedProductId] = useState(products[0]?.id || '');
  const [discountCodeInput, setDiscountCodeInput] = useState('');
  const [appliedDiscount, setAppliedDiscount] = useState<typeof discountCodes[0] | null>(null);
  const [discountError, setDiscountError] = useState('');
  const [formData, setFormData] = useState<Record<string, string>>({});
  const [isProcessing, setIsProcessing] = useState(false);
  const [paymentSuccess, setPaymentSuccess] = useState(false);
  const [showProductDropdown, setShowProductDropdown] = useState(false);
  const [showInvoiceModal, setShowInvoiceModal] = useState(false);
  const [currentTransaction, setCurrentTransaction] = useState<{
    invoiceId: string;
    amount: number;
    originalAmount: number;
  } | null>(null);

  const invoiceRef = useRef<HTMLDivElement>(null);

  const selectedProduct = products.find(p => p.id === selectedProductId);
  const activeProducts = products.filter(p => p.active);
  const enabledFields = formConfig.fields.filter(f => f.enabled).sort((a, b) => a.order - b.order);

  const applyDiscountCode = () => {
    if (!discountCodeInput.trim()) {
      setDiscountError('Ingresa un código');
      return;
    }

    const discount = validateDiscountCode(discountCodeInput, selectedProductId);
    if (discount) {
      setAppliedDiscount(discount);
      setDiscountError('');
    } else {
      setAppliedDiscount(null);
      setDiscountError('Código inválido o expirado');
    }
  };

  const removeDiscount = () => {
    setAppliedDiscount(null);
    setDiscountCodeInput('');
  };

  const calculateTotal = () => {
    if (!selectedProduct) return 0;
    let total = selectedProduct.price;

    if (appliedDiscount) {
      if (appliedDiscount.discountType === 'percentage') {
        total = total * (1 - appliedDiscount.discountValue / 100);
      } else {
        total = Math.max(0, total - appliedDiscount.discountValue);
      }
    }

    return total;
  };

  const discountAmount = selectedProduct ? selectedProduct.price - calculateTotal() : 0;

  const generateInvoiceId = () => `INV-${Date.now()}-${Math.random().toString(36).substr(2, 9).toUpperCase()}`;

  const handlePayment = async (method: 'stripe' | 'paypal' | 'card') => {
    if (!selectedProduct) {
      alert('Por favor selecciona un producto');
      return;
    }

    // Check if user has credentials for real testing
    if (method === 'paypal' && !paymentConfig.paypalClientId && !isProcessing) {
      alert("⚠️ No has configurado un 'Client ID' de PayPal en Configuración.\n\nSe usará el modo simulación.");
    }

    // Validate required fields
    const missingFields = enabledFields
      .filter(f => f.required && !formData[f.id]?.trim())
      .map(f => f.label);

    if (missingFields.length > 0) {
      alert(`Por favor completa los campos requeridos: ${missingFields.join(', ')}`);
      return;
    }

    setIsProcessing(true);

    // Simulate payment processing
    await new Promise(resolve => setTimeout(resolve, 2000));

    const invoiceId = generateInvoiceId();
    const amount = calculateTotal();
    const originalAmount = selectedProduct.price;

    // Add transaction
    addTransaction({
      productId: selectedProduct.id,
      productName: selectedProduct.name,
      amount,
      originalAmount,
      currency: selectedProduct.currency,
      discountCode: appliedDiscount?.code,
      discountAmount: discountAmount > 0 ? discountAmount : undefined,
      paymentMethod: method === 'card' ? 'stripe' : method,
      status: 'completed',
      customerEmail: formData.email || '',
      customerName: formData.name || '',
      customerPhone: formData.phone,
      customerAddress: formData.address,
      customerCity: formData.city,
      customerCountry: formData.country,
      customerZip: formData.zip,
      customerCompany: formData.company,
      customerNotes: formData.notes,
      invoiceId,
    });

    // Update discount code usage
    if (appliedDiscount) {
      updateDiscountCode(appliedDiscount.id, {
        currentUses: appliedDiscount.currentUses + 1
      });
    }

    setCurrentTransaction({ invoiceId, amount, originalAmount });
    setIsProcessing(false);
    setPaymentSuccess(true);

    // Show invoice modal if enabled
    if (formStyle.sendInvoiceEmail) {
      setShowInvoiceModal(true);
    }
  };

  const getBorderRadiusClass = (radius: string) => {
    switch (radius) {
      case 'none': return 'rounded-none';
      case 'sm': return 'rounded-sm';
      case 'md': return 'rounded-md';
      case 'lg': return 'rounded-lg';
      case 'xl': return 'rounded-xl';
      case 'full': return 'rounded-2xl';
      default: return 'rounded-lg';
    }
  };

  const handleDownloadInvoice = () => {
    // In a real app, this would generate a PDF
    alert('En producción, esto generaría y descargaría un PDF de la factura.');
  };

  const resetForm = () => {
    setPaymentSuccess(false);
    setFormData({});
    setAppliedDiscount(null);
    setDiscountCodeInput('');
    setShowInvoiceModal(false);
    setCurrentTransaction(null);
  };

  // Invoice Modal
  const InvoiceModal = () => (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
      <div className="bg-white rounded-2xl w-full max-w-2xl shadow-xl max-h-[90vh] overflow-y-auto">
        <div className="flex items-center justify-between p-6 border-b sticky top-0 bg-white">
          <h3 className="text-xl font-semibold text-slate-800 flex items-center gap-2">
            <FileText size={24} className="text-green-600" />
            Factura Generada
          </h3>
          <button
            onClick={() => setShowInvoiceModal(false)}
            className="p-2 hover:bg-slate-100 rounded-lg"
          >
            <X size={20} className="text-slate-500" />
          </button>
        </div>

        <div className="p-6">
          <div className="bg-green-50 border border-green-200 rounded-lg p-4 mb-6 flex items-start gap-3">
            <Mail className="text-green-600 flex-shrink-0 mt-0.5" size={20} />
            <div>
              <p className="text-green-800 font-medium">Factura enviada por email</p>
              <p className="text-green-700 text-sm mt-1">
                Se ha enviado una copia de esta factura a <strong>{formData.email}</strong>
              </p>
            </div>
          </div>

          {/* Invoice Preview */}
          <div ref={invoiceRef} className="border rounded-xl p-8 bg-white">
            <div className="flex items-start justify-between mb-8">
              <div>
                {formStyle.logoUrl ? (
                  <img src={formStyle.logoUrl} alt="Logo" className="h-12 mb-2" />
                ) : (
                  <div
                    className="w-12 h-12 rounded-lg flex items-center justify-center mb-2"
                    style={{ backgroundColor: formStyle.primaryColor }}
                  >
                    <span className="text-white font-bold text-xl">
                      {formStyle.companyName.charAt(0)}
                    </span>
                  </div>
                )}
                <h4 className="font-bold text-xl">{formStyle.companyName}</h4>
                <p className="text-sm text-slate-500">{formStyle.invoiceFromEmail}</p>
              </div>
              <div className="text-right">
                <h3 className="text-2xl font-bold text-slate-800">FACTURA</h3>
                <p className="text-sm text-slate-500 font-mono">{currentTransaction?.invoiceId}</p>
                <p className="text-sm text-slate-500 mt-2">
                  Fecha: {new Date().toLocaleDateString('es-ES', {
                    day: '2-digit',
                    month: 'long',
                    year: 'numeric'
                  })}
                </p>
              </div>
            </div>

            <div className="grid grid-cols-2 gap-8 mb-8">
              <div>
                <h5 className="text-sm font-medium text-slate-500 mb-2">FACTURADO A:</h5>
                <p className="font-medium text-slate-800">{formData.name}</p>
                <p className="text-slate-600">{formData.email}</p>
                {formData.phone && <p className="text-slate-600">{formData.phone}</p>}
                {formData.company && <p className="text-slate-600">{formData.company}</p>}
                {formData.address && (
                  <p className="text-slate-600">
                    {formData.address}
                    {formData.city && `, ${formData.city}`}
                    {formData.zip && ` ${formData.zip}`}
                    {formData.country && `, ${formData.country}`}
                  </p>
                )}
              </div>
              <div>
                <h5 className="text-sm font-medium text-slate-500 mb-2">MÉTODO DE PAGO:</h5>
                <p className="font-medium text-slate-800">💳 Tarjeta de crédito</p>
                <p className="text-green-600 font-medium mt-2">✓ Pagado</p>
              </div>
            </div>

            <table className="w-full mb-8">
              <thead>
                <tr className="border-b-2 border-slate-200">
                  <th className="text-left py-3 text-sm font-medium text-slate-500">DESCRIPCIÓN</th>
                  <th className="text-right py-3 text-sm font-medium text-slate-500">PRECIO</th>
                </tr>
              </thead>
              <tbody>
                <tr className="border-b border-slate-100">
                  <td className="py-4">
                    <p className="font-medium text-slate-800">{selectedProduct?.name}</p>
                    <p className="text-sm text-slate-500">{selectedProduct?.description}</p>
                    {selectedProduct?.type === 'subscription' && (
                      <span className="inline-block mt-1 px-2 py-0.5 bg-purple-100 text-purple-700 text-xs rounded">
                        Suscripción {selectedProduct.interval === 'monthly' ? 'mensual' :
                          selectedProduct.interval === 'yearly' ? 'anual' : 'semanal'}
                      </span>
                    )}
                  </td>
                  <td className="py-4 text-right font-medium">
                    ${currentTransaction?.originalAmount.toFixed(2)} {selectedProduct?.currency}
                  </td>
                </tr>
                {appliedDiscount && (
                  <tr className="border-b border-slate-100">
                    <td className="py-4">
                      <p className="text-green-600">Descuento ({appliedDiscount.code})</p>
                      <p className="text-sm text-slate-500">
                        {appliedDiscount.discountType === 'percentage'
                          ? `${appliedDiscount.discountValue}% de descuento`
                          : `$${appliedDiscount.discountValue} de descuento`}
                      </p>
                    </td>
                    <td className="py-4 text-right font-medium text-green-600">
                      -${discountAmount.toFixed(2)} {selectedProduct?.currency}
                    </td>
                  </tr>
                )}
              </tbody>
              <tfoot>
                <tr className="border-t-2 border-slate-200">
                  <td className="py-4 font-bold text-lg">TOTAL</td>
                  <td className="py-4 text-right font-bold text-2xl" style={{ color: formStyle.primaryColor }}>
                    ${currentTransaction?.amount.toFixed(2)} {selectedProduct?.currency}
                  </td>
                </tr>
              </tfoot>
            </table>

            {formData.notes && (
              <div className="mb-6">
                <h5 className="text-sm font-medium text-slate-500 mb-2">NOTAS:</h5>
                <p className="text-slate-600 bg-slate-50 p-3 rounded-lg">{formData.notes}</p>
              </div>
            )}

            <div className="text-center pt-6 border-t">
              <p className="text-slate-500 text-sm">¡Gracias por tu compra!</p>
              <p className="text-slate-400 text-xs mt-2">
                Este documento es una confirmación de pago válida.
              </p>
            </div>
          </div>

          <div className="flex gap-3 mt-6">
            <button
              onClick={handleDownloadInvoice}
              className="flex-1 flex items-center justify-center gap-2 px-4 py-3 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors font-medium"
            >
              <Download size={18} />
              Descargar PDF
            </button>
            <button
              onClick={() => setShowInvoiceModal(false)}
              className="flex-1 px-4 py-3 border border-slate-200 text-slate-700 rounded-lg hover:bg-slate-50 transition-colors font-medium"
            >
              Cerrar
            </button>
          </div>
        </div>
      </div>
    </div>
  );

  if (paymentSuccess) {
    return (
      <div className={cn("p-6 flex items-center justify-center min-h-[600px]", darkMode && "text-white")}>
        <div className="text-center">
          <div
            className="w-20 h-20 mx-auto mb-6 flex items-center justify-center rounded-full"
            style={{ backgroundColor: `${formStyle.primaryColor}20` }}
          >
            <Check size={40} style={{ color: formStyle.primaryColor }} />
          </div>
          <h2 className={cn("text-2xl font-bold mb-2", darkMode ? "text-white" : "text-slate-800")}>
            ¡Pago Exitoso!
          </h2>
          <p className={cn("mb-6", darkMode ? "text-slate-400" : "text-slate-500")}>
            Tu pago ha sido procesado correctamente.
          </p>
          <div className={cn(
            "rounded-xl p-4 mb-6 text-left max-w-sm mx-auto",
            darkMode ? "bg-slate-800" : "bg-slate-50"
          )}>
            <p className={cn("text-sm mb-2", darkMode ? "text-slate-300" : "text-slate-600")}>
              <span className="font-medium">Producto:</span> {selectedProduct?.name}
            </p>
            <p className={cn("text-sm mb-2", darkMode ? "text-slate-300" : "text-slate-600")}>
              <span className="font-medium">Total pagado:</span> ${calculateTotal().toFixed(2)}
            </p>
            <p className={cn("text-sm mb-2", darkMode ? "text-slate-300" : "text-slate-600")}>
              <span className="font-medium">Factura:</span> {currentTransaction?.invoiceId}
            </p>
            {appliedDiscount && (
              <p className="text-sm text-green-600">
                <span className="font-medium">Descuento aplicado:</span> {appliedDiscount.code}
              </p>
            )}
          </div>
          <div className="flex gap-3 justify-center">
            {formStyle.sendInvoiceEmail && (
              <button
                onClick={() => setShowInvoiceModal(true)}
                className="flex items-center gap-2 px-6 py-3 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors font-medium"
              >
                <FileText size={18} />
                Ver Factura
              </button>
            )}
            <button
              onClick={resetForm}
              className={cn(
                "px-6 py-3 rounded-lg transition-colors font-medium",
                darkMode
                  ? "bg-slate-700 text-white hover:bg-slate-600"
                  : "bg-slate-100 text-slate-700 hover:bg-slate-200"
              )}
            >
              Nueva compra
            </button>
          </div>
        </div>
        {showInvoiceModal && <InvoiceModal />}
      </div>
    );
  }

  return (
    <div className={cn("p-6", darkMode && "text-white")}>
      <div className="mb-6">
        <h2 className={cn("text-2xl font-bold", darkMode ? "text-white" : "text-slate-800")}>
          Vista Previa del Checkout
        </h2>
        <p className={cn(darkMode ? "text-slate-400" : "text-slate-500")}>
          Así verán tus clientes el formulario de pago
        </p>
      </div>

      <div className="flex justify-center">
        <div
          className={cn("w-full max-w-md p-8 shadow-2xl", getBorderRadiusClass(formStyle.borderRadius))}
          style={{
            backgroundColor: formStyle.backgroundColor,
            fontFamily: formStyle.fontFamily,
          }}
        >
          {/* Header */}
          <div className="text-center mb-8">
            {formStyle.showLogo && (
              formStyle.logoUrl ? (
                <img
                  src={formStyle.logoUrl}
                  alt="Logo"
                  className="h-14 mx-auto mb-4 object-contain"
                  onError={(e) => {
                    (e.target as HTMLImageElement).style.display = 'none';
                  }}
                />
              ) : (
                <div
                  className={cn("w-14 h-14 mx-auto mb-4 flex items-center justify-center", getBorderRadiusClass(formStyle.borderRadius))}
                  style={{ backgroundColor: formStyle.primaryColor }}
                >
                  <span className="text-white font-bold text-2xl">
                    {formStyle.companyName.charAt(0)}
                  </span>
                </div>
              )
            )}
            <h2
              className="text-2xl font-bold"
              style={{ color: formStyle.textColor }}
            >
              {formStyle.companyName}
            </h2>
            <p
              className="text-sm opacity-60 mt-1"
              style={{ color: formStyle.textColor }}
            >
              Checkout seguro
            </p>
          </div>

          {/* Product Selector */}
          {activeProducts.length > 1 && (
            <div className="mb-6 relative">
              <label
                className="block text-sm font-medium mb-2"
                style={{ color: formStyle.textColor }}
              >
                Selecciona un producto
              </label>
              <button
                onClick={() => setShowProductDropdown(!showProductDropdown)}
                className={cn(
                  "w-full px-4 py-3 border text-left flex items-center justify-between",
                  getBorderRadiusClass(formStyle.borderRadius)
                )}
                style={{
                  borderColor: `${formStyle.primaryColor}40`,
                  color: formStyle.textColor,
                  backgroundColor: formStyle.backgroundColor,
                }}
              >
                <span>{selectedProduct?.name || 'Seleccionar...'}</span>
                <ChevronDown size={20} style={{ color: formStyle.primaryColor }} />
              </button>
              {showProductDropdown && (
                <div
                  className={cn("absolute top-full left-0 right-0 mt-1 shadow-lg border z-10", getBorderRadiusClass(formStyle.borderRadius))}
                  style={{
                    backgroundColor: formStyle.backgroundColor,
                    borderColor: `${formStyle.primaryColor}20`
                  }}
                >
                  {activeProducts.map((product) => (
                    <button
                      key={product.id}
                      onClick={() => {
                        setSelectedProductId(product.id);
                        setShowProductDropdown(false);
                        setAppliedDiscount(null);
                        setDiscountCodeInput('');
                      }}
                      className={cn(
                        "w-full px-4 py-3 text-left hover:opacity-80 transition-opacity flex items-center justify-between",
                        product.id === selectedProductId && "font-medium"
                      )}
                      style={{
                        color: formStyle.textColor,
                        backgroundColor: product.id === selectedProductId ? `${formStyle.primaryColor}10` : 'transparent'
                      }}
                    >
                      <span>{product.name}</span>
                      <span style={{ color: formStyle.primaryColor }}>
                        ${product.price}
                        {product.type === 'subscription' && '/mes'}
                      </span>
                    </button>
                  ))}
                </div>
              )}
            </div>
          )}

          {/* Selected Product Info */}
          {selectedProduct && (
            <div
              className={cn("p-5 mb-6", getBorderRadiusClass(formStyle.borderRadius))}
              style={{ backgroundColor: `${formStyle.primaryColor}10` }}
            >
              <div className="flex items-start justify-between">
                <div>
                  <h3
                    className="font-semibold text-lg"
                    style={{ color: formStyle.textColor }}
                  >
                    {selectedProduct.name}
                  </h3>
                  <p
                    className="text-sm opacity-70 mt-1"
                    style={{ color: formStyle.textColor }}
                  >
                    {selectedProduct.description}
                  </p>
                </div>
                <span
                  className={cn("px-2 py-1 text-xs font-medium", getBorderRadiusClass(formStyle.borderRadius))}
                  style={{
                    backgroundColor: formStyle.primaryColor,
                    color: '#fff'
                  }}
                >
                  {selectedProduct.type === 'subscription' ? 'Suscripción' : 'Único'}
                </span>
              </div>
              <div className="mt-4 pt-4 border-t" style={{ borderColor: `${formStyle.primaryColor}20` }}>
                <div className="flex items-baseline justify-between">
                  <span style={{ color: formStyle.textColor }} className="opacity-70">Precio:</span>
                  <span className="text-xl font-bold" style={{ color: formStyle.textColor }}>
                    ${selectedProduct.price.toFixed(2)}
                    {selectedProduct.type === 'subscription' && (
                      <span className="text-sm font-normal opacity-70">
                        /{selectedProduct.interval === 'monthly' ? 'mes' :
                          selectedProduct.interval === 'yearly' ? 'año' : 'semana'}
                      </span>
                    )}
                  </span>
                </div>
                {appliedDiscount && (
                  <>
                    <div className="flex items-center justify-between mt-2 text-green-600">
                      <span>Descuento ({appliedDiscount.code}):</span>
                      <span className="font-medium">-${discountAmount.toFixed(2)}</span>
                    </div>
                    <div className="flex items-baseline justify-between mt-2 pt-2 border-t" style={{ borderColor: `${formStyle.primaryColor}20` }}>
                      <span className="font-semibold" style={{ color: formStyle.textColor }}>Total:</span>
                      <span className="text-2xl font-bold" style={{ color: formStyle.primaryColor }}>
                        ${calculateTotal().toFixed(2)}
                      </span>
                    </div>
                  </>
                )}
              </div>
            </div>
          )}

          {/* Dynamic Form Fields */}
          <div className="space-y-4 mb-6">
            <div className="grid grid-cols-2 gap-4">
              {enabledFields.map((field) => (
                <div
                  key={field.id}
                  className={field.width === 'full' ? 'col-span-2' : ''}
                >
                  <label
                    className="block text-sm font-medium mb-1"
                    style={{ color: formStyle.textColor }}
                  >
                    {field.label}
                    {field.required && <span className="text-red-500 ml-1">*</span>}
                  </label>
                  {field.type === 'notes' ? (
                    <textarea
                      value={formData[field.id] || ''}
                      onChange={(e) => setFormData({ ...formData, [field.id]: e.target.value })}
                      placeholder={field.placeholder}
                      rows={2}
                      className={cn("w-full px-4 py-3 border", getBorderRadiusClass(formStyle.borderRadius))}
                      style={{
                        borderColor: `${formStyle.primaryColor}40`,
                        color: formStyle.textColor,
                        backgroundColor: formStyle.backgroundColor,
                      }}
                    />
                  ) : (
                    <input
                      type={field.type === 'email' ? 'email' : field.type === 'phone' ? 'tel' : 'text'}
                      value={formData[field.id] || ''}
                      onChange={(e) => setFormData({ ...formData, [field.id]: e.target.value })}
                      placeholder={field.placeholder}
                      className={cn("w-full px-4 py-3 border", getBorderRadiusClass(formStyle.borderRadius))}
                      style={{
                        borderColor: `${formStyle.primaryColor}40`,
                        color: formStyle.textColor,
                        backgroundColor: formStyle.backgroundColor,
                      }}
                    />
                  )}
                </div>
              ))}
            </div>
          </div>

          {/* Discount Code */}
          <div className="mb-6">
            <label
              className="block text-sm font-medium mb-1"
              style={{ color: formStyle.textColor }}
            >
              Código de descuento
            </label>
            {appliedDiscount ? (
              <div
                className={cn("flex items-center justify-between p-3 bg-green-50 border border-green-200", getBorderRadiusClass(formStyle.borderRadius))}
              >
                <div className="flex items-center gap-2">
                  <Check className="text-green-600" size={18} />
                  <span className="text-green-700 font-medium">{appliedDiscount.code}</span>
                  <span className="text-green-600 text-sm">
                    ({appliedDiscount.discountType === 'percentage'
                      ? `${appliedDiscount.discountValue}%`
                      : `$${appliedDiscount.discountValue}`} off)
                  </span>
                </div>
                <button
                  onClick={removeDiscount}
                  className="text-sm text-red-500 hover:underline"
                >
                  Quitar
                </button>
              </div>
            ) : (
              <div className="flex gap-2">
                <input
                  type="text"
                  value={discountCodeInput}
                  onChange={(e) => {
                    setDiscountCodeInput(e.target.value.toUpperCase());
                    setDiscountError('');
                  }}
                  placeholder="Ingresa tu código"
                  className={cn(
                    "flex-1 px-4 py-3 border",
                    getBorderRadiusClass(formStyle.borderRadius),
                    discountError && "border-red-300"
                  )}
                  style={{
                    borderColor: discountError ? undefined : `${formStyle.primaryColor}40`,
                    color: formStyle.textColor,
                    backgroundColor: formStyle.backgroundColor,
                  }}
                />
                <button
                  onClick={applyDiscountCode}
                  className={cn("px-4 py-3 font-medium", getBorderRadiusClass(formStyle.borderRadius))}
                  style={{
                    backgroundColor: `${formStyle.primaryColor}20`,
                    color: formStyle.primaryColor,
                  }}
                >
                  Aplicar
                </button>
              </div>
            )}
            {discountError && (
              <p className="flex items-center gap-1 text-red-500 text-sm mt-1">
                <AlertCircle size={14} />
                {discountError}
              </p>
            )}
          </div>

          {/* Payment Buttons */}
          <div className="space-y-3">
            {calculateTotal() > 0 ? (
              <>
                {formStyle.showCardButton && (
                  <button
                    onClick={() => handlePayment('card')}
                    disabled={isProcessing || !selectedProduct}
                    className={cn(
                      "w-full py-4 font-semibold transition-all flex items-center justify-center gap-2 disabled:opacity-50",
                      getBorderRadiusClass(formStyle.borderRadius),
                      formStyle.buttonStyle === 'outline' && "border-2 bg-transparent",
                    )}
                    style={{
                      backgroundColor: formStyle.buttonStyle === 'solid' ? formStyle.primaryColor :
                        formStyle.buttonStyle === 'outline' ? 'transparent' : undefined,
                      borderColor: formStyle.buttonStyle === 'outline' ? formStyle.primaryColor : undefined,
                      color: formStyle.buttonStyle === 'outline' ? formStyle.primaryColor : '#ffffff',
                      backgroundImage: formStyle.buttonStyle === 'gradient'
                        ? `linear-gradient(to right, ${formStyle.primaryColor}, ${formStyle.primaryColor}cc)`
                        : undefined,
                    }}
                  >
                    {isProcessing ? (
                      <span className="animate-pulse">Procesando...</span>
                    ) : (
                      <>
                        <CreditCard size={20} />
                        {formStyle.cardButtonText} ${calculateTotal().toFixed(2)}
                      </>
                    )}
                  </button>
                )}

                {formStyle.showStripeButton && (
                  <button
                    onClick={() => handlePayment('stripe')}
                    disabled={isProcessing || !selectedProduct}
                    className={cn(
                      "w-full py-4 font-semibold bg-indigo-600 text-white hover:bg-indigo-700 transition-all flex items-center justify-center gap-2 disabled:opacity-50",
                      getBorderRadiusClass(formStyle.borderRadius)
                    )}
                  >
                    {isProcessing ? (
                      <span className="animate-pulse">Procesando...</span>
                    ) : (
                      <>
                        <span className="w-6 h-6 bg-white/20 rounded flex items-center justify-center font-bold">S</span>
                        {formStyle.stripeButtonText}
                      </>
                    )}
                  </button>
                )}

                {formStyle.showPaypalButton && (
                  <>
                    {paymentConfig.paypalEnabled && paymentConfig.paypalClientId ? (
                      <div className="w-full relative z-0">
                        <PayPalScriptProvider options={{
                          "clientId": paymentConfig.paypalClientId,
                          currency: selectedProduct?.currency || "USD",
                          intent: "capture"
                        }}>
                          <div className="w-full">
                            <PayPalButtons
                              style={{
                                layout: "vertical",
                                color: "blue",
                                shape: "rect",
                                label: "pay",
                                height: 48
                              }}
                              forceReRender={[selectedProduct?.price, selectedProduct?.currency, calculateTotal()]}
                              createOrder={(data, actions) => {
                                const totalValue = calculateTotal().toFixed(2);
                                if (parseFloat(totalValue) <= 0) {
                                  alert("El monto debe ser mayor a 0.");
                                  return Promise.reject("Amount must be > 0");
                                }

                                return actions.order.create({
                                  purchase_units: [
                                    {
                                      description: selectedProduct?.name.substring(0, 127) || "Producto",
                                      amount: {
                                        value: totalValue,
                                        currency_code: selectedProduct?.currency || "USD"
                                      },
                                    },
                                  ],
                                  application_context: {
                                    shipping_preference: "NO_SHIPPING"
                                  }
                                }).catch((err) => {
                                  console.error("Create Order Error:", err);
                                  alert("Error iniciando PayPal. Verifica tu Client ID y que el monto no sea 0.");
                                  throw err;
                                });
                              }}
                              onApprove={async (data, actions) => {
                                if (!actions.order) return;
                                try {
                                  const order = await actions.order.capture();
                                  const invoiceId = generateInvoiceId();
                                  const amount = calculateTotal();
                                  const originalAmount = selectedProduct?.price || 0;

                                  addTransaction({
                                    productId: selectedProduct?.id || '',
                                    productName: selectedProduct?.name || '',
                                    amount,
                                    originalAmount,
                                    currency: selectedProduct?.currency || 'USD',
                                    discountCode: appliedDiscount?.code,
                                    discountAmount: discountAmount > 0 ? discountAmount : undefined,
                                    paymentMethod: 'paypal',
                                    status: 'completed',
                                    customerEmail: formData.email || (order.payer?.email_address || ''),
                                    customerName: formData.name || ((order.payer?.name?.given_name || '') + ' ' + (order.payer?.name?.surname || '')),
                                    invoiceId,
                                  });

                                  setCurrentTransaction({ invoiceId, amount, originalAmount });
                                  setPaymentSuccess(true);
                                  if (formStyle.sendInvoiceEmail) setShowInvoiceModal(true);
                                } catch (err) {
                                  console.error("Capture Error:", err);
                                  alert("Error procesando el pago");
                                }
                              }}
                              onError={(err) => {
                                console.error("PayPal onError:", err);
                              }}
                            />
                          </div>
                        </PayPalScriptProvider>
                      </div>
                    ) : (
                      <button
                        onClick={() => handlePayment('paypal')}
                        disabled={isProcessing || !selectedProduct}
                        className={cn(
                          "w-full py-4 font-semibold bg-[#0070ba] text-white hover:bg-[#005ea6] transition-all flex items-center justify-center gap-2 disabled:opacity-50",
                          getBorderRadiusClass(formStyle.borderRadius)
                        )}
                      >
                        {isProcessing ? (
                          <span className="animate-pulse">Procesando...</span>
                        ) : (
                          <>
                            <span className="font-bold">Pay</span>
                            <span className="font-bold text-[#003087]">Pal</span>
                            <span className="text-xs opacity-80 font-normal ml-1">(Simulación)</span>
                          </>
                        )}
                      </button>
                    )}
                  </>
                )}
              </>
            ) : (
              <div className="p-4 bg-yellow-50 border border-yellow-200 rounded-lg text-yellow-800 text-sm text-center">
                ⚠️ Para probar el pago, primero <strong>crea un producto</strong> con precio mayor a $0 en la pestaña Productos.
              </div>
            )}
          </div>

          {/* No payment buttons warning */}
          {!formStyle.showCardButton && !formStyle.showStripeButton && !formStyle.showPaypalButton && (
            <div className="p-4 bg-amber-50 border border-amber-200 rounded-lg text-amber-800 text-sm text-center">
              ⚠️ No hay métodos de pago habilitados. Configúralos en el constructor de formularios.
            </div>
          )}

          {/* Security Footer */}
          <div className="mt-6 flex items-center justify-center gap-2 text-xs opacity-50" style={{ color: formStyle.textColor }}>
            <Lock size={14} />
            <span>Pago 100% seguro • Encriptación SSL de 256-bit</span>
          </div>
        </div>
      </div>

      {showInvoiceModal && <InvoiceModal />}
    </div>
  );
}
