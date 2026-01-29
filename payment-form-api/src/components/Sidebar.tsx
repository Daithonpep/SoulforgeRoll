import {
  LayoutDashboard,
  Package,
  Ticket,
  Settings,
  PaintBucket,
  Eye,
  Receipt,
  CreditCard,
  Moon,
  Sun,
  Code,
  User
} from 'lucide-react';
import { useApp } from '../context/AppContext';
import { TabType } from '../types';
import { cn } from '../utils/cn';

const menuItems: { id: TabType; label: string; icon: React.ReactNode }[] = [
  { id: 'dashboard', label: 'Dashboard', icon: <LayoutDashboard size={20} /> },
  { id: 'products', label: 'Productos', icon: <Package size={20} /> },
  { id: 'discounts', label: 'Códigos Descuento', icon: <Ticket size={20} /> },
  { id: 'transactions', label: 'Transacciones', icon: <Receipt size={20} /> },
  { id: 'form-builder', label: 'Personalizar Formulario', icon: <PaintBucket size={20} /> },
  { id: 'preview', label: 'Vista Previa', icon: <Eye size={20} /> },
  { id: 'integration', label: 'Integración / Código', icon: <Code size={20} /> },
  { id: 'settings', label: 'Configuración APIs', icon: <Settings size={20} /> },
  { id: 'profile', label: 'Perfil y Respaldo', icon: <User size={20} /> },
];

export function Sidebar() {
  const { activeTab, setActiveTab, darkMode, toggleDarkMode } = useApp();

  return (
    <aside className={cn(
      "w-64 min-h-screen p-4 flex flex-col transition-colors duration-300",
      darkMode ? "bg-slate-950" : "bg-slate-900"
    )}>
      <div className="flex items-center gap-3 px-3 py-4 mb-6">
        <div className="w-10 h-10 bg-gradient-to-br from-indigo-500 to-purple-600 rounded-xl flex items-center justify-center">
          <CreditCard className="text-white" size={22} />
        </div>
        <div>
          <h1 className="text-white font-bold text-lg">PayForm Pro</h1>
          <p className="text-slate-400 text-xs">Gestión de Pagos</p>
        </div>
      </div>

      <nav className="flex-1 space-y-1">
        {menuItems.map((item) => (
          <button
            key={item.id}
            onClick={() => setActiveTab(item.id)}
            className={cn(
              "w-full flex items-center gap-3 px-4 py-3 rounded-lg text-left transition-all",
              activeTab === item.id
                ? "bg-indigo-600 text-white shadow-lg shadow-indigo-500/30"
                : "text-slate-400 hover:bg-slate-800 hover:text-white"
            )}
          >
            {item.icon}
            <span className="text-sm font-medium">{item.label}</span>
          </button>
        ))}
      </nav>

      <div className="mt-auto pt-4 border-t border-slate-700 space-y-3">
        {/* Dark Mode Toggle */}
        <button
          onClick={toggleDarkMode}
          className={cn(
            "w-full flex items-center justify-between px-4 py-3 rounded-lg transition-all",
            "text-slate-400 hover:bg-slate-800 hover:text-white"
          )}
        >
          <span className="flex items-center gap-3">
            {darkMode ? <Moon size={20} /> : <Sun size={20} />}
            <span className="text-sm font-medium">
              {darkMode ? 'Modo Oscuro' : 'Modo Claro'}
            </span>
          </span>
          <div className={cn(
            "w-10 h-6 rounded-full relative transition-colors",
            darkMode ? "bg-indigo-600" : "bg-slate-600"
          )}>
            <div className={cn(
              "absolute top-1 w-4 h-4 bg-white rounded-full transition-transform",
              darkMode ? "translate-x-5" : "translate-x-1"
            )}></div>
          </div>
        </button>

        <div className={cn(
          "px-4 py-3 rounded-lg",
          darkMode
            ? "bg-gradient-to-r from-indigo-900/50 to-purple-900/50"
            : "bg-gradient-to-r from-indigo-600/20 to-purple-600/20"
        )}>
          <p className="text-xs text-slate-400">Estado del Sistema</p>
          <p className="text-sm text-green-400 font-medium flex items-center gap-2 mt-1">
            <span className="w-2 h-2 bg-green-400 rounded-full animate-pulse"></span>
            Operativo
          </p>
        </div>
      </div>
    </aside>
  );
}
