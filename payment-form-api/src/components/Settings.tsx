import { useState } from 'react';
import { Eye, EyeOff, Save, ExternalLink, AlertCircle, CheckCircle } from 'lucide-react';
import { useApp } from '../context/AppContext';
import { cn } from '../utils/cn';

export function Settings() {
  const { paymentConfig, updatePaymentConfig, darkMode } = useApp();
  const [showStripeSecret, setShowStripeSecret] = useState(false);
  const [showPaypalSecret, setShowPaypalSecret] = useState(false);
  const [saved, setSaved] = useState(false);
  
  const [formData, setFormData] = useState({
    stripeEnabled: paymentConfig.stripeEnabled,
    stripePublicKey: paymentConfig.stripePublicKey,
    stripeSecretKey: paymentConfig.stripeSecretKey,
    paypalEnabled: paymentConfig.paypalEnabled,
    paypalClientId: paymentConfig.paypalClientId,
    paypalClientSecret: paymentConfig.paypalClientSecret,
    paypalSandbox: paymentConfig.paypalSandbox,
  });

  const handleSave = () => {
    updatePaymentConfig(formData);
    setSaved(true);
    setTimeout(() => setSaved(false), 3000);
  };

  return (
    <div className="p-6 space-y-6 max-w-4xl">
      <div>
        <h2 className={cn("text-2xl font-bold", darkMode ? "text-white" : "text-slate-800")}>
          Configuración de APIs de Pago
        </h2>
        <p className={cn(darkMode ? "text-slate-400" : "text-slate-500")}>
          Conecta tus cuentas de Stripe y PayPal para procesar pagos
        </p>
      </div>

      {/* Alert */}
      <div className={cn(
        "border rounded-xl p-4 flex gap-3",
        darkMode ? "bg-amber-900/20 border-amber-700" : "bg-amber-50 border-amber-200"
      )}>
        <AlertCircle className="text-amber-500 flex-shrink-0 mt-0.5" size={20} />
        <div>
          <p className={cn("font-medium", darkMode ? "text-amber-400" : "text-amber-800")}>
            Importante sobre seguridad
          </p>
          <p className={cn("text-sm mt-1", darkMode ? "text-amber-300/80" : "text-amber-700")}>
            En un entorno de producción, las claves secretas deben guardarse de forma segura en el servidor. 
            Esta interfaz es solo para demostración y configuración inicial.
          </p>
        </div>
      </div>

      {/* Stripe Configuration */}
      <div className={cn(
        "rounded-xl shadow-sm border overflow-hidden",
        darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100"
      )}>
        <div className={cn(
          "p-6 border-b",
          darkMode ? "border-slate-700" : "border-slate-100"
        )}>
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-4">
              <div className={cn(
                "w-12 h-12 rounded-xl flex items-center justify-center",
                darkMode ? "bg-indigo-900/50" : "bg-indigo-100"
              )}>
                <span className="text-2xl font-bold text-indigo-500">S</span>
              </div>
              <div>
                <h3 className={cn("text-lg font-semibold", darkMode ? "text-white" : "text-slate-800")}>
                  Stripe
                </h3>
                <p className={cn("text-sm", darkMode ? "text-slate-400" : "text-slate-500")}>
                  Procesa pagos con tarjeta de crédito/débito
                </p>
              </div>
            </div>
            <label className="relative inline-flex items-center cursor-pointer">
              <input
                type="checkbox"
                checked={formData.stripeEnabled}
                onChange={(e) => setFormData({ ...formData, stripeEnabled: e.target.checked })}
                className="sr-only peer"
              />
              <div className={cn(
                "w-11 h-6 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-indigo-600",
                darkMode ? "bg-slate-600" : "bg-slate-200"
              )}></div>
            </label>
          </div>
        </div>
        
        {formData.stripeEnabled && (
          <div className={cn(
            "p-6 space-y-4",
            darkMode ? "bg-slate-700/50" : "bg-slate-50"
          )}>
            <div>
              <label className={cn(
                "block text-sm font-medium mb-1",
                darkMode ? "text-slate-300" : "text-slate-700"
              )}>
                Clave Pública (Publishable Key)
              </label>
              <input
                type="text"
                value={formData.stripePublicKey}
                onChange={(e) => setFormData({ ...formData, stripePublicKey: e.target.value })}
                placeholder="pk_live_..."
                className={cn(
                  "w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent font-mono text-sm",
                  darkMode 
                    ? "bg-slate-600 border-slate-500 text-white placeholder:text-slate-400" 
                    : "bg-white border-slate-200"
                )}
              />
            </div>
            <div>
              <label className={cn(
                "block text-sm font-medium mb-1",
                darkMode ? "text-slate-300" : "text-slate-700"
              )}>
                Clave Secreta (Secret Key)
              </label>
              <div className="relative">
                <input
                  type={showStripeSecret ? 'text' : 'password'}
                  value={formData.stripeSecretKey}
                  onChange={(e) => setFormData({ ...formData, stripeSecretKey: e.target.value })}
                  placeholder="sk_live_..."
                  className={cn(
                    "w-full px-4 py-2 pr-12 border rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent font-mono text-sm",
                    darkMode 
                      ? "bg-slate-600 border-slate-500 text-white placeholder:text-slate-400" 
                      : "bg-white border-slate-200"
                  )}
                />
                <button
                  type="button"
                  onClick={() => setShowStripeSecret(!showStripeSecret)}
                  className={cn(
                    "absolute right-3 top-1/2 -translate-y-1/2",
                    darkMode ? "text-slate-400 hover:text-slate-300" : "text-slate-400 hover:text-slate-600"
                  )}
                >
                  {showStripeSecret ? <EyeOff size={20} /> : <Eye size={20} />}
                </button>
              </div>
            </div>
            <a
              href="https://dashboard.stripe.com/apikeys"
              target="_blank"
              rel="noopener noreferrer"
              className="inline-flex items-center gap-2 text-sm text-indigo-500 hover:underline"
            >
              Obtener claves de Stripe
              <ExternalLink size={14} />
            </a>
          </div>
        )}
      </div>

      {/* PayPal Configuration */}
      <div className={cn(
        "rounded-xl shadow-sm border overflow-hidden",
        darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100"
      )}>
        <div className={cn(
          "p-6 border-b",
          darkMode ? "border-slate-700" : "border-slate-100"
        )}>
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-4">
              <div className={cn(
                "w-12 h-12 rounded-xl flex items-center justify-center",
                darkMode ? "bg-blue-900/50" : "bg-blue-100"
              )}>
                <span className="text-2xl font-bold text-blue-500">P</span>
              </div>
              <div>
                <h3 className={cn("text-lg font-semibold", darkMode ? "text-white" : "text-slate-800")}>
                  PayPal
                </h3>
                <p className={cn("text-sm", darkMode ? "text-slate-400" : "text-slate-500")}>
                  Acepta pagos con cuenta PayPal
                </p>
              </div>
            </div>
            <label className="relative inline-flex items-center cursor-pointer">
              <input
                type="checkbox"
                checked={formData.paypalEnabled}
                onChange={(e) => setFormData({ ...formData, paypalEnabled: e.target.checked })}
                className="sr-only peer"
              />
              <div className={cn(
                "w-11 h-6 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-600",
                darkMode ? "bg-slate-600" : "bg-slate-200"
              )}></div>
            </label>
          </div>
        </div>
        
        {formData.paypalEnabled && (
          <div className={cn(
            "p-6 space-y-4",
            darkMode ? "bg-slate-700/50" : "bg-slate-50"
          )}>
            <div className="flex items-center gap-4 mb-4">
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="radio"
                  name="paypalMode"
                  checked={formData.paypalSandbox}
                  onChange={() => setFormData({ ...formData, paypalSandbox: true })}
                  className="text-blue-600"
                />
                <span className={cn("text-sm", darkMode ? "text-slate-300" : "text-slate-700")}>
                  Modo Sandbox (Pruebas)
                </span>
              </label>
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="radio"
                  name="paypalMode"
                  checked={!formData.paypalSandbox}
                  onChange={() => setFormData({ ...formData, paypalSandbox: false })}
                  className="text-blue-600"
                />
                <span className={cn("text-sm", darkMode ? "text-slate-300" : "text-slate-700")}>
                  Modo Producción
                </span>
              </label>
            </div>
            
            <div>
              <label className={cn(
                "block text-sm font-medium mb-1",
                darkMode ? "text-slate-300" : "text-slate-700"
              )}>
                Client ID
              </label>
              <input
                type="text"
                value={formData.paypalClientId}
                onChange={(e) => setFormData({ ...formData, paypalClientId: e.target.value })}
                placeholder="Tu Client ID de PayPal"
                className={cn(
                  "w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent font-mono text-sm",
                  darkMode 
                    ? "bg-slate-600 border-slate-500 text-white placeholder:text-slate-400" 
                    : "bg-white border-slate-200"
                )}
              />
            </div>
            <div>
              <label className={cn(
                "block text-sm font-medium mb-1",
                darkMode ? "text-slate-300" : "text-slate-700"
              )}>
                Client Secret
              </label>
              <div className="relative">
                <input
                  type={showPaypalSecret ? 'text' : 'password'}
                  value={formData.paypalClientSecret}
                  onChange={(e) => setFormData({ ...formData, paypalClientSecret: e.target.value })}
                  placeholder="Tu Client Secret de PayPal"
                  className={cn(
                    "w-full px-4 py-2 pr-12 border rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent font-mono text-sm",
                    darkMode 
                      ? "bg-slate-600 border-slate-500 text-white placeholder:text-slate-400" 
                      : "bg-white border-slate-200"
                  )}
                />
                <button
                  type="button"
                  onClick={() => setShowPaypalSecret(!showPaypalSecret)}
                  className={cn(
                    "absolute right-3 top-1/2 -translate-y-1/2",
                    darkMode ? "text-slate-400 hover:text-slate-300" : "text-slate-400 hover:text-slate-600"
                  )}
                >
                  {showPaypalSecret ? <EyeOff size={20} /> : <Eye size={20} />}
                </button>
              </div>
            </div>
            <a
              href="https://developer.paypal.com/dashboard/applications"
              target="_blank"
              rel="noopener noreferrer"
              className="inline-flex items-center gap-2 text-sm text-blue-500 hover:underline"
            >
              Obtener credenciales de PayPal
              <ExternalLink size={14} />
            </a>
          </div>
        )}
      </div>

      {/* Save Button */}
      <div className="flex items-center gap-4">
        <button
          onClick={handleSave}
          className="flex items-center gap-2 px-6 py-3 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors font-medium"
        >
          <Save size={20} />
          Guardar Configuración
        </button>
        {saved && (
          <span className="flex items-center gap-2 text-green-500 font-medium">
            <CheckCircle size={20} />
            Configuración guardada
          </span>
        )}
      </div>
    </div>
  );
}
