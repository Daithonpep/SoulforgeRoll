import { useState } from 'react';
import { Plus, Pencil, Trash2, X, Ticket, Copy, Check } from 'lucide-react';
import { useApp } from '../context/AppContext';
import { DiscountCode } from '../types';
import { cn } from '../utils/cn';

export function DiscountCodes() {
  const { discountCodes, addDiscountCode, updateDiscountCode, deleteDiscountCode, products, darkMode } = useApp();
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [editingCode, setEditingCode] = useState<DiscountCode | null>(null);
  const [copiedCode, setCopiedCode] = useState<string | null>(null);
  const [formData, setFormData] = useState({
    code: '',
    creatorName: '',
    discountType: 'percentage' as 'percentage' | 'fixed',
    discountValue: '',
    maxUses: '',
    expiresAt: '',
    active: true,
    applicableProducts: [] as string[],
  });

  const openModal = (code?: DiscountCode) => {
    if (code) {
      setEditingCode(code);
      setFormData({
        code: code.code,
        creatorName: code.creatorName,
        discountType: code.discountType,
        discountValue: code.discountValue.toString(),
        maxUses: code.maxUses?.toString() || '',
        expiresAt: code.expiresAt ? new Date(code.expiresAt).toISOString().split('T')[0] : '',
        active: code.active,
        applicableProducts: code.applicableProducts,
      });
    } else {
      setEditingCode(null);
      setFormData({
        code: '',
        creatorName: '',
        discountType: 'percentage',
        discountValue: '',
        maxUses: '',
        expiresAt: '',
        active: true,
        applicableProducts: [],
      });
    }
    setIsModalOpen(true);
  };

  const closeModal = () => {
    setIsModalOpen(false);
    setEditingCode(null);
  };

  const generateCode = () => {
    const name = formData.creatorName.split(' ')[0].toUpperCase().slice(0, 6);
    const discount = formData.discountValue || '00';
    const code = `${name}${discount}`;
    setFormData({ ...formData, code });
  };

  const copyToClipboard = (code: string) => {
    navigator.clipboard.writeText(code);
    setCopiedCode(code);
    setTimeout(() => setCopiedCode(null), 2000);
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const codeData = {
      code: formData.code.toUpperCase(),
      creatorName: formData.creatorName,
      discountType: formData.discountType,
      discountValue: parseFloat(formData.discountValue),
      maxUses: formData.maxUses ? parseInt(formData.maxUses) : null,
      expiresAt: formData.expiresAt ? new Date(formData.expiresAt) : null,
      active: formData.active,
      applicableProducts: formData.applicableProducts,
    };

    if (editingCode) {
      updateDiscountCode(editingCode.id, codeData);
    } else {
      addDiscountCode(codeData);
    }
    closeModal();
  };

  return (
    <div className="p-6 space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h2 className={cn("text-2xl font-bold", darkMode ? "text-white" : "text-slate-800")}>
            Códigos de Descuento
          </h2>
          <p className={cn(darkMode ? "text-slate-400" : "text-slate-500")}>
            Crea códigos personalizados para creadores de contenido y afiliados
          </p>
        </div>
        <button
          onClick={() => openModal()}
          className="flex items-center gap-2 px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors"
        >
          <Plus size={20} />
          Nuevo Código
        </button>
      </div>

      {/* Info Card */}
      <div className="bg-gradient-to-r from-indigo-500 to-purple-600 rounded-xl p-6 text-white">
        <h3 className="text-lg font-semibold mb-2">💡 Sistema de Afiliados</h3>
        <p className="text-indigo-100">
          Crea códigos únicos para cada creador de contenido. Cuando sus seguidores usen el código, 
          obtendrán un descuento y podrás rastrear las conversiones de cada afiliado.
        </p>
      </div>

      {/* Codes Table */}
      <div className={cn(
        "rounded-xl shadow-sm border overflow-hidden",
        darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100"
      )}>
        <div className="overflow-x-auto">
          <table className="w-full">
            <thead className={cn(darkMode ? "bg-slate-700/50" : "bg-slate-50")}>
              <tr className={cn(
                "text-left text-sm",
                darkMode ? "text-slate-400" : "text-slate-500"
              )}>
                <th className="px-6 py-4 font-medium">Código</th>
                <th className="px-6 py-4 font-medium">Creador/Afiliado</th>
                <th className="px-6 py-4 font-medium">Descuento</th>
                <th className="px-6 py-4 font-medium">Usos</th>
                <th className="px-6 py-4 font-medium">Expira</th>
                <th className="px-6 py-4 font-medium">Estado</th>
                <th className="px-6 py-4 font-medium">Acciones</th>
              </tr>
            </thead>
            <tbody className={cn("divide-y", darkMode ? "divide-slate-700" : "divide-slate-100")}>
              {discountCodes.map((code) => (
                <tr key={code.id} className={cn(
                  "transition-colors",
                  darkMode ? "hover:bg-slate-700/50" : "hover:bg-slate-50"
                )}>
                  <td className="px-6 py-4">
                    <div className="flex items-center gap-2">
                      <span className={cn(
                        "px-3 py-1.5 rounded-lg font-mono font-bold text-sm",
                        darkMode ? "bg-indigo-900/50 text-indigo-400" : "bg-indigo-50 text-indigo-700"
                      )}>
                        {code.code}
                      </span>
                      <button
                        onClick={() => copyToClipboard(code.code)}
                        className={cn(
                          "p-1.5 rounded-lg transition-colors",
                          darkMode ? "hover:bg-slate-600" : "hover:bg-slate-100"
                        )}
                        title="Copiar código"
                      >
                        {copiedCode === code.code ? (
                          <Check size={16} className="text-green-500" />
                        ) : (
                          <Copy size={16} className={cn(darkMode ? "text-slate-500" : "text-slate-400")} />
                        )}
                      </button>
                    </div>
                  </td>
                  <td className="px-6 py-4">
                    <div className="flex items-center gap-3">
                      <div className="w-8 h-8 bg-gradient-to-br from-indigo-400 to-purple-500 rounded-full flex items-center justify-center text-white text-sm font-medium">
                        {code.creatorName.charAt(0)}
                      </div>
                      <span className={cn("font-medium", darkMode ? "text-white" : "text-slate-700")}>
                        {code.creatorName}
                      </span>
                    </div>
                  </td>
                  <td className="px-6 py-4">
                    <span className="text-lg font-semibold text-green-500">
                      {code.discountType === 'percentage' ? `${code.discountValue}%` : `$${code.discountValue}`}
                    </span>
                  </td>
                  <td className="px-6 py-4">
                    <div className="flex items-center gap-2">
                      <span className={cn("font-medium", darkMode ? "text-white" : "text-slate-700")}>
                        {code.currentUses}
                      </span>
                      {code.maxUses && (
                        <>
                          <span className={cn(darkMode ? "text-slate-500" : "text-slate-400")}>/</span>
                          <span className={cn(darkMode ? "text-slate-400" : "text-slate-500")}>
                            {code.maxUses}
                          </span>
                        </>
                      )}
                    </div>
                    {code.maxUses && (
                      <div className={cn(
                        "w-24 h-1.5 rounded-full mt-1",
                        darkMode ? "bg-slate-600" : "bg-slate-100"
                      )}>
                        <div 
                          className="h-full bg-indigo-500 rounded-full"
                          style={{ width: `${Math.min((code.currentUses / code.maxUses) * 100, 100)}%` }}
                        ></div>
                      </div>
                    )}
                  </td>
                  <td className={cn("px-6 py-4 text-sm", darkMode ? "text-slate-400" : "text-slate-500")}>
                    {code.expiresAt 
                      ? new Date(code.expiresAt).toLocaleDateString('es-ES')
                      : 'Sin expiración'
                    }
                  </td>
                  <td className="px-6 py-4">
                    <span className={cn(
                      "px-3 py-1 rounded-full text-xs font-medium",
                      code.active 
                        ? "bg-green-100 text-green-700" 
                        : darkMode ? "bg-slate-600 text-slate-300" : "bg-slate-100 text-slate-500"
                    )}>
                      {code.active ? 'Activo' : 'Inactivo'}
                    </span>
                  </td>
                  <td className="px-6 py-4">
                    <div className="flex items-center gap-1">
                      <button
                        onClick={() => openModal(code)}
                        className={cn(
                          "p-2 rounded-lg transition-colors",
                          darkMode ? "hover:bg-slate-600" : "hover:bg-slate-100"
                        )}
                      >
                        <Pencil size={16} className={cn(darkMode ? "text-slate-400" : "text-slate-500")} />
                      </button>
                      <button
                        onClick={() => deleteDiscountCode(code.id)}
                        className="p-2 hover:bg-red-50 rounded-lg transition-colors"
                      >
                        <Trash2 size={16} className="text-red-500" />
                      </button>
                    </div>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>

        {discountCodes.length === 0 && (
          <div className="text-center py-12">
            <Ticket className={cn("mx-auto", darkMode ? "text-slate-600" : "text-slate-300")} size={48} />
            <p className={cn("mt-4", darkMode ? "text-slate-400" : "text-slate-500")}>
              No hay códigos de descuento
            </p>
            <button
              onClick={() => openModal()}
              className="mt-4 text-indigo-500 hover:underline font-medium"
            >
              Crear el primero
            </button>
          </div>
        )}
      </div>

      {/* Modal */}
      {isModalOpen && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
          <div className={cn(
            "rounded-2xl w-full max-w-lg shadow-xl max-h-[90vh] overflow-y-auto",
            darkMode ? "bg-slate-800" : "bg-white"
          )}>
            <div className={cn(
              "flex items-center justify-between p-6 border-b sticky top-0",
              darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-200"
            )}>
              <h3 className={cn("text-xl font-semibold", darkMode ? "text-white" : "text-slate-800")}>
                {editingCode ? 'Editar Código' : 'Nuevo Código de Descuento'}
              </h3>
              <button 
                onClick={closeModal} 
                className={cn(
                  "p-2 rounded-lg",
                  darkMode ? "hover:bg-slate-700" : "hover:bg-slate-100"
                )}
              >
                <X size={20} className={cn(darkMode ? "text-slate-400" : "text-slate-500")} />
              </button>
            </div>
            
            <form onSubmit={handleSubmit} className="p-6 space-y-4">
              <div>
                <label className={cn(
                  "block text-sm font-medium mb-1",
                  darkMode ? "text-slate-300" : "text-slate-700"
                )}>
                  Nombre del Creador/Afiliado
                </label>
                <input
                  type="text"
                  value={formData.creatorName}
                  onChange={(e) => setFormData({ ...formData, creatorName: e.target.value })}
                  placeholder="Ej: Juan Pérez"
                  className={cn(
                    "w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent",
                    darkMode 
                      ? "bg-slate-700 border-slate-600 text-white placeholder:text-slate-500" 
                      : "bg-white border-slate-200"
                  )}
                  required
                />
              </div>
              
              <div>
                <label className={cn(
                  "block text-sm font-medium mb-1",
                  darkMode ? "text-slate-300" : "text-slate-700"
                )}>
                  Código de Descuento
                </label>
                <div className="flex gap-2">
                  <input
                    type="text"
                    value={formData.code}
                    onChange={(e) => setFormData({ ...formData, code: e.target.value.toUpperCase() })}
                    placeholder="Ej: JUAN20"
                    className={cn(
                      "flex-1 px-4 py-2 border rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent font-mono uppercase",
                      darkMode 
                        ? "bg-slate-700 border-slate-600 text-white placeholder:text-slate-500" 
                        : "bg-white border-slate-200"
                    )}
                    required
                  />
                  <button
                    type="button"
                    onClick={generateCode}
                    className={cn(
                      "px-4 py-2 rounded-lg text-sm font-medium transition-colors",
                      darkMode 
                        ? "bg-slate-700 text-slate-300 hover:bg-slate-600" 
                        : "bg-slate-100 text-slate-700 hover:bg-slate-200"
                    )}
                  >
                    Generar
                  </button>
                </div>
                <p className={cn("text-xs mt-1", darkMode ? "text-slate-500" : "text-slate-500")}>
                  El código se generará automáticamente basado en el nombre
                </p>
              </div>
              
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className={cn(
                    "block text-sm font-medium mb-1",
                    darkMode ? "text-slate-300" : "text-slate-700"
                  )}>
                    Tipo de descuento
                  </label>
                  <select
                    value={formData.discountType}
                    onChange={(e) => setFormData({ ...formData, discountType: e.target.value as 'percentage' | 'fixed' })}
                    className={cn(
                      "w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent",
                      darkMode 
                        ? "bg-slate-700 border-slate-600 text-white" 
                        : "bg-white border-slate-200"
                    )}
                  >
                    <option value="percentage">Porcentaje (%)</option>
                    <option value="fixed">Monto fijo ($)</option>
                  </select>
                </div>
                <div>
                  <label className={cn(
                    "block text-sm font-medium mb-1",
                    darkMode ? "text-slate-300" : "text-slate-700"
                  )}>
                    Valor del descuento
                  </label>
                  <input
                    type="number"
                    value={formData.discountValue}
                    onChange={(e) => setFormData({ ...formData, discountValue: e.target.value })}
                    placeholder={formData.discountType === 'percentage' ? '20' : '10.00'}
                    className={cn(
                      "w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent",
                      darkMode 
                        ? "bg-slate-700 border-slate-600 text-white placeholder:text-slate-500" 
                        : "bg-white border-slate-200"
                    )}
                    required
                  />
                </div>
              </div>
              
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className={cn(
                    "block text-sm font-medium mb-1",
                    darkMode ? "text-slate-300" : "text-slate-700"
                  )}>
                    Máximo de usos
                  </label>
                  <input
                    type="number"
                    value={formData.maxUses}
                    onChange={(e) => setFormData({ ...formData, maxUses: e.target.value })}
                    placeholder="Ilimitado"
                    className={cn(
                      "w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent",
                      darkMode 
                        ? "bg-slate-700 border-slate-600 text-white placeholder:text-slate-500" 
                        : "bg-white border-slate-200"
                    )}
                  />
                </div>
                <div>
                  <label className={cn(
                    "block text-sm font-medium mb-1",
                    darkMode ? "text-slate-300" : "text-slate-700"
                  )}>
                    Fecha de expiración
                  </label>
                  <input
                    type="date"
                    value={formData.expiresAt}
                    onChange={(e) => setFormData({ ...formData, expiresAt: e.target.value })}
                    className={cn(
                      "w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent",
                      darkMode 
                        ? "bg-slate-700 border-slate-600 text-white" 
                        : "bg-white border-slate-200"
                    )}
                  />
                </div>
              </div>

              <div>
                <label className={cn(
                  "block text-sm font-medium mb-2",
                  darkMode ? "text-slate-300" : "text-slate-700"
                )}>
                  Aplicable a productos (dejar vacío para todos)
                </label>
                <div className={cn(
                  "space-y-2 max-h-32 overflow-y-auto border rounded-lg p-3",
                  darkMode ? "border-slate-600" : "border-slate-200"
                )}>
                  {products.map((product) => (
                    <label key={product.id} className="flex items-center gap-2 cursor-pointer">
                      <input
                        type="checkbox"
                        checked={formData.applicableProducts.includes(product.id)}
                        onChange={(e) => {
                          if (e.target.checked) {
                            setFormData({
                              ...formData,
                              applicableProducts: [...formData.applicableProducts, product.id]
                            });
                          } else {
                            setFormData({
                              ...formData,
                              applicableProducts: formData.applicableProducts.filter(id => id !== product.id)
                            });
                          }
                        }}
                        className="rounded text-indigo-600"
                      />
                      <span className={cn("text-sm", darkMode ? "text-slate-300" : "text-slate-700")}>
                        {product.name}
                      </span>
                    </label>
                  ))}
                </div>
              </div>
              
              <div className="flex items-center gap-2">
                <input
                  type="checkbox"
                  id="codeActive"
                  checked={formData.active}
                  onChange={(e) => setFormData({ ...formData, active: e.target.checked })}
                  className="rounded text-indigo-600"
                />
                <label 
                  htmlFor="codeActive" 
                  className={cn("text-sm", darkMode ? "text-slate-300" : "text-slate-700")}
                >
                  Código activo
                </label>
              </div>
              
              <div className="flex gap-3 pt-4">
                <button
                  type="button"
                  onClick={closeModal}
                  className={cn(
                    "flex-1 px-4 py-2 border rounded-lg transition-colors",
                    darkMode 
                      ? "border-slate-600 text-slate-300 hover:bg-slate-700" 
                      : "border-slate-200 text-slate-700 hover:bg-slate-50"
                  )}
                >
                  Cancelar
                </button>
                <button
                  type="submit"
                  className="flex-1 px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors"
                >
                  {editingCode ? 'Guardar Cambios' : 'Crear Código'}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}
