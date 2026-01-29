import { useState } from 'react';
import { 
  Palette, Type, Square, Image, Save, CheckCircle, RotateCcw, 
  GripVertical, Eye, EyeOff, ToggleLeft, ToggleRight,
  CreditCard, Mail, FileText
} from 'lucide-react';
import { useApp } from '../context/AppContext';
import { cn } from '../utils/cn';

const colorPresets = [
  { name: 'Indigo', primary: '#6366f1', bg: '#ffffff' },
  { name: 'Emerald', primary: '#10b981', bg: '#ffffff' },
  { name: 'Rose', primary: '#f43f5e', bg: '#ffffff' },
  { name: 'Amber', primary: '#f59e0b', bg: '#ffffff' },
  { name: 'Violet', primary: '#8b5cf6', bg: '#ffffff' },
  { name: 'Cyan', primary: '#06b6d4', bg: '#ffffff' },
  { name: 'Dark', primary: '#6366f1', bg: '#1f2937' },
  { name: 'Sunset', primary: '#f97316', bg: '#fef3c7' },
];

const fontOptions = [
  { name: 'Inter', value: 'Inter' },
  { name: 'Poppins', value: 'Poppins' },
  { name: 'Roboto', value: 'Roboto' },
  { name: 'Open Sans', value: 'Open Sans' },
  { name: 'Montserrat', value: 'Montserrat' },
];

const borderRadiusOptions = [
  { label: 'Sin bordes', value: 'none' as const },
  { label: 'Pequeño', value: 'sm' as const },
  { label: 'Mediano', value: 'md' as const },
  { label: 'Grande', value: 'lg' as const },
  { label: 'Extra grande', value: 'xl' as const },
  { label: 'Completo', value: 'full' as const },
];

export function FormBuilder() {
  const { formStyle, updateFormStyle, formConfig, updateFormField, reorderFields, darkMode } = useApp();
  const [saved, setSaved] = useState(false);
  const [activeSection, setActiveSection] = useState<'style' | 'fields' | 'buttons' | 'invoice'>('fields');
  const [draggedIndex, setDraggedIndex] = useState<number | null>(null);

  const handleSave = () => {
    setSaved(true);
    setTimeout(() => setSaved(false), 3000);
  };

  const handleReset = () => {
    updateFormStyle({
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
    });
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

  const handleDragStart = (index: number) => {
    setDraggedIndex(index);
  };

  const handleDragOver = (e: React.DragEvent, index: number) => {
    e.preventDefault();
    if (draggedIndex !== null && draggedIndex !== index) {
      reorderFields(draggedIndex, index);
      setDraggedIndex(index);
    }
  };

  const handleDragEnd = () => {
    setDraggedIndex(null);
  };

  const sortedFields = [...formConfig.fields].sort((a, b) => a.order - b.order);

  const enabledFields = sortedFields.filter(f => f.enabled);

  return (
    <div className={cn("p-6", darkMode && "text-white")}>
      <div className="mb-6">
        <h2 className={cn("text-2xl font-bold", darkMode ? "text-white" : "text-slate-800")}>
          Personalizar Formulario de Pago
        </h2>
        <p className={cn(darkMode ? "text-slate-400" : "text-slate-500")}>
          Diseña cómo se verá tu página de checkout, configura campos y botones
        </p>
      </div>

      {/* Section Tabs */}
      <div className="flex gap-2 mb-6 flex-wrap">
        {[
          { id: 'fields', label: 'Campos del Formulario', icon: <FileText size={18} /> },
          { id: 'buttons', label: 'Botones de Pago', icon: <CreditCard size={18} /> },
          { id: 'style', label: 'Estilos y Colores', icon: <Palette size={18} /> },
          { id: 'invoice', label: 'Facturas por Email', icon: <Mail size={18} /> },
        ].map((section) => (
          <button
            key={section.id}
            onClick={() => setActiveSection(section.id as typeof activeSection)}
            className={cn(
              "flex items-center gap-2 px-4 py-2 rounded-lg font-medium transition-all",
              activeSection === section.id
                ? "bg-indigo-600 text-white"
                : darkMode 
                  ? "bg-slate-800 text-slate-300 hover:bg-slate-700" 
                  : "bg-white text-slate-600 hover:bg-slate-50 border border-slate-200"
            )}
          >
            {section.icon}
            {section.label}
          </button>
        ))}
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        {/* Settings Panel */}
        <div className="space-y-6">
          {/* Fields Configuration */}
          {activeSection === 'fields' && (
            <div className={cn(
              "rounded-xl p-6 shadow-sm border",
              darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100"
            )}>
              <h3 className={cn(
                "flex items-center gap-2 text-lg font-semibold mb-4",
                darkMode ? "text-white" : "text-slate-800"
              )}>
                <GripVertical size={20} />
                Campos del Formulario
              </h3>
              <p className={cn("text-sm mb-4", darkMode ? "text-slate-400" : "text-slate-500")}>
                Arrastra para reordenar, activa/desactiva campos según necesites
              </p>

              <div className="space-y-2">
                {sortedFields.map((field, index) => (
                  <div
                    key={field.id}
                    draggable
                    onDragStart={() => handleDragStart(index)}
                    onDragOver={(e) => handleDragOver(e, index)}
                    onDragEnd={handleDragEnd}
                    className={cn(
                      "p-4 rounded-lg border transition-all cursor-move",
                      draggedIndex === index && "opacity-50 scale-95",
                      field.enabled
                        ? darkMode 
                          ? "bg-slate-700 border-slate-600" 
                          : "bg-white border-slate-200"
                        : darkMode
                          ? "bg-slate-800/50 border-slate-700 opacity-60"
                          : "bg-slate-50 border-slate-200 opacity-60"
                    )}
                  >
                    <div className="flex items-center gap-3">
                      <GripVertical size={18} className={cn(darkMode ? "text-slate-500" : "text-slate-400")} />
                      
                      <button
                        onClick={() => updateFormField(field.id, { enabled: !field.enabled })}
                        className="flex-shrink-0"
                      >
                        {field.enabled ? (
                          <ToggleRight size={24} className="text-indigo-600" />
                        ) : (
                          <ToggleLeft size={24} className={cn(darkMode ? "text-slate-500" : "text-slate-400")} />
                        )}
                      </button>

                      <div className="flex-1 min-w-0">
                        <input
                          type="text"
                          value={field.label}
                          onChange={(e) => updateFormField(field.id, { label: e.target.value })}
                          className={cn(
                            "w-full px-2 py-1 rounded border font-medium",
                            darkMode 
                              ? "bg-slate-600 border-slate-500 text-white" 
                              : "bg-slate-50 border-slate-200 text-slate-800"
                          )}
                        />
                        <input
                          type="text"
                          value={field.placeholder}
                          onChange={(e) => updateFormField(field.id, { placeholder: e.target.value })}
                          placeholder="Placeholder..."
                          className={cn(
                            "w-full px-2 py-1 mt-1 rounded border text-sm",
                            darkMode 
                              ? "bg-slate-600 border-slate-500 text-slate-300" 
                              : "bg-slate-50 border-slate-200 text-slate-600"
                          )}
                        />
                      </div>

                      <div className="flex items-center gap-2 flex-shrink-0">
                        <select
                          value={field.width}
                          onChange={(e) => updateFormField(field.id, { width: e.target.value as 'full' | 'half' })}
                          className={cn(
                            "px-2 py-1 rounded border text-sm",
                            darkMode 
                              ? "bg-slate-600 border-slate-500 text-white" 
                              : "bg-slate-50 border-slate-200"
                          )}
                        >
                          <option value="full">Ancho completo</option>
                          <option value="half">Medio ancho</option>
                        </select>

                        <label className="flex items-center gap-1 cursor-pointer">
                          <input
                            type="checkbox"
                            checked={field.required}
                            onChange={(e) => updateFormField(field.id, { required: e.target.checked })}
                            className="rounded text-indigo-600"
                          />
                          <span className={cn("text-xs", darkMode ? "text-slate-400" : "text-slate-500")}>
                            Requerido
                          </span>
                        </label>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}

          {/* Payment Buttons Configuration */}
          {activeSection === 'buttons' && (
            <div className={cn(
              "rounded-xl p-6 shadow-sm border",
              darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100"
            )}>
              <h3 className={cn(
                "flex items-center gap-2 text-lg font-semibold mb-4",
                darkMode ? "text-white" : "text-slate-800"
              )}>
                <CreditCard size={20} />
                Botones de Pago
              </h3>
              <p className={cn("text-sm mb-4", darkMode ? "text-slate-400" : "text-slate-500")}>
                Elige qué métodos de pago mostrar y personaliza el texto de los botones
              </p>

              <div className="space-y-4">
                {/* Card Button */}
                <div className={cn(
                  "p-4 rounded-lg border",
                  darkMode ? "border-slate-700" : "border-slate-200"
                )}>
                  <div className="flex items-center justify-between mb-3">
                    <div className="flex items-center gap-3">
                      <div className="w-10 h-10 bg-gradient-to-br from-slate-700 to-slate-900 rounded-lg flex items-center justify-center">
                        <CreditCard className="text-white" size={20} />
                      </div>
                      <div>
                        <p className={cn("font-medium", darkMode ? "text-white" : "text-slate-800")}>
                          Tarjeta de Crédito/Débito
                        </p>
                        <p className={cn("text-sm", darkMode ? "text-slate-400" : "text-slate-500")}>
                          Visa, Mastercard, Amex
                        </p>
                      </div>
                    </div>
                    <button
                      onClick={() => updateFormStyle({ showCardButton: !formStyle.showCardButton })}
                    >
                      {formStyle.showCardButton ? (
                        <Eye size={24} className="text-indigo-600" />
                      ) : (
                        <EyeOff size={24} className={cn(darkMode ? "text-slate-500" : "text-slate-400")} />
                      )}
                    </button>
                  </div>
                  {formStyle.showCardButton && (
                    <input
                      type="text"
                      value={formStyle.cardButtonText}
                      onChange={(e) => updateFormStyle({ cardButtonText: e.target.value })}
                      placeholder="Texto del botón..."
                      className={cn(
                        "w-full px-4 py-2 rounded-lg border",
                        darkMode 
                          ? "bg-slate-700 border-slate-600 text-white" 
                          : "bg-white border-slate-200"
                      )}
                    />
                  )}
                </div>

                {/* Stripe Button */}
                <div className={cn(
                  "p-4 rounded-lg border",
                  darkMode ? "border-slate-700" : "border-slate-200"
                )}>
                  <div className="flex items-center justify-between mb-3">
                    <div className="flex items-center gap-3">
                      <div className="w-10 h-10 bg-gradient-to-br from-indigo-500 to-purple-600 rounded-lg flex items-center justify-center">
                        <span className="text-white font-bold">S</span>
                      </div>
                      <div>
                        <p className={cn("font-medium", darkMode ? "text-white" : "text-slate-800")}>
                          Stripe
                        </p>
                        <p className={cn("text-sm", darkMode ? "text-slate-400" : "text-slate-500")}>
                          Checkout integrado
                        </p>
                      </div>
                    </div>
                    <button
                      onClick={() => updateFormStyle({ showStripeButton: !formStyle.showStripeButton })}
                    >
                      {formStyle.showStripeButton ? (
                        <Eye size={24} className="text-indigo-600" />
                      ) : (
                        <EyeOff size={24} className={cn(darkMode ? "text-slate-500" : "text-slate-400")} />
                      )}
                    </button>
                  </div>
                  {formStyle.showStripeButton && (
                    <input
                      type="text"
                      value={formStyle.stripeButtonText}
                      onChange={(e) => updateFormStyle({ stripeButtonText: e.target.value })}
                      placeholder="Texto del botón..."
                      className={cn(
                        "w-full px-4 py-2 rounded-lg border",
                        darkMode 
                          ? "bg-slate-700 border-slate-600 text-white" 
                          : "bg-white border-slate-200"
                      )}
                    />
                  )}
                </div>

                {/* PayPal Button */}
                <div className={cn(
                  "p-4 rounded-lg border",
                  darkMode ? "border-slate-700" : "border-slate-200"
                )}>
                  <div className="flex items-center justify-between mb-3">
                    <div className="flex items-center gap-3">
                      <div className="w-10 h-10 bg-gradient-to-br from-blue-500 to-blue-700 rounded-lg flex items-center justify-center">
                        <span className="text-white font-bold">P</span>
                      </div>
                      <div>
                        <p className={cn("font-medium", darkMode ? "text-white" : "text-slate-800")}>
                          PayPal
                        </p>
                        <p className={cn("text-sm", darkMode ? "text-slate-400" : "text-slate-500")}>
                          Cuenta PayPal o tarjeta
                        </p>
                      </div>
                    </div>
                    <button
                      onClick={() => updateFormStyle({ showPaypalButton: !formStyle.showPaypalButton })}
                    >
                      {formStyle.showPaypalButton ? (
                        <Eye size={24} className="text-indigo-600" />
                      ) : (
                        <EyeOff size={24} className={cn(darkMode ? "text-slate-500" : "text-slate-400")} />
                      )}
                    </button>
                  </div>
                  {formStyle.showPaypalButton && (
                    <input
                      type="text"
                      value={formStyle.paypalButtonText}
                      onChange={(e) => updateFormStyle({ paypalButtonText: e.target.value })}
                      placeholder="Texto del botón..."
                      className={cn(
                        "w-full px-4 py-2 rounded-lg border",
                        darkMode 
                          ? "bg-slate-700 border-slate-600 text-white" 
                          : "bg-white border-slate-200"
                      )}
                    />
                  )}
                </div>
              </div>
            </div>
          )}

          {/* Style Configuration */}
          {activeSection === 'style' && (
            <>
              {/* Company Info */}
              <div className={cn(
                "rounded-xl p-6 shadow-sm border",
                darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100"
              )}>
                <h3 className={cn(
                  "flex items-center gap-2 text-lg font-semibold mb-4",
                  darkMode ? "text-white" : "text-slate-800"
                )}>
                  <Image size={20} />
                  Información de la Empresa
                </h3>
                <div className="space-y-4">
                  <div>
                    <label className={cn(
                      "block text-sm font-medium mb-1",
                      darkMode ? "text-slate-300" : "text-slate-700"
                    )}>
                      Nombre de la empresa
                    </label>
                    <input
                      type="text"
                      value={formStyle.companyName}
                      onChange={(e) => updateFormStyle({ companyName: e.target.value })}
                      className={cn(
                        "w-full px-4 py-2 border rounded-lg",
                        darkMode 
                          ? "bg-slate-700 border-slate-600 text-white" 
                          : "bg-white border-slate-200"
                      )}
                    />
                  </div>
                  <div>
                    <label className={cn(
                      "block text-sm font-medium mb-1",
                      darkMode ? "text-slate-300" : "text-slate-700"
                    )}>
                      URL del Logo
                    </label>
                    <input
                      type="url"
                      value={formStyle.logoUrl}
                      onChange={(e) => updateFormStyle({ logoUrl: e.target.value })}
                      placeholder="https://ejemplo.com/logo.png"
                      className={cn(
                        "w-full px-4 py-2 border rounded-lg",
                        darkMode 
                          ? "bg-slate-700 border-slate-600 text-white placeholder:text-slate-500" 
                          : "bg-white border-slate-200"
                      )}
                    />
                  </div>
                  <label className="flex items-center gap-2 cursor-pointer">
                    <input
                      type="checkbox"
                      checked={formStyle.showLogo}
                      onChange={(e) => updateFormStyle({ showLogo: e.target.checked })}
                      className="rounded text-indigo-600"
                    />
                    <span className={cn("text-sm", darkMode ? "text-slate-300" : "text-slate-700")}>
                      Mostrar logo en el formulario
                    </span>
                  </label>
                </div>
              </div>

              {/* Colors */}
              <div className={cn(
                "rounded-xl p-6 shadow-sm border",
                darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100"
              )}>
                <h3 className={cn(
                  "flex items-center gap-2 text-lg font-semibold mb-4",
                  darkMode ? "text-white" : "text-slate-800"
                )}>
                  <Palette size={20} />
                  Colores
                </h3>
                
                <div className="mb-4">
                  <p className={cn("text-sm mb-3", darkMode ? "text-slate-400" : "text-slate-600")}>
                    Presets de colores
                  </p>
                  <div className="grid grid-cols-4 gap-2">
                    {colorPresets.map((preset) => (
                      <button
                        key={preset.name}
                        onClick={() => updateFormStyle({ 
                          primaryColor: preset.primary, 
                          backgroundColor: preset.bg,
                          textColor: preset.bg === '#ffffff' ? '#1f2937' : '#ffffff'
                        })}
                        className={cn(
                          "p-3 rounded-lg border-2 transition-colors group",
                          darkMode 
                            ? "border-slate-600 hover:border-slate-500" 
                            : "border-slate-200 hover:border-slate-300"
                        )}
                        title={preset.name}
                      >
                        <div className="flex gap-1 justify-center">
                          <div 
                            className="w-4 h-4 rounded-full" 
                            style={{ backgroundColor: preset.primary }}
                          ></div>
                          <div 
                            className="w-4 h-4 rounded-full border border-slate-200" 
                            style={{ backgroundColor: preset.bg }}
                          ></div>
                        </div>
                        <p className={cn(
                          "text-xs mt-1 text-center",
                          darkMode ? "text-slate-400" : "text-slate-500"
                        )}>{preset.name}</p>
                      </button>
                    ))}
                  </div>
                </div>
                
                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <label className={cn(
                      "block text-sm font-medium mb-1",
                      darkMode ? "text-slate-300" : "text-slate-700"
                    )}>
                      Color Principal
                    </label>
                    <div className="flex gap-2">
                      <input
                        type="color"
                        value={formStyle.primaryColor}
                        onChange={(e) => updateFormStyle({ primaryColor: e.target.value })}
                        className="w-12 h-10 rounded-lg cursor-pointer border-0"
                      />
                      <input
                        type="text"
                        value={formStyle.primaryColor}
                        onChange={(e) => updateFormStyle({ primaryColor: e.target.value })}
                        className={cn(
                          "flex-1 px-3 py-2 border rounded-lg font-mono text-sm",
                          darkMode 
                            ? "bg-slate-700 border-slate-600 text-white" 
                            : "bg-white border-slate-200"
                        )}
                      />
                    </div>
                  </div>
                  <div>
                    <label className={cn(
                      "block text-sm font-medium mb-1",
                      darkMode ? "text-slate-300" : "text-slate-700"
                    )}>
                      Color de Fondo
                    </label>
                    <div className="flex gap-2">
                      <input
                        type="color"
                        value={formStyle.backgroundColor}
                        onChange={(e) => updateFormStyle({ backgroundColor: e.target.value })}
                        className="w-12 h-10 rounded-lg cursor-pointer border-0"
                      />
                      <input
                        type="text"
                        value={formStyle.backgroundColor}
                        onChange={(e) => updateFormStyle({ backgroundColor: e.target.value })}
                        className={cn(
                          "flex-1 px-3 py-2 border rounded-lg font-mono text-sm",
                          darkMode 
                            ? "bg-slate-700 border-slate-600 text-white" 
                            : "bg-white border-slate-200"
                        )}
                      />
                    </div>
                  </div>
                </div>
                
                <div className="mt-4">
                  <label className={cn(
                    "block text-sm font-medium mb-1",
                    darkMode ? "text-slate-300" : "text-slate-700"
                  )}>
                    Color de Texto
                  </label>
                  <div className="flex gap-2">
                    <input
                      type="color"
                      value={formStyle.textColor}
                      onChange={(e) => updateFormStyle({ textColor: e.target.value })}
                      className="w-12 h-10 rounded-lg cursor-pointer border-0"
                    />
                    <input
                      type="text"
                      value={formStyle.textColor}
                      onChange={(e) => updateFormStyle({ textColor: e.target.value })}
                      className={cn(
                        "flex-1 px-3 py-2 border rounded-lg font-mono text-sm",
                        darkMode 
                          ? "bg-slate-700 border-slate-600 text-white" 
                          : "bg-white border-slate-200"
                      )}
                    />
                  </div>
                </div>
              </div>

              {/* Typography & Style */}
              <div className={cn(
                "rounded-xl p-6 shadow-sm border",
                darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100"
              )}>
                <h3 className={cn(
                  "flex items-center gap-2 text-lg font-semibold mb-4",
                  darkMode ? "text-white" : "text-slate-800"
                )}>
                  <Type size={20} />
                  Tipografía y Estilo
                </h3>
                
                <div className="space-y-4">
                  <div>
                    <label className={cn(
                      "block text-sm font-medium mb-1",
                      darkMode ? "text-slate-300" : "text-slate-700"
                    )}>
                      Fuente
                    </label>
                    <select
                      value={formStyle.fontFamily}
                      onChange={(e) => updateFormStyle({ fontFamily: e.target.value })}
                      className={cn(
                        "w-full px-4 py-2 border rounded-lg",
                        darkMode 
                          ? "bg-slate-700 border-slate-600 text-white" 
                          : "bg-white border-slate-200"
                      )}
                    >
                      {fontOptions.map((font) => (
                        <option key={font.value} value={font.value}>{font.name}</option>
                      ))}
                    </select>
                  </div>
                  
                  <div>
                    <label className={cn(
                      "block text-sm font-medium mb-2",
                      darkMode ? "text-slate-300" : "text-slate-700"
                    )}>
                      <Square size={16} className="inline mr-1" />
                      Bordes redondeados
                    </label>
                    <div className="grid grid-cols-3 gap-2">
                      {borderRadiusOptions.map((option) => (
                        <button
                          key={option.value}
                          onClick={() => updateFormStyle({ borderRadius: option.value })}
                          className={cn(
                            "px-3 py-2 text-sm font-medium border-2 transition-all",
                            getBorderRadiusClass(option.value),
                            formStyle.borderRadius === option.value
                              ? "border-indigo-500 bg-indigo-50 text-indigo-700"
                              : darkMode
                                ? "border-slate-600 text-slate-300 hover:border-slate-500"
                                : "border-slate-200 text-slate-600 hover:border-slate-300"
                          )}
                        >
                          {option.label}
                        </button>
                      ))}
                    </div>
                  </div>
                  
                  <div>
                    <label className={cn(
                      "block text-sm font-medium mb-2",
                      darkMode ? "text-slate-300" : "text-slate-700"
                    )}>
                      Estilo del botón
                    </label>
                    <div className="grid grid-cols-3 gap-2">
                      {(['solid', 'outline', 'gradient'] as const).map((style) => (
                        <button
                          key={style}
                          onClick={() => updateFormStyle({ buttonStyle: style })}
                          className={cn(
                            "px-4 py-2 text-sm font-medium rounded-lg transition-all",
                            formStyle.buttonStyle === style
                              ? style === 'solid' 
                                ? "bg-indigo-600 text-white"
                                : style === 'outline'
                                ? "border-2 border-indigo-600 text-indigo-600 bg-white"
                                : "bg-gradient-to-r from-indigo-600 to-purple-600 text-white"
                              : darkMode
                                ? "bg-slate-700 text-slate-300 hover:bg-slate-600"
                                : "bg-slate-100 text-slate-600 hover:bg-slate-200"
                          )}
                        >
                          {style === 'solid' ? 'Sólido' : style === 'outline' ? 'Contorno' : 'Gradiente'}
                        </button>
                      ))}
                    </div>
                  </div>
                </div>
              </div>
            </>
          )}

          {/* Invoice Configuration */}
          {activeSection === 'invoice' && (
            <div className={cn(
              "rounded-xl p-6 shadow-sm border",
              darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100"
            )}>
              <h3 className={cn(
                "flex items-center gap-2 text-lg font-semibold mb-4",
                darkMode ? "text-white" : "text-slate-800"
              )}>
                <Mail size={20} />
                Configuración de Facturas
              </h3>
              <p className={cn("text-sm mb-4", darkMode ? "text-slate-400" : "text-slate-500")}>
                Envía automáticamente una factura/recibo por email después de cada pago
              </p>

              <div className="space-y-4">
                <label className="flex items-center gap-3 cursor-pointer">
                  <button
                    onClick={() => updateFormStyle({ sendInvoiceEmail: !formStyle.sendInvoiceEmail })}
                    className={cn(
                      "w-12 h-7 rounded-full relative transition-colors",
                      formStyle.sendInvoiceEmail ? "bg-indigo-600" : darkMode ? "bg-slate-600" : "bg-slate-300"
                    )}
                  >
                    <div className={cn(
                      "absolute top-1 w-5 h-5 bg-white rounded-full transition-transform shadow",
                      formStyle.sendInvoiceEmail ? "translate-x-6" : "translate-x-1"
                    )}></div>
                  </button>
                  <span className={cn("font-medium", darkMode ? "text-white" : "text-slate-700")}>
                    Enviar factura automáticamente por email
                  </span>
                </label>

                {formStyle.sendInvoiceEmail && (
                  <>
                    <div>
                      <label className={cn(
                        "block text-sm font-medium mb-1",
                        darkMode ? "text-slate-300" : "text-slate-700"
                      )}>
                        Nombre del remitente
                      </label>
                      <input
                        type="text"
                        value={formStyle.invoiceFromName}
                        onChange={(e) => updateFormStyle({ invoiceFromName: e.target.value })}
                        placeholder="Mi Empresa"
                        className={cn(
                          "w-full px-4 py-2 border rounded-lg",
                          darkMode 
                            ? "bg-slate-700 border-slate-600 text-white" 
                            : "bg-white border-slate-200"
                        )}
                      />
                    </div>
                    <div>
                      <label className={cn(
                        "block text-sm font-medium mb-1",
                        darkMode ? "text-slate-300" : "text-slate-700"
                      )}>
                        Email del remitente
                      </label>
                      <input
                        type="email"
                        value={formStyle.invoiceFromEmail}
                        onChange={(e) => updateFormStyle({ invoiceFromEmail: e.target.value })}
                        placeholder="pagos@miempresa.com"
                        className={cn(
                          "w-full px-4 py-2 border rounded-lg",
                          darkMode 
                            ? "bg-slate-700 border-slate-600 text-white" 
                            : "bg-white border-slate-200"
                        )}
                      />
                    </div>

                    {/* Invoice Preview */}
                    <div className={cn(
                      "mt-6 p-4 rounded-lg border-2 border-dashed",
                      darkMode ? "border-slate-600" : "border-slate-300"
                    )}>
                      <p className={cn(
                        "text-sm font-medium mb-3 text-center",
                        darkMode ? "text-slate-400" : "text-slate-500"
                      )}>
                        Vista previa del email de factura
                      </p>
                      <div className="bg-white rounded-lg p-4 text-slate-800">
                        <div className="border-b pb-4 mb-4">
                          <div className="flex items-center justify-between">
                            <div>
                              <h4 className="font-bold text-lg">{formStyle.companyName}</h4>
                              <p className="text-sm text-slate-500">{formStyle.invoiceFromEmail}</p>
                            </div>
                            <div className="text-right">
                              <p className="text-sm font-medium">FACTURA</p>
                              <p className="text-xs text-slate-500">#INV-XXXXXX</p>
                            </div>
                          </div>
                        </div>
                        <div className="space-y-2 text-sm">
                          <div className="flex justify-between">
                            <span className="text-slate-500">Cliente:</span>
                            <span className="font-medium">Nombre del Cliente</span>
                          </div>
                          <div className="flex justify-between">
                            <span className="text-slate-500">Producto:</span>
                            <span className="font-medium">Plan Premium</span>
                          </div>
                          <div className="flex justify-between">
                            <span className="text-slate-500">Fecha:</span>
                            <span className="font-medium">{new Date().toLocaleDateString('es-ES')}</span>
                          </div>
                          <div className="border-t pt-2 mt-2">
                            <div className="flex justify-between font-bold">
                              <span>Total:</span>
                              <span style={{ color: formStyle.primaryColor }}>$29.99 USD</span>
                            </div>
                          </div>
                        </div>
                        <div className="mt-4 pt-4 border-t text-center">
                          <p className="text-xs text-slate-400">
                            ¡Gracias por tu compra! Este recibo confirma tu pago.
                          </p>
                        </div>
                      </div>
                    </div>
                  </>
                )}
              </div>
            </div>
          )}

          {/* Actions */}
          <div className="flex items-center gap-4">
            <button
              onClick={handleSave}
              className="flex items-center gap-2 px-6 py-3 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors font-medium"
            >
              <Save size={20} />
              Guardar Todo
            </button>
            <button
              onClick={handleReset}
              className={cn(
                "flex items-center gap-2 px-6 py-3 border rounded-lg transition-colors font-medium",
                darkMode 
                  ? "border-slate-600 text-slate-300 hover:bg-slate-700" 
                  : "border-slate-200 text-slate-600 hover:bg-slate-50"
              )}
            >
              <RotateCcw size={20} />
              Restablecer
            </button>
            {saved && (
              <span className="flex items-center gap-2 text-green-600 font-medium">
                <CheckCircle size={20} />
                Guardado
              </span>
            )}
          </div>
        </div>

        {/* Preview Panel */}
        <div className="lg:sticky lg:top-6 h-fit">
          <div className={cn(
            "rounded-xl p-4",
            darkMode ? "bg-slate-800" : "bg-slate-100"
          )}>
            <p className={cn(
              "text-sm mb-3 text-center",
              darkMode ? "text-slate-400" : "text-slate-500"
            )}>Vista previa del formulario</p>
            <div 
              className={cn("p-8 shadow-lg", getBorderRadiusClass(formStyle.borderRadius))}
              style={{ 
                backgroundColor: formStyle.backgroundColor,
                fontFamily: formStyle.fontFamily,
              }}
            >
              {/* Header */}
              <div className="text-center mb-6">
                {formStyle.showLogo && (
                  formStyle.logoUrl ? (
                    <img 
                      src={formStyle.logoUrl} 
                      alt="Logo" 
                      className="h-12 mx-auto mb-4 object-contain"
                      onError={(e) => {
                        (e.target as HTMLImageElement).style.display = 'none';
                      }}
                    />
                  ) : (
                    <div 
                      className={cn("w-12 h-12 mx-auto mb-4 flex items-center justify-center", getBorderRadiusClass(formStyle.borderRadius))}
                      style={{ backgroundColor: formStyle.primaryColor }}
                    >
                      <span className="text-white font-bold text-xl">
                        {formStyle.companyName.charAt(0)}
                      </span>
                    </div>
                  )
                )}
                <h2 
                  className="text-xl font-bold"
                  style={{ color: formStyle.textColor }}
                >
                  {formStyle.companyName}
                </h2>
              </div>

              {/* Dynamic Form Fields */}
              <div className="space-y-3">
                <div className="grid grid-cols-2 gap-3">
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
                          placeholder={field.placeholder}
                          rows={2}
                          className={cn("w-full px-4 py-2 border", getBorderRadiusClass(formStyle.borderRadius))}
                          style={{ 
                            borderColor: `${formStyle.primaryColor}40`,
                            color: formStyle.textColor,
                          }}
                          readOnly
                        />
                      ) : (
                        <input
                          type={field.type === 'email' ? 'email' : field.type === 'phone' ? 'tel' : 'text'}
                          placeholder={field.placeholder}
                          className={cn("w-full px-4 py-2 border", getBorderRadiusClass(formStyle.borderRadius))}
                          style={{ 
                            borderColor: `${formStyle.primaryColor}40`,
                            color: formStyle.textColor,
                          }}
                          readOnly
                        />
                      )}
                    </div>
                  ))}
                </div>
              </div>

              {/* Payment Buttons */}
              <div className="mt-6 space-y-3">
                {formStyle.showCardButton && (
                  <button
                    className={cn(
                      "w-full py-3 font-semibold transition-all flex items-center justify-center gap-2",
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
                    <CreditCard size={18} />
                    {formStyle.cardButtonText}
                  </button>
                )}
                {formStyle.showStripeButton && (
                  <button
                    className={cn(
                      "w-full py-3 font-semibold bg-indigo-600 text-white hover:bg-indigo-700 transition-all flex items-center justify-center gap-2",
                      getBorderRadiusClass(formStyle.borderRadius)
                    )}
                  >
                    <span className="font-bold">S</span>
                    {formStyle.stripeButtonText}
                  </button>
                )}
                {formStyle.showPaypalButton && (
                  <button
                    className={cn(
                      "w-full py-3 font-semibold bg-[#0070ba] text-white hover:bg-[#005ea6] transition-all flex items-center justify-center gap-2",
                      getBorderRadiusClass(formStyle.borderRadius)
                    )}
                  >
                    <span className="font-bold">Pay</span>
                    <span className="font-bold text-[#003087]">Pal</span>
                  </button>
                )}
              </div>

              {/* Footer */}
              <p 
                className="text-center text-xs mt-6 opacity-50"
                style={{ color: formStyle.textColor }}
              >
                Pago seguro • Encriptación SSL
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
