import { useState } from 'react';
import { Plus, Pencil, Trash2, X, Package } from 'lucide-react';
import { useApp } from '../context/AppContext';
import { Product } from '../types';
import { cn } from '../utils/cn';

export function Products() {
  const { products, addProduct, updateProduct, deleteProduct, darkMode } = useApp();
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [editingProduct, setEditingProduct] = useState<Product | null>(null);
  const [formData, setFormData] = useState({
    name: '',
    description: '',
    price: '',
    currency: 'USD',
    type: 'one-time' as 'one-time' | 'subscription',
    interval: 'monthly' as 'monthly' | 'yearly' | 'weekly',
    active: true,
  });

  const openModal = (product?: Product) => {
    if (product) {
      setEditingProduct(product);
      setFormData({
        name: product.name,
        description: product.description,
        price: product.price.toString(),
        currency: product.currency,
        type: product.type,
        interval: product.interval || 'monthly',
        active: product.active,
      });
    } else {
      setEditingProduct(null);
      setFormData({
        name: '',
        description: '',
        price: '',
        currency: 'USD',
        type: 'one-time',
        interval: 'monthly',
        active: true,
      });
    }
    setIsModalOpen(true);
  };

  const closeModal = () => {
    setIsModalOpen(false);
    setEditingProduct(null);
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const productData = {
      name: formData.name,
      description: formData.description,
      price: parseFloat(formData.price),
      currency: formData.currency,
      type: formData.type,
      interval: formData.type === 'subscription' ? formData.interval : undefined,
      active: formData.active,
    };

    if (editingProduct) {
      updateProduct(editingProduct.id, productData);
    } else {
      addProduct(productData);
    }
    closeModal();
  };

  return (
    <div className="p-6 space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h2 className={cn("text-2xl font-bold", darkMode ? "text-white" : "text-slate-800")}>
            Productos y Planes
          </h2>
          <p className={cn(darkMode ? "text-slate-400" : "text-slate-500")}>
            Gestiona tus productos y suscripciones
          </p>
        </div>
        <button
          onClick={() => openModal()}
          className="flex items-center gap-2 px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors"
        >
          <Plus size={20} />
          Nuevo Producto
        </button>
      </div>

      {/* Products Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {products.map((product) => (
          <div
            key={product.id}
            className={cn(
              "rounded-xl p-6 shadow-sm border transition-all hover:shadow-md",
              darkMode 
                ? product.active 
                  ? "bg-slate-800 border-slate-700" 
                  : "bg-slate-800/50 border-slate-700 opacity-60"
                : product.active 
                  ? "bg-white border-slate-100" 
                  : "bg-white border-slate-200 opacity-60"
            )}
          >
            <div className="flex items-start justify-between mb-4">
              <div className={cn(
                "p-3 rounded-lg",
                product.type === 'subscription' 
                  ? darkMode ? "bg-purple-900/30" : "bg-purple-50" 
                  : darkMode ? "bg-blue-900/30" : "bg-blue-50"
              )}>
                <Package className={product.type === 'subscription' ? "text-purple-500" : "text-blue-500"} size={24} />
              </div>
              <div className="flex items-center gap-2">
                <button
                  onClick={() => openModal(product)}
                  className={cn(
                    "p-2 rounded-lg transition-colors",
                    darkMode ? "hover:bg-slate-700" : "hover:bg-slate-100"
                  )}
                >
                  <Pencil size={16} className={cn(darkMode ? "text-slate-400" : "text-slate-500")} />
                </button>
                <button
                  onClick={() => deleteProduct(product.id)}
                  className="p-2 hover:bg-red-50 rounded-lg transition-colors"
                >
                  <Trash2 size={16} className="text-red-500" />
                </button>
              </div>
            </div>
            
            <h3 className={cn("text-lg font-semibold", darkMode ? "text-white" : "text-slate-800")}>
              {product.name}
            </h3>
            <p className={cn("text-sm mt-1 line-clamp-2", darkMode ? "text-slate-400" : "text-slate-500")}>
              {product.description}
            </p>
            
            <div className="mt-4 flex items-baseline gap-1">
              <span className={cn("text-3xl font-bold", darkMode ? "text-white" : "text-slate-800")}>
                ${product.price}
              </span>
              <span className={cn("text-sm", darkMode ? "text-slate-400" : "text-slate-500")}>
                {product.currency}
                {product.type === 'subscription' && `/${product.interval === 'monthly' ? 'mes' : product.interval === 'yearly' ? 'año' : 'semana'}`}
              </span>
            </div>

            <div className="mt-4 flex items-center gap-2">
              <span className={cn(
                "px-2 py-1 rounded-full text-xs font-medium",
                product.type === 'subscription' 
                  ? "bg-purple-100 text-purple-700" 
                  : "bg-blue-100 text-blue-700"
              )}>
                {product.type === 'subscription' ? 'Suscripción' : 'Pago único'}
              </span>
              <span className={cn(
                "px-2 py-1 rounded-full text-xs font-medium",
                product.active 
                  ? "bg-green-100 text-green-700" 
                  : darkMode ? "bg-slate-600 text-slate-300" : "bg-slate-100 text-slate-500"
              )}>
                {product.active ? 'Activo' : 'Inactivo'}
              </span>
            </div>
          </div>
        ))}
      </div>

      {products.length === 0 && (
        <div className={cn(
          "text-center py-12 rounded-xl border",
          darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100"
        )}>
          <Package className={cn("mx-auto", darkMode ? "text-slate-600" : "text-slate-300")} size={48} />
          <p className={cn("mt-4", darkMode ? "text-slate-400" : "text-slate-500")}>
            No hay productos creados
          </p>
          <button
            onClick={() => openModal()}
            className="mt-4 text-indigo-500 hover:underline font-medium"
          >
            Crear el primero
          </button>
        </div>
      )}

      {/* Modal */}
      {isModalOpen && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
          <div className={cn(
            "rounded-2xl w-full max-w-lg shadow-xl",
            darkMode ? "bg-slate-800" : "bg-white"
          )}>
            <div className={cn(
              "flex items-center justify-between p-6 border-b",
              darkMode ? "border-slate-700" : "border-slate-200"
            )}>
              <h3 className={cn("text-xl font-semibold", darkMode ? "text-white" : "text-slate-800")}>
                {editingProduct ? 'Editar Producto' : 'Nuevo Producto'}
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
                  Nombre del producto
                </label>
                <input
                  type="text"
                  value={formData.name}
                  onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                  className={cn(
                    "w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent",
                    darkMode 
                      ? "bg-slate-700 border-slate-600 text-white" 
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
                  Descripción
                </label>
                <textarea
                  value={formData.description}
                  onChange={(e) => setFormData({ ...formData, description: e.target.value })}
                  rows={3}
                  className={cn(
                    "w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent",
                    darkMode 
                      ? "bg-slate-700 border-slate-600 text-white" 
                      : "bg-white border-slate-200"
                  )}
                />
              </div>
              
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className={cn(
                    "block text-sm font-medium mb-1",
                    darkMode ? "text-slate-300" : "text-slate-700"
                  )}>
                    Precio
                  </label>
                  <input
                    type="number"
                    step="0.01"
                    value={formData.price}
                    onChange={(e) => setFormData({ ...formData, price: e.target.value })}
                    className={cn(
                      "w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent",
                      darkMode 
                        ? "bg-slate-700 border-slate-600 text-white" 
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
                    Moneda
                  </label>
                  <select
                    value={formData.currency}
                    onChange={(e) => setFormData({ ...formData, currency: e.target.value })}
                    className={cn(
                      "w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent",
                      darkMode 
                        ? "bg-slate-700 border-slate-600 text-white" 
                        : "bg-white border-slate-200"
                    )}
                  >
                    <option value="USD">USD</option>
                    <option value="EUR">EUR</option>
                    <option value="MXN">MXN</option>
                    <option value="COP">COP</option>
                  </select>
                </div>
              </div>
              
              <div>
                <label className={cn(
                  "block text-sm font-medium mb-2",
                  darkMode ? "text-slate-300" : "text-slate-700"
                )}>
                  Tipo de producto
                </label>
                <div className="flex gap-4">
                  <label className="flex items-center gap-2 cursor-pointer">
                    <input
                      type="radio"
                      value="one-time"
                      checked={formData.type === 'one-time'}
                      onChange={(e) => setFormData({ ...formData, type: e.target.value as 'one-time' | 'subscription' })}
                      className="text-indigo-600"
                    />
                    <span className={cn("text-sm", darkMode ? "text-slate-300" : "text-slate-700")}>
                      Pago único
                    </span>
                  </label>
                  <label className="flex items-center gap-2 cursor-pointer">
                    <input
                      type="radio"
                      value="subscription"
                      checked={formData.type === 'subscription'}
                      onChange={(e) => setFormData({ ...formData, type: e.target.value as 'one-time' | 'subscription' })}
                      className="text-indigo-600"
                    />
                    <span className={cn("text-sm", darkMode ? "text-slate-300" : "text-slate-700")}>
                      Suscripción
                    </span>
                  </label>
                </div>
              </div>
              
              {formData.type === 'subscription' && (
                <div>
                  <label className={cn(
                    "block text-sm font-medium mb-1",
                    darkMode ? "text-slate-300" : "text-slate-700"
                  )}>
                    Intervalo de cobro
                  </label>
                  <select
                    value={formData.interval}
                    onChange={(e) => setFormData({ ...formData, interval: e.target.value as 'monthly' | 'yearly' | 'weekly' })}
                    className={cn(
                      "w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent",
                      darkMode 
                        ? "bg-slate-700 border-slate-600 text-white" 
                        : "bg-white border-slate-200"
                    )}
                  >
                    <option value="weekly">Semanal</option>
                    <option value="monthly">Mensual</option>
                    <option value="yearly">Anual</option>
                  </select>
                </div>
              )}
              
              <div className="flex items-center gap-2">
                <input
                  type="checkbox"
                  id="active"
                  checked={formData.active}
                  onChange={(e) => setFormData({ ...formData, active: e.target.checked })}
                  className="rounded text-indigo-600"
                />
                <label 
                  htmlFor="active" 
                  className={cn("text-sm", darkMode ? "text-slate-300" : "text-slate-700")}
                >
                  Producto activo
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
                  {editingProduct ? 'Guardar Cambios' : 'Crear Producto'}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}
