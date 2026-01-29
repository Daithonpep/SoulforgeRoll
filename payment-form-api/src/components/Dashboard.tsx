import {
  TrendingUp,
  DollarSign,
  Users,
  CreditCard,
  ArrowUpRight,
  ArrowDownRight
} from 'lucide-react';
import { useApp } from '../context/AppContext';
import { cn } from '../utils/cn';

export function Dashboard() {
  const { products, discountCodes, transactions, paymentConfig, darkMode } = useApp();

  const totalRevenue = transactions
    .filter(t => t.status === 'completed')
    .reduce((sum, t) => sum + t.amount, 0);

  const activeProducts = products.filter(p => p.active).length;
  const activeCodes = discountCodes.filter(c => c.active).length;
  const totalDiscountUses = discountCodes.reduce((sum, c) => sum + c.currentUses, 0);

  const stats = [
    {
      title: 'Ingresos Totales',
      value: `$${totalRevenue.toFixed(2)}`,
      change: totalRevenue > 0 ? '+100%' : '',
      positive: true,
      icon: <DollarSign className="text-green-500" size={24} />,
      bgColor: darkMode ? 'bg-green-900/30' : 'bg-green-50',
    },
    {
      title: 'Productos Activos',
      value: activeProducts,
      change: products.length > 0 ? `${products.length} total` : '',
      positive: true,
      icon: <CreditCard className="text-blue-500" size={24} />,
      bgColor: darkMode ? 'bg-blue-900/30' : 'bg-blue-50',
    },
    {
      title: 'Códigos Usados',
      value: totalDiscountUses,
      change: activeCodes > 0 ? `${activeCodes} activos` : '',
      positive: true,
      icon: <Users className="text-purple-500" size={24} />,
      bgColor: darkMode ? 'bg-purple-900/30' : 'bg-purple-50',
    },
    {
      title: 'Transacciones',
      value: transactions.length,
      change: transactions.length > 0 ? '100%' : '',
      positive: true,
      icon: <TrendingUp className="text-orange-500" size={24} />,
      bgColor: darkMode ? 'bg-orange-900/30' : 'bg-orange-50',
    },
  ];

  return (
    <div className="p-6 space-y-6">
      <div>
        <h2 className={cn("text-2xl font-bold", darkMode ? "text-white" : "text-slate-800")}>Dashboard</h2>
        <p className={cn(darkMode ? "text-slate-400" : "text-slate-500")}>Resumen general de tu sistema de pagos</p>
      </div>

      {/* Stats Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        {stats.map((stat, index) => (
          <div
            key={index}
            className={cn(
              "rounded-xl p-6 shadow-sm border transition-shadow hover:shadow-md",
              darkMode
                ? "bg-slate-800 border-slate-700"
                : "bg-white border-slate-100"
            )}
          >
            <div className="flex items-start justify-between">
              <div className={cn("p-3 rounded-lg", stat.bgColor)}>
                {stat.icon}
              </div>
              {stat.change && (
                <span className={cn(
                  "flex items-center text-sm font-medium",
                  stat.positive ? 'text-green-500' : 'text-red-500'
                )}>
                  {stat.positive ? <ArrowUpRight size={16} /> : <ArrowDownRight size={16} />}
                  {stat.change}
                </span>
              )}
            </div>
            <div className="mt-4">
              <p className={cn("text-2xl font-bold", darkMode ? "text-white" : "text-slate-800")}>{stat.value}</p>
              <p className={cn("text-sm", darkMode ? "text-slate-400" : "text-slate-500")}>{stat.title}</p>
            </div>
          </div>
        ))}
      </div>

      {/* API Status & Recent Activity */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* API Status */}
        <div className={cn(
          "rounded-xl p-6 shadow-sm border",
          darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100"
        )}>
          <h3 className={cn("text-lg font-semibold mb-4", darkMode ? "text-white" : "text-slate-800")}>
            Estado de APIs
          </h3>
          <div className="space-y-4">
            <div className={cn(
              "flex items-center justify-between p-4 rounded-lg",
              darkMode ? "bg-slate-700/50" : "bg-slate-50"
            )}>
              <div className="flex items-center gap-3">
                <div className={cn(
                  "w-10 h-10 rounded-lg flex items-center justify-center",
                  darkMode ? "bg-indigo-900/50" : "bg-indigo-100"
                )}>
                  <span className="font-bold text-indigo-500">S</span>
                </div>
                <div>
                  <p className={cn("font-medium", darkMode ? "text-white" : "text-slate-800")}>Stripe</p>
                  <p className={cn("text-sm", darkMode ? "text-slate-400" : "text-slate-500")}>Pagos con tarjeta</p>
                </div>
              </div>
              <span className={cn(
                "px-3 py-1 rounded-full text-sm font-medium",
                paymentConfig.stripeEnabled
                  ? 'bg-green-100 text-green-700'
                  : darkMode ? 'bg-slate-600 text-slate-300' : 'bg-slate-100 text-slate-500'
              )}>
                {paymentConfig.stripeEnabled ? 'Conectado' : 'No configurado'}
              </span>
            </div>

            <div className={cn(
              "flex items-center justify-between p-4 rounded-lg",
              darkMode ? "bg-slate-700/50" : "bg-slate-50"
            )}>
              <div className="flex items-center gap-3">
                <div className={cn(
                  "w-10 h-10 rounded-lg flex items-center justify-center",
                  darkMode ? "bg-blue-900/50" : "bg-blue-100"
                )}>
                  <span className="font-bold text-blue-500">P</span>
                </div>
                <div>
                  <p className={cn("font-medium", darkMode ? "text-white" : "text-slate-800")}>PayPal</p>
                  <p className={cn("text-sm", darkMode ? "text-slate-400" : "text-slate-500")}>Pagos digitales</p>
                </div>
              </div>
              <span className={cn(
                "px-3 py-1 rounded-full text-sm font-medium",
                paymentConfig.paypalEnabled
                  ? 'bg-green-100 text-green-700'
                  : darkMode ? 'bg-slate-600 text-slate-300' : 'bg-slate-100 text-slate-500'
              )}>
                {paymentConfig.paypalEnabled ? 'Conectado' : 'No configurado'}
              </span>
            </div>
          </div>
        </div>

        {/* Recent Transactions */}
        <div className={cn(
          "rounded-xl p-6 shadow-sm border",
          darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100"
        )}>
          <h3 className={cn("text-lg font-semibold mb-4", darkMode ? "text-white" : "text-slate-800")}>
            Transacciones Recientes
          </h3>
          <div className="space-y-3">
            {transactions.slice(0, 5).map((tx) => (
              <div
                key={tx.id}
                className={cn(
                  "flex items-center justify-between p-3 rounded-lg transition-colors",
                  darkMode ? "hover:bg-slate-700/50" : "hover:bg-slate-50"
                )}
              >
                <div className="flex items-center gap-3">
                  <div className={cn(
                    "w-2 h-2 rounded-full",
                    tx.status === 'completed' ? 'bg-green-500' :
                      tx.status === 'pending' ? 'bg-yellow-500' :
                        tx.status === 'failed' ? 'bg-red-500' : 'bg-slate-400'
                  )}></div>
                  <div>
                    <p className={cn("font-medium text-sm", darkMode ? "text-white" : "text-slate-800")}>
                      {tx.customerName}
                    </p>
                    <p className={cn("text-xs", darkMode ? "text-slate-400" : "text-slate-500")}>
                      {tx.productName}
                    </p>
                  </div>
                </div>
                <div className="text-right">
                  <p className={cn("font-semibold", darkMode ? "text-white" : "text-slate-800")}>
                    ${tx.amount.toFixed(2)}
                  </p>
                  <p className={cn("text-xs uppercase", darkMode ? "text-slate-400" : "text-slate-500")}>
                    {tx.paymentMethod}
                  </p>
                </div>
              </div>
            ))}
            {transactions.length === 0 && (
              <p className={cn("text-center py-8", darkMode ? "text-slate-500" : "text-slate-400")}>
                No hay transacciones aún
              </p>
            )}
          </div>
        </div>
      </div>

      {/* Top Discount Codes */}
      <div className={cn(
        "rounded-xl p-6 shadow-sm border",
        darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100"
      )}>
        <h3 className={cn("text-lg font-semibold mb-4", darkMode ? "text-white" : "text-slate-800")}>
          Códigos de Descuento Populares
        </h3>
        <div className="overflow-x-auto">
          <table className="w-full">
            <thead>
              <tr className={cn(
                "text-left text-sm border-b",
                darkMode ? "text-slate-400 border-slate-700" : "text-slate-500 border-slate-200"
              )}>
                <th className="pb-3 font-medium">Código</th>
                <th className="pb-3 font-medium">Creador</th>
                <th className="pb-3 font-medium">Descuento</th>
                <th className="pb-3 font-medium">Usos</th>
                <th className="pb-3 font-medium">Estado</th>
              </tr>
            </thead>
            <tbody className={cn("divide-y", darkMode ? "divide-slate-700" : "divide-slate-100")}>
              {discountCodes.slice(0, 5).map((code) => (
                <tr key={code.id} className="text-sm">
                  <td className="py-3">
                    <span className={cn(
                      "px-2 py-1 rounded font-mono font-medium",
                      darkMode ? "bg-indigo-900/50 text-indigo-400" : "bg-indigo-50 text-indigo-700"
                    )}>
                      {code.code}
                    </span>
                  </td>
                  <td className={cn("py-3", darkMode ? "text-slate-300" : "text-slate-700")}>
                    {code.creatorName}
                  </td>
                  <td className={cn("py-3", darkMode ? "text-slate-300" : "text-slate-700")}>
                    {code.discountType === 'percentage' ? `${code.discountValue}%` : `$${code.discountValue}`}
                  </td>
                  <td className={cn("py-3", darkMode ? "text-slate-300" : "text-slate-700")}>
                    {code.currentUses}{code.maxUses ? `/${code.maxUses}` : ''}
                  </td>
                  <td className="py-3">
                    <span className={cn(
                      "px-2 py-1 rounded-full text-xs font-medium",
                      code.active
                        ? 'bg-green-100 text-green-700'
                        : darkMode ? 'bg-slate-600 text-slate-300' : 'bg-slate-100 text-slate-500'
                    )}>
                      {code.active ? 'Activo' : 'Inactivo'}
                    </span>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}
