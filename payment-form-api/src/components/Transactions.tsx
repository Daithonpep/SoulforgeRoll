import { useState } from 'react';
import { Search, Download, Filter, CheckCircle, Clock, XCircle, RefreshCw, FileText } from 'lucide-react';
import { useApp } from '../context/AppContext';
import { cn } from '../utils/cn';

export function Transactions() {
  const { transactions, darkMode, formStyle } = useApp();
  const [searchTerm, setSearchTerm] = useState('');
  const [statusFilter, setStatusFilter] = useState<string>('all');
  const [methodFilter, setMethodFilter] = useState<string>('all');
  const [selectedInvoice, setSelectedInvoice] = useState<typeof transactions[0] | null>(null);

  const filteredTransactions = transactions.filter((tx) => {
    const matchesSearch = 
      tx.customerName.toLowerCase().includes(searchTerm.toLowerCase()) ||
      tx.customerEmail.toLowerCase().includes(searchTerm.toLowerCase()) ||
      tx.productName.toLowerCase().includes(searchTerm.toLowerCase()) ||
      tx.invoiceId.toLowerCase().includes(searchTerm.toLowerCase()) ||
      (tx.discountCode && tx.discountCode.toLowerCase().includes(searchTerm.toLowerCase()));
    
    const matchesStatus = statusFilter === 'all' || tx.status === statusFilter;
    const matchesMethod = methodFilter === 'all' || tx.paymentMethod === methodFilter;
    
    return matchesSearch && matchesStatus && matchesMethod;
  });

  const totalAmount = filteredTransactions
    .filter(tx => tx.status === 'completed')
    .reduce((sum, tx) => sum + tx.amount, 0);

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'completed': return <CheckCircle size={16} className="text-green-500" />;
      case 'pending': return <Clock size={16} className="text-yellow-500" />;
      case 'failed': return <XCircle size={16} className="text-red-500" />;
      case 'refunded': return <RefreshCw size={16} className="text-slate-500" />;
      default: return null;
    }
  };

  const getStatusLabel = (status: string) => {
    switch (status) {
      case 'completed': return 'Completado';
      case 'pending': return 'Pendiente';
      case 'failed': return 'Fallido';
      case 'refunded': return 'Reembolsado';
      default: return status;
    }
  };

  const exportToCSV = () => {
    const headers = ['Fecha', 'Factura', 'Cliente', 'Email', 'Producto', 'Monto Original', 'Descuento', 'Total', 'Método', 'Estado'];
    const rows = filteredTransactions.map(tx => [
      new Date(tx.createdAt).toLocaleDateString('es-ES'),
      tx.invoiceId,
      tx.customerName,
      tx.customerEmail,
      tx.productName,
      `$${tx.originalAmount.toFixed(2)}`,
      tx.discountCode || '-',
      `$${tx.amount.toFixed(2)}`,
      tx.paymentMethod.toUpperCase(),
      getStatusLabel(tx.status)
    ]);
    
    const csvContent = [headers.join(','), ...rows.map(row => row.join(','))].join('\n');
    const blob = new Blob([csvContent], { type: 'text/csv' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `transacciones-${new Date().toISOString().split('T')[0]}.csv`;
    a.click();
  };

  return (
    <div className="p-6 space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h2 className={cn("text-2xl font-bold", darkMode ? "text-white" : "text-slate-800")}>
            Transacciones
          </h2>
          <p className={cn(darkMode ? "text-slate-400" : "text-slate-500")}>
            Historial de todos los pagos recibidos
          </p>
        </div>
        <button
          onClick={exportToCSV}
          className={cn(
            "flex items-center gap-2 px-4 py-2 rounded-lg transition-colors",
            darkMode 
              ? "bg-slate-700 text-slate-300 hover:bg-slate-600" 
              : "bg-slate-100 text-slate-700 hover:bg-slate-200"
          )}
        >
          <Download size={18} />
          Exportar CSV
        </button>
      </div>

      {/* Stats */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <div className={cn(
          "rounded-xl p-4 shadow-sm border",
          darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100"
        )}>
          <p className={cn("text-sm", darkMode ? "text-slate-400" : "text-slate-500")}>
            Total Recaudado
          </p>
          <p className={cn("text-2xl font-bold", darkMode ? "text-white" : "text-slate-800")}>
            ${totalAmount.toFixed(2)}
          </p>
        </div>
        <div className={cn(
          "rounded-xl p-4 shadow-sm border",
          darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100"
        )}>
          <p className={cn("text-sm", darkMode ? "text-slate-400" : "text-slate-500")}>
            Transacciones
          </p>
          <p className={cn("text-2xl font-bold", darkMode ? "text-white" : "text-slate-800")}>
            {filteredTransactions.length}
          </p>
        </div>
        <div className={cn(
          "rounded-xl p-4 shadow-sm border",
          darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100"
        )}>
          <p className={cn("text-sm", darkMode ? "text-slate-400" : "text-slate-500")}>
            Con Descuento
          </p>
          <p className={cn("text-2xl font-bold", darkMode ? "text-white" : "text-slate-800")}>
            {filteredTransactions.filter(tx => tx.discountCode).length}
          </p>
        </div>
        <div className={cn(
          "rounded-xl p-4 shadow-sm border",
          darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100"
        )}>
          <p className={cn("text-sm", darkMode ? "text-slate-400" : "text-slate-500")}>
            Ticket Promedio
          </p>
          <p className={cn("text-2xl font-bold", darkMode ? "text-white" : "text-slate-800")}>
            ${filteredTransactions.length > 0 
              ? (totalAmount / filteredTransactions.filter(tx => tx.status === 'completed').length || 0).toFixed(2)
              : '0.00'
            }
          </p>
        </div>
      </div>

      {/* Filters */}
      <div className={cn(
        "rounded-xl p-4 shadow-sm border",
        darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100"
      )}>
        <div className="flex flex-wrap gap-4">
          <div className="flex-1 min-w-[200px]">
            <div className="relative">
              <Search className={cn(
                "absolute left-3 top-1/2 -translate-y-1/2",
                darkMode ? "text-slate-500" : "text-slate-400"
              )} size={20} />
              <input
                type="text"
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
                placeholder="Buscar por cliente, email, producto, factura o código..."
                className={cn(
                  "w-full pl-10 pr-4 py-2 border rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent",
                  darkMode 
                    ? "bg-slate-700 border-slate-600 text-white placeholder:text-slate-500" 
                    : "bg-white border-slate-200"
                )}
              />
            </div>
          </div>
          
          <div className="flex items-center gap-2">
            <Filter size={18} className={cn(darkMode ? "text-slate-500" : "text-slate-400")} />
            <select
              value={statusFilter}
              onChange={(e) => setStatusFilter(e.target.value)}
              className={cn(
                "px-4 py-2 border rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent",
                darkMode 
                  ? "bg-slate-700 border-slate-600 text-white" 
                  : "bg-white border-slate-200"
              )}
            >
              <option value="all">Todos los estados</option>
              <option value="completed">Completados</option>
              <option value="pending">Pendientes</option>
              <option value="failed">Fallidos</option>
              <option value="refunded">Reembolsados</option>
            </select>
            
            <select
              value={methodFilter}
              onChange={(e) => setMethodFilter(e.target.value)}
              className={cn(
                "px-4 py-2 border rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent",
                darkMode 
                  ? "bg-slate-700 border-slate-600 text-white" 
                  : "bg-white border-slate-200"
              )}
            >
              <option value="all">Todos los métodos</option>
              <option value="card">Tarjeta</option>
              <option value="stripe">Stripe</option>
              <option value="paypal">PayPal</option>
            </select>
          </div>
        </div>
      </div>

      {/* Table */}
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
                <th className="px-6 py-4 font-medium">Fecha</th>
                <th className="px-6 py-4 font-medium">Factura</th>
                <th className="px-6 py-4 font-medium">Cliente</th>
                <th className="px-6 py-4 font-medium">Producto</th>
                <th className="px-6 py-4 font-medium">Monto</th>
                <th className="px-6 py-4 font-medium">Método</th>
                <th className="px-6 py-4 font-medium">Estado</th>
                <th className="px-6 py-4 font-medium">Acciones</th>
              </tr>
            </thead>
            <tbody className={cn("divide-y", darkMode ? "divide-slate-700" : "divide-slate-100")}>
              {filteredTransactions.map((tx) => (
                <tr key={tx.id} className={cn(
                  "transition-colors",
                  darkMode ? "hover:bg-slate-700/50" : "hover:bg-slate-50"
                )}>
                  <td className={cn("px-6 py-4 text-sm", darkMode ? "text-slate-400" : "text-slate-600")}>
                    {new Date(tx.createdAt).toLocaleDateString('es-ES', {
                      day: '2-digit',
                      month: 'short',
                      year: 'numeric',
                      hour: '2-digit',
                      minute: '2-digit'
                    })}
                  </td>
                  <td className="px-6 py-4">
                    <span className={cn(
                      "px-2 py-1 rounded font-mono text-xs",
                      darkMode ? "bg-slate-700 text-slate-300" : "bg-slate-100 text-slate-600"
                    )}>
                      {tx.invoiceId.slice(0, 15)}...
                    </span>
                  </td>
                  <td className="px-6 py-4">
                    <div>
                      <p className={cn("font-medium", darkMode ? "text-white" : "text-slate-800")}>
                        {tx.customerName}
                      </p>
                      <p className={cn("text-sm", darkMode ? "text-slate-400" : "text-slate-500")}>
                        {tx.customerEmail}
                      </p>
                    </div>
                  </td>
                  <td className={cn("px-6 py-4 text-sm", darkMode ? "text-slate-300" : "text-slate-700")}>
                    {tx.productName}
                  </td>
                  <td className="px-6 py-4">
                    <div>
                      <span className={cn("font-semibold", darkMode ? "text-white" : "text-slate-800")}>
                        ${tx.amount.toFixed(2)} {tx.currency}
                      </span>
                      {tx.discountCode && (
                        <div className="flex items-center gap-1 mt-1">
                          <span className={cn(
                            "px-2 py-0.5 rounded text-xs font-mono",
                            darkMode ? "bg-indigo-900/50 text-indigo-400" : "bg-indigo-50 text-indigo-700"
                          )}>
                            {tx.discountCode}
                          </span>
                          <span className="text-xs text-green-500">
                            -${tx.discountAmount?.toFixed(2)}
                          </span>
                        </div>
                      )}
                    </div>
                  </td>
                  <td className="px-6 py-4">
                    <span className={cn(
                      "px-3 py-1 rounded-full text-xs font-medium uppercase",
                      tx.paymentMethod === 'stripe' 
                        ? "bg-indigo-100 text-indigo-700"
                        : tx.paymentMethod === 'paypal'
                        ? "bg-blue-100 text-blue-700"
                        : "bg-slate-100 text-slate-700"
                    )}>
                      {tx.paymentMethod}
                    </span>
                  </td>
                  <td className="px-6 py-4">
                    <span className={cn(
                      "flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-medium w-fit",
                      tx.status === 'completed' && "bg-green-100 text-green-700",
                      tx.status === 'pending' && "bg-yellow-100 text-yellow-700",
                      tx.status === 'failed' && "bg-red-100 text-red-700",
                      tx.status === 'refunded' && (darkMode ? "bg-slate-600 text-slate-300" : "bg-slate-100 text-slate-600")
                    )}>
                      {getStatusIcon(tx.status)}
                      {getStatusLabel(tx.status)}
                    </span>
                  </td>
                  <td className="px-6 py-4">
                    <button
                      onClick={() => setSelectedInvoice(tx)}
                      className={cn(
                        "p-2 rounded-lg transition-colors",
                        darkMode ? "hover:bg-slate-600" : "hover:bg-slate-100"
                      )}
                      title="Ver factura"
                    >
                      <FileText size={18} className={cn(darkMode ? "text-slate-400" : "text-slate-500")} />
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>

        {filteredTransactions.length === 0 && (
          <div className={cn("text-center py-12", darkMode ? "text-slate-500" : "text-slate-400")}>
            No se encontraron transacciones
          </div>
        )}
      </div>

      {/* Invoice Modal */}
      {selectedInvoice && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
          <div className="bg-white rounded-2xl w-full max-w-2xl shadow-xl max-h-[90vh] overflow-y-auto">
            <div className="flex items-center justify-between p-6 border-b sticky top-0 bg-white">
              <h3 className="text-xl font-semibold text-slate-800 flex items-center gap-2">
                <FileText size={24} className="text-indigo-600" />
                Factura {selectedInvoice.invoiceId}
              </h3>
              <button 
                onClick={() => setSelectedInvoice(null)} 
                className="p-2 hover:bg-slate-100 rounded-lg text-slate-500"
              >
                ✕
              </button>
            </div>
            
            <div className="p-8">
              <div className="flex items-start justify-between mb-8">
                <div>
                  <div 
                    className="w-12 h-12 rounded-lg flex items-center justify-center mb-2"
                    style={{ backgroundColor: formStyle.primaryColor }}
                  >
                    <span className="text-white font-bold text-xl">
                      {formStyle.companyName.charAt(0)}
                    </span>
                  </div>
                  <h4 className="font-bold text-xl">{formStyle.companyName}</h4>
                  <p className="text-sm text-slate-500">{formStyle.invoiceFromEmail}</p>
                </div>
                <div className="text-right">
                  <h3 className="text-2xl font-bold text-slate-800">FACTURA</h3>
                  <p className="text-sm text-slate-500 font-mono">{selectedInvoice.invoiceId}</p>
                  <p className="text-sm text-slate-500 mt-2">
                    Fecha: {new Date(selectedInvoice.createdAt).toLocaleDateString('es-ES', {
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
                  <p className="font-medium text-slate-800">{selectedInvoice.customerName}</p>
                  <p className="text-slate-600">{selectedInvoice.customerEmail}</p>
                  {selectedInvoice.customerPhone && <p className="text-slate-600">{selectedInvoice.customerPhone}</p>}
                  {selectedInvoice.customerCompany && <p className="text-slate-600">{selectedInvoice.customerCompany}</p>}
                  {selectedInvoice.customerAddress && (
                    <p className="text-slate-600">
                      {selectedInvoice.customerAddress}
                      {selectedInvoice.customerCity && `, ${selectedInvoice.customerCity}`}
                      {selectedInvoice.customerZip && ` ${selectedInvoice.customerZip}`}
                      {selectedInvoice.customerCountry && `, ${selectedInvoice.customerCountry}`}
                    </p>
                  )}
                </div>
                <div>
                  <h5 className="text-sm font-medium text-slate-500 mb-2">MÉTODO DE PAGO:</h5>
                  <p className="font-medium text-slate-800 capitalize">
                    {selectedInvoice.paymentMethod === 'card' ? '💳 Tarjeta' : 
                     selectedInvoice.paymentMethod === 'stripe' ? '💳 Stripe' : '🅿️ PayPal'}
                  </p>
                  <p className={cn(
                    "font-medium mt-2",
                    selectedInvoice.status === 'completed' ? "text-green-600" :
                    selectedInvoice.status === 'pending' ? "text-yellow-600" :
                    selectedInvoice.status === 'failed' ? "text-red-600" : "text-slate-600"
                  )}>
                    {selectedInvoice.status === 'completed' ? '✓ Pagado' :
                     selectedInvoice.status === 'pending' ? '⏳ Pendiente' :
                     selectedInvoice.status === 'failed' ? '✕ Fallido' : '↩ Reembolsado'}
                  </p>
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
                      <p className="font-medium text-slate-800">{selectedInvoice.productName}</p>
                    </td>
                    <td className="py-4 text-right font-medium">
                      ${selectedInvoice.originalAmount.toFixed(2)} {selectedInvoice.currency}
                    </td>
                  </tr>
                  {selectedInvoice.discountCode && (
                    <tr className="border-b border-slate-100">
                      <td className="py-4">
                        <p className="text-green-600">Descuento ({selectedInvoice.discountCode})</p>
                      </td>
                      <td className="py-4 text-right font-medium text-green-600">
                        -${selectedInvoice.discountAmount?.toFixed(2)} {selectedInvoice.currency}
                      </td>
                    </tr>
                  )}
                </tbody>
                <tfoot>
                  <tr className="border-t-2 border-slate-200">
                    <td className="py-4 font-bold text-lg">TOTAL</td>
                    <td className="py-4 text-right font-bold text-2xl" style={{ color: formStyle.primaryColor }}>
                      ${selectedInvoice.amount.toFixed(2)} {selectedInvoice.currency}
                    </td>
                  </tr>
                </tfoot>
              </table>

              {selectedInvoice.customerNotes && (
                <div className="mb-6">
                  <h5 className="text-sm font-medium text-slate-500 mb-2">NOTAS:</h5>
                  <p className="text-slate-600 bg-slate-50 p-3 rounded-lg">{selectedInvoice.customerNotes}</p>
                </div>
              )}

              <div className="text-center pt-6 border-t">
                <p className="text-slate-500 text-sm">¡Gracias por tu compra!</p>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
