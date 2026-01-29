import { useState } from 'react';
import { 
  Code, Copy, Check, ExternalLink, Globe, Layout, 
  MessageSquare, Package, ChevronDown, ChevronUp, Eye
} from 'lucide-react';
import { useApp } from '../context/AppContext';
import { cn } from '../utils/cn';

type IntegrationType = 'script' | 'iframe' | 'link' | 'widget' | 'popup';

export function Integration() {
  const { products, formStyle, formConfig, paymentConfig, darkMode } = useApp();
  const [copiedCode, setCopiedCode] = useState<string | null>(null);
  const [selectedType, setSelectedType] = useState<IntegrationType>('script');
  const [selectedProductIds, setSelectedProductIds] = useState<string[]>(products.filter(p => p.active).map(p => p.id));
  const [widgetPosition, setWidgetPosition] = useState<'bottom-right' | 'bottom-left'>('bottom-right');
  const [widgetButtonText, setWidgetButtonText] = useState('💳 Comprar Ahora');
  const [showProductSelector, setShowProductSelector] = useState(false);
  const [customDomain, setCustomDomain] = useState('https://tudominio.com');

  const activeProducts = products.filter(p => p.active);
  const embedId = `payform_${Math.random().toString(36).substr(2, 9)}`;

  const copyToClipboard = (code: string, id: string) => {
    navigator.clipboard.writeText(code);
    setCopiedCode(id);
    setTimeout(() => setCopiedCode(null), 2000);
  };

  // Generate configuration JSON
  const configJSON = JSON.stringify({
    embedId,
    products: selectedProductIds,
    style: {
      primaryColor: formStyle.primaryColor,
      backgroundColor: formStyle.backgroundColor,
      textColor: formStyle.textColor,
      borderRadius: formStyle.borderRadius,
      buttonStyle: formStyle.buttonStyle,
      fontFamily: formStyle.fontFamily,
      companyName: formStyle.companyName,
      logoUrl: formStyle.logoUrl,
    },
    fields: formConfig.fields.filter(f => f.enabled).map(f => ({
      id: f.id,
      label: f.label,
      required: f.required,
      width: f.width,
    })),
    paymentMethods: {
      card: formStyle.showCardButton,
      stripe: formStyle.showStripeButton && paymentConfig.stripeEnabled,
      paypal: formStyle.showPaypalButton && paymentConfig.paypalEnabled,
    },
    buttons: {
      card: formStyle.cardButtonText,
      stripe: formStyle.stripeButtonText,
      paypal: formStyle.paypalButtonText,
    },
    invoice: {
      enabled: formStyle.sendInvoiceEmail,
      fromName: formStyle.invoiceFromName,
      fromEmail: formStyle.invoiceFromEmail,
    }
  }, null, 2);

  // Generate different code types
  const generateScriptCode = () => `<!-- PayForm Pro - Formulario de Pago -->
<div id="payform-container"></div>
<script>
  window.PayFormConfig = ${configJSON};
</script>
<script src="${customDomain}/payform.js" async></script>
<!-- Fin PayForm Pro -->`;

  const generateIframeCode = () => `<!-- PayForm Pro - iFrame -->
<iframe 
  src="${customDomain}/checkout?products=${selectedProductIds.join(',')}&embed=true"
  width="100%" 
  height="700" 
  frameborder="0"
  style="border: none; border-radius: ${formStyle.borderRadius === 'none' ? '0' : formStyle.borderRadius === 'sm' ? '4px' : formStyle.borderRadius === 'md' ? '8px' : formStyle.borderRadius === 'lg' ? '12px' : '16px'}; box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);">
</iframe>`;

  const generateDirectLink = () => `${customDomain}/checkout?products=${selectedProductIds.join(',')}`;

  const generateWidgetCode = () => `<!-- PayForm Pro - Widget Flotante -->
<script>
  window.PayFormWidget = {
    position: '${widgetPosition}',
    buttonText: '${widgetButtonText}',
    buttonColor: '${formStyle.primaryColor}',
    products: ${JSON.stringify(selectedProductIds)},
    config: ${configJSON}
  };
</script>
<script src="${customDomain}/payform-widget.js" async></script>
<!-- Fin Widget PayForm Pro -->`;

  const generatePopupCode = () => `<!-- PayForm Pro - Botón con Popup -->
<button 
  onclick="PayFormPopup.open({products: ${JSON.stringify(selectedProductIds)}})"
  style="background: ${formStyle.primaryColor}; color: white; padding: 12px 24px; border: none; border-radius: 8px; font-weight: 600; cursor: pointer; font-size: 16px;">
  ${formStyle.cardButtonText}
</button>
<script src="${customDomain}/payform-popup.js" async></script>`;

  const integrationTypes = [
    {
      id: 'script' as IntegrationType,
      title: 'Script Embebible',
      description: 'El formulario se renderiza directamente en tu página',
      icon: <Code size={24} />,
      recommended: true,
    },
    {
      id: 'iframe' as IntegrationType,
      title: 'iFrame',
      description: 'Insertar el checkout en un iframe aislado',
      icon: <Layout size={24} />,
      recommended: false,
    },
    {
      id: 'link' as IntegrationType,
      title: 'Link Directo',
      description: 'Enlace a una página de checkout externa',
      icon: <ExternalLink size={24} />,
      recommended: false,
    },
    {
      id: 'widget' as IntegrationType,
      title: 'Widget Flotante',
      description: 'Botón flotante que abre el checkout en modal',
      icon: <MessageSquare size={24} />,
      recommended: false,
    },
    {
      id: 'popup' as IntegrationType,
      title: 'Botón con Popup',
      description: 'Botón personalizable que abre popup de pago',
      icon: <Globe size={24} />,
      recommended: false,
    },
  ];

  const getCodeByType = () => {
    switch (selectedType) {
      case 'script': return generateScriptCode();
      case 'iframe': return generateIframeCode();
      case 'link': return generateDirectLink();
      case 'widget': return generateWidgetCode();
      case 'popup': return generatePopupCode();
      default: return '';
    }
  };

  return (
    <div className="p-6 space-y-6">
      <div>
        <h2 className={cn("text-2xl font-bold", darkMode ? "text-white" : "text-slate-800")}>
          Integración y Código
        </h2>
        <p className={cn(darkMode ? "text-slate-400" : "text-slate-500")}>
          Obtén el código para integrar el formulario de pago en tu sitio web
        </p>
      </div>

      {/* Info Banner */}
      <div className="bg-gradient-to-r from-indigo-500 to-purple-600 rounded-xl p-6 text-white">
        <h3 className="text-lg font-semibold mb-2">🚀 Integra pagos en minutos</h3>
        <p className="text-indigo-100">
          Copia el código generado y pégalo en tu sitio web. El formulario se conectará 
          automáticamente con tus configuraciones de Stripe y PayPal.
        </p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Left Column - Settings */}
        <div className="lg:col-span-1 space-y-6">
          {/* Domain Configuration */}
          <div className={cn(
            "rounded-xl p-6 shadow-sm border",
            darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100"
          )}>
            <h3 className={cn(
              "flex items-center gap-2 text-lg font-semibold mb-4",
              darkMode ? "text-white" : "text-slate-800"
            )}>
              <Globe size={20} />
              Configuración de Dominio
            </h3>
            <div>
              <label className={cn(
                "block text-sm font-medium mb-1",
                darkMode ? "text-slate-300" : "text-slate-700"
              )}>
                URL Base de tu sitio
              </label>
              <input
                type="url"
                value={customDomain}
                onChange={(e) => setCustomDomain(e.target.value)}
                placeholder="https://tudominio.com"
                className={cn(
                  "w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-indigo-500",
                  darkMode 
                    ? "bg-slate-700 border-slate-600 text-white placeholder:text-slate-500" 
                    : "bg-white border-slate-200"
                )}
              />
              <p className={cn("text-xs mt-2", darkMode ? "text-slate-500" : "text-slate-400")}>
                Esta URL se usará para generar los enlaces de checkout
              </p>
            </div>
          </div>

          {/* Product Selector */}
          <div className={cn(
            "rounded-xl p-6 shadow-sm border",
            darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100"
          )}>
            <button
              onClick={() => setShowProductSelector(!showProductSelector)}
              className={cn(
                "w-full flex items-center justify-between text-lg font-semibold",
                darkMode ? "text-white" : "text-slate-800"
              )}
            >
              <span className="flex items-center gap-2">
                <Package size={20} />
                Productos a Incluir
              </span>
              {showProductSelector ? <ChevronUp size={20} /> : <ChevronDown size={20} />}
            </button>
            
            {showProductSelector && (
              <div className="mt-4 space-y-2">
                <button
                  onClick={() => setSelectedProductIds(activeProducts.map(p => p.id))}
                  className="text-sm text-indigo-500 hover:underline"
                >
                  Seleccionar todos
                </button>
                <button
                  onClick={() => setSelectedProductIds([])}
                  className="text-sm text-slate-400 hover:underline ml-4"
                >
                  Deseleccionar todos
                </button>
                
                <div className="space-y-2 mt-3">
                  {activeProducts.map((product) => (
                    <label
                      key={product.id}
                      className={cn(
                        "flex items-center gap-3 p-3 rounded-lg cursor-pointer transition-colors",
                        selectedProductIds.includes(product.id)
                          ? darkMode ? "bg-indigo-900/30" : "bg-indigo-50"
                          : darkMode ? "hover:bg-slate-700" : "hover:bg-slate-50"
                      )}
                    >
                      <input
                        type="checkbox"
                        checked={selectedProductIds.includes(product.id)}
                        onChange={(e) => {
                          if (e.target.checked) {
                            setSelectedProductIds([...selectedProductIds, product.id]);
                          } else {
                            setSelectedProductIds(selectedProductIds.filter(id => id !== product.id));
                          }
                        }}
                        className="rounded text-indigo-600"
                      />
                      <div className="flex-1">
                        <p className={cn(
                          "font-medium text-sm",
                          darkMode ? "text-white" : "text-slate-800"
                        )}>
                          {product.name}
                        </p>
                        <p className={cn(
                          "text-xs",
                          darkMode ? "text-slate-400" : "text-slate-500"
                        )}>
                          ${product.price} {product.currency}
                          {product.type === 'subscription' && '/mes'}
                        </p>
                      </div>
                    </label>
                  ))}
                </div>
                
                {activeProducts.length === 0 && (
                  <p className={cn("text-sm text-center py-4", darkMode ? "text-slate-500" : "text-slate-400")}>
                    No hay productos activos. Crea productos primero.
                  </p>
                )}
              </div>
            )}
            
            <div className={cn(
              "mt-4 pt-4 border-t",
              darkMode ? "border-slate-700" : "border-slate-200"
            )}>
              <p className={cn("text-sm", darkMode ? "text-slate-400" : "text-slate-500")}>
                <span className="font-medium">{selectedProductIds.length}</span> producto(s) seleccionado(s)
              </p>
            </div>
          </div>

          {/* Widget Settings (only for widget type) */}
          {selectedType === 'widget' && (
            <div className={cn(
              "rounded-xl p-6 shadow-sm border",
              darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100"
            )}>
              <h3 className={cn(
                "flex items-center gap-2 text-lg font-semibold mb-4",
                darkMode ? "text-white" : "text-slate-800"
              )}>
                <MessageSquare size={20} />
                Configuración del Widget
              </h3>
              
              <div className="space-y-4">
                <div>
                  <label className={cn(
                    "block text-sm font-medium mb-1",
                    darkMode ? "text-slate-300" : "text-slate-700"
                  )}>
                    Posición del botón
                  </label>
                  <div className="grid grid-cols-2 gap-2">
                    {(['bottom-right', 'bottom-left'] as const).map((pos) => (
                      <button
                        key={pos}
                        onClick={() => setWidgetPosition(pos)}
                        className={cn(
                          "px-4 py-2 rounded-lg text-sm font-medium transition-all",
                          widgetPosition === pos
                            ? "bg-indigo-600 text-white"
                            : darkMode
                              ? "bg-slate-700 text-slate-300 hover:bg-slate-600"
                              : "bg-slate-100 text-slate-600 hover:bg-slate-200"
                        )}
                      >
                        {pos === 'bottom-right' ? '↘ Derecha' : '↙ Izquierda'}
                      </button>
                    ))}
                  </div>
                </div>
                
                <div>
                  <label className={cn(
                    "block text-sm font-medium mb-1",
                    darkMode ? "text-slate-300" : "text-slate-700"
                  )}>
                    Texto del botón
                  </label>
                  <input
                    type="text"
                    value={widgetButtonText}
                    onChange={(e) => setWidgetButtonText(e.target.value)}
                    className={cn(
                      "w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-indigo-500",
                      darkMode 
                        ? "bg-slate-700 border-slate-600 text-white" 
                        : "bg-white border-slate-200"
                    )}
                  />
                </div>
              </div>
            </div>
          )}
        </div>

        {/* Right Column - Code Generation */}
        <div className="lg:col-span-2 space-y-6">
          {/* Integration Type Selector */}
          <div className={cn(
            "rounded-xl p-6 shadow-sm border",
            darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100"
          )}>
            <h3 className={cn(
              "text-lg font-semibold mb-4",
              darkMode ? "text-white" : "text-slate-800"
            )}>
              Tipo de Integración
            </h3>
            
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
              {integrationTypes.map((type) => (
                <button
                  key={type.id}
                  onClick={() => setSelectedType(type.id)}
                  className={cn(
                    "relative p-4 rounded-xl border-2 text-left transition-all",
                    selectedType === type.id
                      ? "border-indigo-500 bg-indigo-50"
                      : darkMode
                        ? "border-slate-700 hover:border-slate-600 bg-slate-700/50"
                        : "border-slate-200 hover:border-slate-300"
                  )}
                >
                  {type.recommended && (
                    <span className="absolute top-2 right-2 px-2 py-0.5 bg-green-500 text-white text-xs font-medium rounded-full">
                      Recomendado
                    </span>
                  )}
                  <div className={cn(
                    "w-10 h-10 rounded-lg flex items-center justify-center mb-3",
                    selectedType === type.id
                      ? "bg-indigo-600 text-white"
                      : darkMode
                        ? "bg-slate-600 text-slate-300"
                        : "bg-slate-100 text-slate-600"
                  )}>
                    {type.icon}
                  </div>
                  <h4 className={cn(
                    "font-semibold text-sm",
                    selectedType === type.id
                      ? "text-indigo-700"
                      : darkMode ? "text-white" : "text-slate-800"
                  )}>
                    {type.title}
                  </h4>
                  <p className={cn(
                    "text-xs mt-1",
                    selectedType === type.id
                      ? "text-indigo-600"
                      : darkMode ? "text-slate-400" : "text-slate-500"
                  )}>
                    {type.description}
                  </p>
                </button>
              ))}
            </div>
          </div>

          {/* Generated Code */}
          <div className={cn(
            "rounded-xl shadow-sm border overflow-hidden",
            darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100"
          )}>
            <div className={cn(
              "flex items-center justify-between px-6 py-4 border-b",
              darkMode ? "border-slate-700 bg-slate-700/50" : "border-slate-100 bg-slate-50"
            )}>
              <h3 className={cn(
                "font-semibold",
                darkMode ? "text-white" : "text-slate-800"
              )}>
                {selectedType === 'link' ? 'Link Directo' : 'Código para Copiar'}
              </h3>
              <button
                onClick={() => copyToClipboard(getCodeByType(), 'main')}
                className={cn(
                  "flex items-center gap-2 px-4 py-2 rounded-lg font-medium transition-all",
                  copiedCode === 'main'
                    ? "bg-green-500 text-white"
                    : "bg-indigo-600 text-white hover:bg-indigo-700"
                )}
              >
                {copiedCode === 'main' ? (
                  <>
                    <Check size={18} />
                    ¡Copiado!
                  </>
                ) : (
                  <>
                    <Copy size={18} />
                    Copiar Código
                  </>
                )}
              </button>
            </div>
            
            <div className="p-6">
              {selectedType === 'link' ? (
                <div className="space-y-4">
                  <div className={cn(
                    "flex items-center gap-3 p-4 rounded-lg",
                    darkMode ? "bg-slate-700" : "bg-slate-50"
                  )}>
                    <ExternalLink size={20} className="text-indigo-500 flex-shrink-0" />
                    <input
                      type="text"
                      value={generateDirectLink()}
                      readOnly
                      className={cn(
                        "flex-1 bg-transparent font-mono text-sm",
                        darkMode ? "text-slate-300" : "text-slate-700"
                      )}
                    />
                    <button
                      onClick={() => copyToClipboard(generateDirectLink(), 'link')}
                      className={cn(
                        "p-2 rounded-lg transition-colors",
                        copiedCode === 'link'
                          ? "bg-green-100 text-green-600"
                          : darkMode
                            ? "hover:bg-slate-600 text-slate-400"
                            : "hover:bg-slate-200 text-slate-500"
                      )}
                    >
                      {copiedCode === 'link' ? <Check size={18} /> : <Copy size={18} />}
                    </button>
                  </div>
                  
                  <div className="flex gap-3">
                    <a
                      href={generateDirectLink()}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="flex items-center gap-2 px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors"
                    >
                      <Eye size={18} />
                      Probar Link
                    </a>
                    <button
                      onClick={() => {
                        const qrUrl = `https://api.qrserver.com/v1/create-qr-code/?size=200x200&data=${encodeURIComponent(generateDirectLink())}`;
                        window.open(qrUrl, '_blank');
                      }}
                      className={cn(
                        "flex items-center gap-2 px-4 py-2 rounded-lg transition-colors",
                        darkMode
                          ? "bg-slate-700 text-slate-300 hover:bg-slate-600"
                          : "bg-slate-100 text-slate-700 hover:bg-slate-200"
                      )}
                    >
                      Generar QR
                    </button>
                  </div>
                </div>
              ) : (
                <pre className={cn(
                  "p-4 rounded-lg overflow-x-auto text-sm font-mono leading-relaxed",
                  darkMode ? "bg-slate-900 text-slate-300" : "bg-slate-900 text-slate-100"
                )}>
                  <code>{getCodeByType()}</code>
                </pre>
              )}
            </div>
          </div>

          {/* Instructions */}
          <div className={cn(
            "rounded-xl p-6 shadow-sm border",
            darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100"
          )}>
            <h3 className={cn(
              "text-lg font-semibold mb-4",
              darkMode ? "text-white" : "text-slate-800"
            )}>
              📋 Instrucciones de Integración
            </h3>
            
            <div className={cn(
              "space-y-4 text-sm",
              darkMode ? "text-slate-300" : "text-slate-600"
            )}>
              {selectedType === 'script' && (
                <>
                  <div className="flex gap-3">
                    <span className="flex-shrink-0 w-6 h-6 bg-indigo-100 text-indigo-600 rounded-full flex items-center justify-center text-xs font-bold">1</span>
                    <p>Copia el código generado arriba.</p>
                  </div>
                  <div className="flex gap-3">
                    <span className="flex-shrink-0 w-6 h-6 bg-indigo-100 text-indigo-600 rounded-full flex items-center justify-center text-xs font-bold">2</span>
                    <p>Pega el código en tu página HTML donde quieras que aparezca el formulario (generalmente dentro de un <code className="bg-slate-100 dark:bg-slate-700 px-1 rounded">&lt;div&gt;</code>).</p>
                  </div>
                  <div className="flex gap-3">
                    <span className="flex-shrink-0 w-6 h-6 bg-indigo-100 text-indigo-600 rounded-full flex items-center justify-center text-xs font-bold">3</span>
                    <p>El formulario se cargará automáticamente con tu configuración de estilos y productos.</p>
                  </div>
                  <div className="flex gap-3">
                    <span className="flex-shrink-0 w-6 h-6 bg-indigo-100 text-indigo-600 rounded-full flex items-center justify-center text-xs font-bold">4</span>
                    <p>Los pagos se procesarán a través de Stripe/PayPal según tu configuración de APIs.</p>
                  </div>
                </>
              )}
              
              {selectedType === 'iframe' && (
                <>
                  <div className="flex gap-3">
                    <span className="flex-shrink-0 w-6 h-6 bg-indigo-100 text-indigo-600 rounded-full flex items-center justify-center text-xs font-bold">1</span>
                    <p>Copia el código del iframe.</p>
                  </div>
                  <div className="flex gap-3">
                    <span className="flex-shrink-0 w-6 h-6 bg-indigo-100 text-indigo-600 rounded-full flex items-center justify-center text-xs font-bold">2</span>
                    <p>Pégalo en cualquier lugar de tu página HTML.</p>
                  </div>
                  <div className="flex gap-3">
                    <span className="flex-shrink-0 w-6 h-6 bg-indigo-100 text-indigo-600 rounded-full flex items-center justify-center text-xs font-bold">3</span>
                    <p>Ajusta el <code className="bg-slate-100 dark:bg-slate-700 px-1 rounded">width</code> y <code className="bg-slate-100 dark:bg-slate-700 px-1 rounded">height</code> según necesites.</p>
                  </div>
                </>
              )}
              
              {selectedType === 'link' && (
                <>
                  <div className="flex gap-3">
                    <span className="flex-shrink-0 w-6 h-6 bg-indigo-100 text-indigo-600 rounded-full flex items-center justify-center text-xs font-bold">1</span>
                    <p>Usa este link en botones, emails, redes sociales, o donde prefieras.</p>
                  </div>
                  <div className="flex gap-3">
                    <span className="flex-shrink-0 w-6 h-6 bg-indigo-100 text-indigo-600 rounded-full flex items-center justify-center text-xs font-bold">2</span>
                    <p>Los usuarios serán llevados a una página de checkout completa.</p>
                  </div>
                  <div className="flex gap-3">
                    <span className="flex-shrink-0 w-6 h-6 bg-indigo-100 text-indigo-600 rounded-full flex items-center justify-center text-xs font-bold">3</span>
                    <p>Puedes agregar códigos de descuento a la URL: <code className="bg-slate-100 dark:bg-slate-700 px-1 rounded">?discount=CODIGO</code></p>
                  </div>
                </>
              )}
              
              {selectedType === 'widget' && (
                <>
                  <div className="flex gap-3">
                    <span className="flex-shrink-0 w-6 h-6 bg-indigo-100 text-indigo-600 rounded-full flex items-center justify-center text-xs font-bold">1</span>
                    <p>Copia el código del widget.</p>
                  </div>
                  <div className="flex gap-3">
                    <span className="flex-shrink-0 w-6 h-6 bg-indigo-100 text-indigo-600 rounded-full flex items-center justify-center text-xs font-bold">2</span>
                    <p>Pégalo antes del cierre de <code className="bg-slate-100 dark:bg-slate-700 px-1 rounded">&lt;/body&gt;</code> en tu página.</p>
                  </div>
                  <div className="flex gap-3">
                    <span className="flex-shrink-0 w-6 h-6 bg-indigo-100 text-indigo-600 rounded-full flex items-center justify-center text-xs font-bold">3</span>
                    <p>Aparecerá un botón flotante que al hacer clic abre el checkout en un modal.</p>
                  </div>
                </>
              )}
              
              {selectedType === 'popup' && (
                <>
                  <div className="flex gap-3">
                    <span className="flex-shrink-0 w-6 h-6 bg-indigo-100 text-indigo-600 rounded-full flex items-center justify-center text-xs font-bold">1</span>
                    <p>Copia el código del botón.</p>
                  </div>
                  <div className="flex gap-3">
                    <span className="flex-shrink-0 w-6 h-6 bg-indigo-100 text-indigo-600 rounded-full flex items-center justify-center text-xs font-bold">2</span>
                    <p>Colócalo donde quieras que aparezca el botón de compra.</p>
                  </div>
                  <div className="flex gap-3">
                    <span className="flex-shrink-0 w-6 h-6 bg-indigo-100 text-indigo-600 rounded-full flex items-center justify-center text-xs font-bold">3</span>
                    <p>Personaliza el estilo del botón según tu diseño.</p>
                  </div>
                </>
              )}
            </div>
          </div>

          {/* API Webhook Info */}
          <div className={cn(
            "rounded-xl p-6 shadow-sm border",
            darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100"
          )}>
            <h3 className={cn(
              "text-lg font-semibold mb-4",
              darkMode ? "text-white" : "text-slate-800"
            )}>
              🔗 Webhooks y Notificaciones
            </h3>
            
            <p className={cn(
              "text-sm mb-4",
              darkMode ? "text-slate-400" : "text-slate-600"
            )}>
              Configura webhooks para recibir notificaciones cuando ocurran pagos:
            </p>
            
            <div className={cn(
              "p-4 rounded-lg font-mono text-sm",
              darkMode ? "bg-slate-900 text-slate-300" : "bg-slate-100 text-slate-700"
            )}>
              <p className="mb-2"><span className="text-indigo-500">POST</span> {customDomain}/api/webhooks/payment</p>
              <p className="text-xs text-slate-500 mt-2">
                Eventos: payment.completed, payment.failed, subscription.created, subscription.cancelled
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
