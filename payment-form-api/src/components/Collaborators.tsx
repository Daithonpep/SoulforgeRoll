import { useState } from 'react';
import {
    Users, Plus, Trash2, Save, Edit2, CheckCircle, AlertTriangle,
    DollarSign, TrendingUp, UserPlus, Copy, ExternalLink
} from 'lucide-react';
import { useApp } from '../context/AppContext';
import { cn } from '../utils/cn';
import { Collaborator } from '../types';

export function Collaborators() {
    const { darkMode } = useApp();
    const [notification, setNotification] = useState<{ type: 'success' | 'error', message: string } | null>(null);
    const [editingId, setEditingId] = useState<string | null>(null);
    const [showAddModal, setShowAddModal] = useState(false);

    // Load from localStorage
    const [collaborators, setCollaborators] = useState<Collaborator[]>(() => {
        const saved = localStorage.getItem('payform_collaborators');
        return saved ? JSON.parse(saved) : [];
    });

    // Form state for new/edit collaborator
    const [formData, setFormData] = useState<Partial<Collaborator>>({
        name: '',
        email: '',
        tag: '',
        commissionType: 'percentage',
        commissionValue: 10,
        payoutMethod: 'paypal',
        paypalEmail: '',
        isActive: true
    });

    const showNotification = (type: 'success' | 'error', message: string) => {
        setNotification({ type, message });
        setTimeout(() => setNotification(null), 3000);
    };

    const saveCollaborators = (newList: Collaborator[]) => {
        localStorage.setItem('payform_collaborators', JSON.stringify(newList));
        setCollaborators(newList);
    };

    const generateTag = (name: string): string => {
        const base = name.toUpperCase().replace(/\s+/g, '').slice(0, 6);
        const random = Math.floor(Math.random() * 100);
        return `${base}${random}`;
    };

    const addCollaborator = () => {
        if (!formData.name || !formData.email) {
            showNotification('error', 'Nombre y email son requeridos');
            return;
        }

        // Check for duplicate tag
        const tag = formData.tag || generateTag(formData.name);
        if (collaborators.some(c => c.tag.toUpperCase() === tag.toUpperCase())) {
            showNotification('error', 'Ese código de afiliado ya existe');
            return;
        }

        const newCollaborator: Collaborator = {
            id: crypto.randomUUID(),
            name: formData.name,
            email: formData.email,
            tag: tag.toUpperCase(),
            commissionType: formData.commissionType || 'percentage',
            commissionValue: formData.commissionValue || 10,
            totalEarnings: 0,
            pendingPayout: 0,
            payoutMethod: formData.payoutMethod || 'paypal',
            paypalEmail: formData.paypalEmail,
            isActive: true,
            createdAt: new Date(),
            referrals: 0
        };

        saveCollaborators([...collaborators, newCollaborator]);
        setFormData({ name: '', email: '', tag: '', commissionType: 'percentage', commissionValue: 10, payoutMethod: 'paypal', paypalEmail: '', isActive: true });
        setShowAddModal(false);
        showNotification('success', `Colaborador ${newCollaborator.name} agregado con código ${newCollaborator.tag}`);
    };

    const updateCollaborator = (id: string, updates: Partial<Collaborator>) => {
        const updated = collaborators.map(c => c.id === id ? { ...c, ...updates } : c);
        saveCollaborators(updated);
        setEditingId(null);
        showNotification('success', 'Colaborador actualizado');
    };

    const deleteCollaborator = (id: string) => {
        if (confirm('¿Estás seguro de eliminar este colaborador?')) {
            saveCollaborators(collaborators.filter(c => c.id !== id));
            showNotification('success', 'Colaborador eliminado');
        }
    };

    const toggleActive = (id: string) => {
        const updated = collaborators.map(c => c.id === id ? { ...c, isActive: !c.isActive } : c);
        saveCollaborators(updated);
    };

    const copyTag = (tag: string) => {
        navigator.clipboard.writeText(tag);
        showNotification('success', `Código ${tag} copiado al portapapeles`);
    };

    // Stats
    const totalEarnings = collaborators.reduce((sum, c) => sum + c.totalEarnings, 0);
    const totalPending = collaborators.reduce((sum, c) => sum + c.pendingPayout, 0);
    const totalReferrals = collaborators.reduce((sum, c) => sum + c.referrals, 0);

    return (
        <div className={cn("p-6 max-w-6xl mx-auto", darkMode && "text-white")}>
            <div className="mb-8">
                <h2 className={cn("text-2xl font-bold", darkMode ? "text-white" : "text-slate-800")}>
                    Sistema de Colaboradores
                </h2>
                <p className={cn(darkMode ? "text-slate-400" : "text-slate-500")}>
                    Gestiona tu red de afiliados y sus comisiones
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

            {/* Stats Cards */}
            <div className="grid grid-cols-1 md:grid-cols-4 gap-4 mb-8">
                <div className={cn("rounded-xl p-4 border", darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100")}>
                    <div className="flex items-center gap-3">
                        <div className="w-10 h-10 bg-indigo-100 rounded-lg flex items-center justify-center">
                            <Users className="text-indigo-600" size={20} />
                        </div>
                        <div>
                            <p className="text-sm text-slate-500">Colaboradores</p>
                            <p className="text-2xl font-bold">{collaborators.length}</p>
                        </div>
                    </div>
                </div>
                <div className={cn("rounded-xl p-4 border", darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100")}>
                    <div className="flex items-center gap-3">
                        <div className="w-10 h-10 bg-green-100 rounded-lg flex items-center justify-center">
                            <DollarSign className="text-green-600" size={20} />
                        </div>
                        <div>
                            <p className="text-sm text-slate-500">Total Ganado</p>
                            <p className="text-2xl font-bold">${totalEarnings.toFixed(2)}</p>
                        </div>
                    </div>
                </div>
                <div className={cn("rounded-xl p-4 border", darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100")}>
                    <div className="flex items-center gap-3">
                        <div className="w-10 h-10 bg-yellow-100 rounded-lg flex items-center justify-center">
                            <TrendingUp className="text-yellow-600" size={20} />
                        </div>
                        <div>
                            <p className="text-sm text-slate-500">Pendiente de Pago</p>
                            <p className="text-2xl font-bold">${totalPending.toFixed(2)}</p>
                        </div>
                    </div>
                </div>
                <div className={cn("rounded-xl p-4 border", darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100")}>
                    <div className="flex items-center gap-3">
                        <div className="w-10 h-10 bg-purple-100 rounded-lg flex items-center justify-center">
                            <ExternalLink className="text-purple-600" size={20} />
                        </div>
                        <div>
                            <p className="text-sm text-slate-500">Referidos Totales</p>
                            <p className="text-2xl font-bold">{totalReferrals}</p>
                        </div>
                    </div>
                </div>
            </div>

            {/* Add Button */}
            <div className="flex justify-end mb-6">
                <button
                    onClick={() => setShowAddModal(true)}
                    className="flex items-center gap-2 px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors"
                >
                    <UserPlus size={18} />
                    Agregar Colaborador
                </button>
            </div>

            {/* Collaborators Table */}
            <div className={cn("rounded-xl shadow-sm border overflow-hidden", darkMode ? "bg-slate-800 border-slate-700" : "bg-white border-slate-100")}>
                {collaborators.length === 0 ? (
                    <div className="text-center py-12">
                        <Users className="mx-auto mb-3 text-slate-400" size={48} />
                        <p className={cn("text-lg font-medium", darkMode ? "text-white" : "text-slate-800")}>
                            No hay colaboradores registrados
                        </p>
                        <p className="text-slate-500 mb-4">Agrega tu primer afiliado para empezar</p>
                        <button
                            onClick={() => setShowAddModal(true)}
                            className="inline-flex items-center gap-2 px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700"
                        >
                            <Plus size={18} />
                            Agregar Colaborador
                        </button>
                    </div>
                ) : (
                    <table className="w-full">
                        <thead className={cn("border-b", darkMode ? "bg-slate-700/50 border-slate-700" : "bg-slate-50 border-slate-100")}>
                            <tr>
                                <th className="text-left px-4 py-3 text-sm font-medium text-slate-500">Colaborador</th>
                                <th className="text-left px-4 py-3 text-sm font-medium text-slate-500">Código</th>
                                <th className="text-left px-4 py-3 text-sm font-medium text-slate-500">Comisión</th>
                                <th className="text-right px-4 py-3 text-sm font-medium text-slate-500">Ganado</th>
                                <th className="text-right px-4 py-3 text-sm font-medium text-slate-500">Referidos</th>
                                <th className="text-center px-4 py-3 text-sm font-medium text-slate-500">Estado</th>
                                <th className="text-right px-4 py-3 text-sm font-medium text-slate-500">Acciones</th>
                            </tr>
                        </thead>
                        <tbody className="divide-y divide-slate-100 dark:divide-slate-700">
                            {collaborators.map((collab) => (
                                <tr key={collab.id} className={cn("hover:bg-slate-50", darkMode && "hover:bg-slate-700/30")}>
                                    <td className="px-4 py-3">
                                        <div>
                                            <p className={cn("font-medium", darkMode ? "text-white" : "text-slate-800")}>{collab.name}</p>
                                            <p className="text-sm text-slate-500">{collab.email}</p>
                                        </div>
                                    </td>
                                    <td className="px-4 py-3">
                                        <div className="flex items-center gap-2">
                                            <code className="px-2 py-1 bg-indigo-100 text-indigo-700 rounded text-sm font-mono">
                                                {collab.tag}
                                            </code>
                                            <button onClick={() => copyTag(collab.tag)} className="text-slate-400 hover:text-indigo-600">
                                                <Copy size={14} />
                                            </button>
                                        </div>
                                    </td>
                                    <td className="px-4 py-3">
                                        <span className={cn("text-sm", darkMode ? "text-white" : "text-slate-700")}>
                                            {collab.commissionValue}{collab.commissionType === 'percentage' ? '%' : ' USD'}
                                        </span>
                                    </td>
                                    <td className="px-4 py-3 text-right">
                                        <span className="font-medium text-green-600">${collab.totalEarnings.toFixed(2)}</span>
                                    </td>
                                    <td className="px-4 py-3 text-right">
                                        <span className={cn(darkMode ? "text-white" : "text-slate-700")}>{collab.referrals}</span>
                                    </td>
                                    <td className="px-4 py-3 text-center">
                                        <button
                                            onClick={() => toggleActive(collab.id)}
                                            className={cn(
                                                "px-2 py-1 rounded-full text-xs font-medium transition-colors",
                                                collab.isActive
                                                    ? "bg-green-100 text-green-700"
                                                    : "bg-red-100 text-red-700"
                                            )}
                                        >
                                            {collab.isActive ? 'Activo' : 'Inactivo'}
                                        </button>
                                    </td>
                                    <td className="px-4 py-3 text-right">
                                        <div className="flex items-center justify-end gap-2">
                                            <button
                                                onClick={() => setEditingId(collab.id)}
                                                className="p-1 text-slate-400 hover:text-indigo-600"
                                            >
                                                <Edit2 size={16} />
                                            </button>
                                            <button
                                                onClick={() => deleteCollaborator(collab.id)}
                                                className="p-1 text-slate-400 hover:text-red-600"
                                            >
                                                <Trash2 size={16} />
                                            </button>
                                        </div>
                                    </td>
                                </tr>
                            ))}
                        </tbody>
                    </table>
                )}
            </div>

            {/* Add/Edit Modal */}
            {showAddModal && (
                <div className="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50">
                    <div className={cn(
                        "w-full max-w-md rounded-xl p-6 shadow-xl",
                        darkMode ? "bg-slate-800" : "bg-white"
                    )}>
                        <h3 className={cn("text-xl font-bold mb-4", darkMode ? "text-white" : "text-slate-800")}>
                            Nuevo Colaborador
                        </h3>

                        <div className="space-y-4">
                            <div>
                                <label className="block text-sm font-medium text-slate-500 mb-1">Nombre *</label>
                                <input
                                    type="text"
                                    value={formData.name || ''}
                                    onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                                    placeholder="Juan Pérez"
                                    className={cn(
                                        "w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-indigo-500",
                                        darkMode ? "bg-slate-700 border-slate-600 text-white" : "bg-white border-slate-200"
                                    )}
                                />
                            </div>

                            <div>
                                <label className="block text-sm font-medium text-slate-500 mb-1">Email *</label>
                                <input
                                    type="email"
                                    value={formData.email || ''}
                                    onChange={(e) => setFormData({ ...formData, email: e.target.value })}
                                    placeholder="juan@email.com"
                                    className={cn(
                                        "w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-indigo-500",
                                        darkMode ? "bg-slate-700 border-slate-600 text-white" : "bg-white border-slate-200"
                                    )}
                                />
                            </div>

                            <div>
                                <label className="block text-sm font-medium text-slate-500 mb-1">
                                    Código de Afiliado (se genera automático si está vacío)
                                </label>
                                <input
                                    type="text"
                                    value={formData.tag || ''}
                                    onChange={(e) => setFormData({ ...formData, tag: e.target.value.toUpperCase() })}
                                    placeholder="JUAN20"
                                    className={cn(
                                        "w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-indigo-500 font-mono uppercase",
                                        darkMode ? "bg-slate-700 border-slate-600 text-white" : "bg-white border-slate-200"
                                    )}
                                />
                            </div>

                            <div className="grid grid-cols-2 gap-4">
                                <div>
                                    <label className="block text-sm font-medium text-slate-500 mb-1">Tipo de Comisión</label>
                                    <select
                                        value={formData.commissionType || 'percentage'}
                                        onChange={(e) => setFormData({ ...formData, commissionType: e.target.value as any })}
                                        className={cn(
                                            "w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-indigo-500",
                                            darkMode ? "bg-slate-700 border-slate-600 text-white" : "bg-white border-slate-200"
                                        )}
                                    >
                                        <option value="percentage">Porcentaje (%)</option>
                                        <option value="fixed">Monto Fijo ($)</option>
                                    </select>
                                </div>
                                <div>
                                    <label className="block text-sm font-medium text-slate-500 mb-1">Valor</label>
                                    <input
                                        type="number"
                                        value={formData.commissionValue || 10}
                                        onChange={(e) => setFormData({ ...formData, commissionValue: parseFloat(e.target.value) })}
                                        min="0"
                                        step="0.5"
                                        className={cn(
                                            "w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-indigo-500",
                                            darkMode ? "bg-slate-700 border-slate-600 text-white" : "bg-white border-slate-200"
                                        )}
                                    />
                                </div>
                            </div>

                            <div>
                                <label className="block text-sm font-medium text-slate-500 mb-1">Método de Pago</label>
                                <select
                                    value={formData.payoutMethod || 'paypal'}
                                    onChange={(e) => setFormData({ ...formData, payoutMethod: e.target.value as any })}
                                    className={cn(
                                        "w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-indigo-500",
                                        darkMode ? "bg-slate-700 border-slate-600 text-white" : "bg-white border-slate-200"
                                    )}
                                >
                                    <option value="paypal">PayPal</option>
                                    <option value="bank">Transferencia Bancaria</option>
                                    <option value="crypto">Criptomonedas</option>
                                </select>
                            </div>

                            {formData.payoutMethod === 'paypal' && (
                                <div>
                                    <label className="block text-sm font-medium text-slate-500 mb-1">Email de PayPal</label>
                                    <input
                                        type="email"
                                        value={formData.paypalEmail || ''}
                                        onChange={(e) => setFormData({ ...formData, paypalEmail: e.target.value })}
                                        placeholder="juan@paypal.com"
                                        className={cn(
                                            "w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-indigo-500",
                                            darkMode ? "bg-slate-700 border-slate-600 text-white" : "bg-white border-slate-200"
                                        )}
                                    />
                                </div>
                            )}
                        </div>

                        <div className="flex justify-end gap-3 mt-6">
                            <button
                                onClick={() => setShowAddModal(false)}
                                className={cn(
                                    "px-4 py-2 rounded-lg transition-colors",
                                    darkMode ? "bg-slate-700 text-slate-300 hover:bg-slate-600" : "bg-slate-100 text-slate-600 hover:bg-slate-200"
                                )}
                            >
                                Cancelar
                            </button>
                            <button
                                onClick={addCollaborator}
                                className="px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700"
                            >
                                Agregar Colaborador
                            </button>
                        </div>
                    </div>
                </div>
            )}
        </div>
    );
}
