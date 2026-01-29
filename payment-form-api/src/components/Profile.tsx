import { useState } from 'react';
import { User, Save, Upload, Download, AlertTriangle, CheckCircle, RefreshCw } from 'lucide-react';
import { useApp } from '../context/AppContext';
import { cn } from '../utils/cn';

export function Profile() {
    const {
        darkMode,
        products,
        discountCodes,
        paymentConfig,
        formStyle,
        formConfig,
        transactions,
        reorderFields // dummy usage if needed, but we mostly just need data
    } = useApp();

    // Local state for user profile (simulated since we don't have a backend specifically for this)
    const [userProfile, setUserProfile] = useState(() => {
        const saved = localStorage.getItem('payform_userProfile');
        return saved ? JSON.parse(saved) : {
            name: 'Usuario',
            email: '',
            businessName: 'Mi Negocio'
        };
    });

    const [notification, setNotification] = useState<{ type: 'success' | 'error', message: string } | null>(null);

    const saveProfile = () => {
        localStorage.setItem('payform_userProfile', JSON.stringify(userProfile));
        showNotification('success', 'Perfil actualizado correctamente');
    };

    const showNotification = (type: 'success' | 'error', message: string) => {
        setNotification({ type, message });
        setTimeout(() => setNotification(null), 3000);
    };

    const handleExportConfig = () => {
        const backupData = {
            version: '1.0',
            timestamp: new Date().toISOString(),
            userProfile,
            products,
            discountCodes,
            paymentConfig,
            formStyle,
            formConfig,
            // We generally don't export transactions for privacy/security reasons in a simple client config backup,
            // but if the user wants "everything", we can include them or offer a separate export.
            // Let's include everything for their request "se me borra todo".
            transactions
        };

        const dataStr = "data:text/json;charset=utf-8," + encodeURIComponent(JSON.stringify(backupData, null, 2));
        const downloadAnchorNode = document.createElement('a');
        downloadAnchorNode.setAttribute("href", dataStr);
        downloadAnchorNode.setAttribute("download", `payment_backup_${new Date().toISOString().slice(0, 10)}.json`);
        document.body.appendChild(downloadAnchorNode); // required for firefox
        downloadAnchorNode.click();
        downloadAnchorNode.remove();

        showNotification('success', 'Configuración exportada exitosamente');
    };

    const handleImportConfig = (event: React.ChangeEvent<HTMLInputElement>) => {
        const file = event.target.files?.[0];
        if (!file) return;

        const reader = new FileReader();
        reader.onload = (e) => {
            try {
                const json = JSON.parse(e.target?.result as string);

                // Basic validation
                if (!json.version) throw new Error('Formato de archivo inválido');

                // Restore everything to localStorage
                // Note: This relies on AppContext loading from localStorage on refresh, 
                // OR we would need "setters" for everything in AppContext. 
                // Since AppContext initializes from localStorage, forcefully updating localStorage
                // and then reloading the page is the safest way to ensure deep state update without massive context types.

                if (json.products) localStorage.setItem('payform_products', JSON.stringify(json.products));
                if (json.discountCodes) localStorage.setItem('payform_discountCodes', JSON.stringify(json.discountCodes));
                if (json.paymentConfig) localStorage.setItem('payform_paymentConfig', JSON.stringify(json.paymentConfig));
                if (json.formStyle) localStorage.setItem('payform_formStyle', JSON.stringify(json.formStyle));
                if (json.formConfig) localStorage.setItem('payform_formConfig', JSON.stringify(json.formConfig));
                if (json.transactions) localStorage.setItem('payform_transactions', JSON.stringify(json.transactions));
                if (json.userProfile) {
                    localStorage.setItem('payform_userProfile', JSON.stringify(json.userProfile));
                    setUserProfile(json.userProfile);
                }

                alert('Backup restaurado correctamente. La página se recargará para aplicar los cambios.');
                window.location.reload();

            } catch (error) {
                console.error(error);
                showNotification('error', 'Error al leer el archivo de respaldo');
            }
        };
        reader.readAsText(file);
    };

    return (
        <div className={cn("p-6 max-w-4xl mx-auto", darkMode && "text-white")}>
            <div className="mb-8">
                <h2 className={cn("text-2xl font-bold", darkMode ? "text-white" : "text-slate-800")}>
                    Mi Perfil y Respaldo
                </h2>
                <p className={cn(darkMode ? "text-slate-400" : "text-slate-500")}>
                    Gestiona tus datos y asegura tu configuración
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

            <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
                {/* Profile Card */}
                <div className={cn(
                    "md:col-span-2 rounded-xl p-6 shadow-sm border",
                    darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100"
                )}>
                    <div className="flex items-center gap-3 mb-6">
                        <div className="w-12 h-12 bg-indigo-100 rounded-full flex items-center justify-center">
                            <User className="text-indigo-600" size={24} />
                        </div>
                        <div>
                            <h3 className={cn("text-lg font-semibold", darkMode ? "text-white" : "text-slate-800")}>
                                Información del Negocio
                            </h3>
                            <p className={cn("text-sm", darkMode ? "text-slate-400" : "text-slate-500")}>
                                Datos asociados a tu configuración
                            </p>
                        </div>
                    </div>

                    <div className="space-y-4">
                        <div>
                            <label className={cn("block text-sm font-medium mb-1", darkMode ? "text-slate-300" : "text-slate-700")}>
                                Nombre del Negocio
                            </label>
                            <input
                                type="text"
                                value={userProfile.businessName}
                                onChange={(e) => setUserProfile({ ...userProfile, businessName: e.target.value })}
                                className={cn(
                                    "w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-indigo-500 outline-none transition-colors",
                                    darkMode
                                        ? "bg-slate-700 border-slate-600 text-white placeholder-slate-400"
                                        : "bg-white border-slate-200 text-slate-900"
                                )}
                            />
                        </div>

                        <div>
                            <label className={cn("block text-sm font-medium mb-1", darkMode ? "text-slate-300" : "text-slate-700")}>
                                Nombre de Contacto
                            </label>
                            <input
                                type="text"
                                value={userProfile.name}
                                onChange={(e) => setUserProfile({ ...userProfile, name: e.target.value })}
                                className={cn(
                                    "w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-indigo-500 outline-none transition-colors",
                                    darkMode
                                        ? "bg-slate-700 border-slate-600 text-white placeholder-slate-400"
                                        : "bg-white border-slate-200 text-slate-900"
                                )}
                            />
                        </div>

                        <div>
                            <label className={cn("block text-sm font-medium mb-1", darkMode ? "text-slate-300" : "text-slate-700")}>
                                Email Administrativo (Opcional)
                            </label>
                            <input
                                type="email"
                                value={userProfile.email}
                                onChange={(e) => setUserProfile({ ...userProfile, email: e.target.value })}
                                className={cn(
                                    "w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-indigo-500 outline-none transition-colors",
                                    darkMode
                                        ? "bg-slate-700 border-slate-600 text-white placeholder-slate-400"
                                        : "bg-white border-slate-200 text-slate-900"
                                )}
                            />
                        </div>

                        <div className="pt-4">
                            <button
                                onClick={saveProfile}
                                className="flex items-center gap-2 px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors"
                            >
                                <Save size={18} />
                                Guardar Perfil
                            </button>
                        </div>
                    </div>
                </div>

                {/* Backup Actions */}
                <div className="space-y-6">
                    <div className={cn(
                        "rounded-xl p-6 shadow-sm border",
                        darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100"
                    )}>
                        <div className="flex items-center gap-3 mb-4">
                            <Download className="text-green-500" size={24} />
                            <h3 className={cn("font-semibold", darkMode ? "text-white" : "text-slate-800")}>
                                Exportar Backup
                            </h3>
                        </div>
                        <p className={cn("text-sm mb-4", darkMode ? "text-slate-400" : "text-slate-500")}>
                            Descarga un archivo con toda tu configuración, productos y claves.
                        </p>
                        <button
                            onClick={handleExportConfig}
                            className={cn(
                                "w-full flex items-center justify-center gap-2 px-4 py-2 border rounded-lg transition-colors",
                                darkMode
                                    ? "border-green-800 text-green-500 hover:bg-green-900/20"
                                    : "border-green-200 text-green-700 hover:bg-green-50"
                            )}
                        >
                            <Download size={18} />
                            Descargar JSON
                        </button>
                    </div>

                    <div className={cn(
                        "rounded-xl p-6 shadow-sm border",
                        darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100"
                    )}>
                        <div className="flex items-center gap-3 mb-4">
                            <Upload className="text-blue-500" size={24} />
                            <h3 className={cn("font-semibold", darkMode ? "text-white" : "text-slate-800")}>
                                Restaurar Backup
                            </h3>
                        </div>
                        <p className={cn("text-sm mb-4", darkMode ? "text-slate-400" : "text-slate-500")}>
                            Sube tu archivo .json para recuperar toda tu configuración al instante.
                        </p>
                        <div className="relative">
                            <input
                                type="file"
                                accept=".json"
                                onChange={handleImportConfig}
                                className="absolute inset-0 w-full h-full opacity-0 cursor-pointer"
                            />
                            <button
                                className={cn(
                                    "w-full flex items-center justify-center gap-2 px-4 py-2 border rounded-lg transition-colors",
                                    darkMode
                                        ? "border-blue-800 text-blue-500 hover:bg-blue-900/20"
                                        : "border-blue-200 text-blue-700 hover:bg-blue-50"
                                )}
                            >
                                <Upload size={18} />
                                Subir Archivo
                            </button>
                        </div>
                        <p className="text-xs text-center mt-2 text-slate-500">
                            Advertencia: Esto reemplazará la configuración actual.
                        </p>
                    </div>
                </div>
            </div>
        </div>
    );
}
