import { Component, ErrorInfo, ReactNode } from "react";

interface Props {
    children?: ReactNode;
}

interface State {
    hasError: boolean;
    error: Error | null;
}

export class ErrorBoundary extends Component<Props, State> {
    public state: State = {
        hasError: false,
        error: null,
    };

    public static getDerivedStateFromError(error: Error): State {
        return { hasError: true, error };
    }

    public componentDidCatch(error: Error, errorInfo: ErrorInfo) {
        console.error("Uncaught error:", error, errorInfo);
    }

    public render() {
        if (this.state.hasError) {
            return (
                <div className="p-8 max-w-2xl mx-auto mt-10 bg-red-50 border border-red-200 rounded-lg shadow-lg">
                    <h1 className="text-2xl font-bold text-red-800 mb-4">Algo salió mal</h1>
                    <p className="text-red-700 mb-4">
                        Ha ocurrido un error en la aplicación. Por favor recarga la página o contacta soporte.
                    </p>
                    <div className="bg-white p-4 rounded border border-red-100 overflow-auto">
                        <p className="font-mono text-sm text-red-600 whitespace-pre-wrap">
                            {this.state.error?.message}
                        </p>
                        <p className="font-mono text-xs text-gray-500 mt-2">
                            {this.state.error?.stack}
                        </p>
                    </div>
                    <button
                        onClick={() => window.location.reload()}
                        className="mt-6 px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700 transition-colors"
                    >
                        Recargar Página
                    </button>
                </div>
            );
        }

        return this.props.children;
    }
}
