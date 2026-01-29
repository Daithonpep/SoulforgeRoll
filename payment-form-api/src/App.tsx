import { AppProvider, useApp } from './context/AppContext';
import { Sidebar } from './components/Sidebar';
import { Dashboard } from './components/Dashboard';
import { Products } from './components/Products';
import { DiscountCodes } from './components/DiscountCodes';
import { Settings } from './components/Settings';
import { FormBuilder } from './components/FormBuilder';
import { Preview } from './components/Preview';
import { Transactions } from './components/Transactions';
import { Integration } from './components/Integration';
import { Profile } from './components/Profile';
import { VendorSettings } from './components/VendorSettings';
import { Collaborators } from './components/Collaborators';
import { cn } from './utils/cn';

function MainContent() {
  const { activeTab, darkMode } = useApp();

  const renderContent = () => {
    switch (activeTab) {
      case 'dashboard':
        return <Dashboard />;
      case 'products':
        return <Products />;
      case 'discounts':
        return <DiscountCodes />;
      case 'settings':
        return <Settings />;
      case 'form-builder':
        return <FormBuilder />;
      case 'preview':
        return <Preview />;
      case 'transactions':
        return <Transactions />;
      case 'integration':
        return <Integration />;
      case 'profile':
        return <Profile />;
      case 'vendor-settings':
        return <VendorSettings />;
      case 'collaborators':
        return <Collaborators />;
      default:
        return <Dashboard />;
    }
  };

  return (
    <div className={cn(
      "flex min-h-screen transition-colors duration-300",
      darkMode ? "bg-slate-900" : "bg-slate-50"
    )}>
      <Sidebar />
      <main className="flex-1 overflow-x-hidden">
        {renderContent()}
      </main>
    </div>
  );
}

export function App() {
  return (
    <AppProvider>
      <MainContent />
    </AppProvider>
  );
}
