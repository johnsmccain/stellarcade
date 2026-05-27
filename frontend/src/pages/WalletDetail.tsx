import React, { useState } from 'react';
import { QuickPivotLinks, type PivotLink } from '../components/v1/QuickPivotLinks';
import { PinnedWalletActionTray } from '../components/v1/PinnedWalletActionTray';
import { StatusPill } from '../components/v1/StatusPill';
import { WalletBalanceDeltaCards } from '../components/v1/WalletBalanceDeltaCards';

interface WalletDetailProps {
  walletId?: string;
}

const WalletDetail: React.FC<WalletDetailProps> = ({ walletId = 'wallet_123' }) => {
  const [activeSection, setActiveSection] = useState<string>('overview');

  const pivotLinks: PivotLink[] = [
    { id: 'contracts', label: 'Related Contracts', onClick: () => setActiveSection('contracts'), icon: 'DOC', badge: 5 },
    { id: 'transactions', label: 'Transaction History', onClick: () => setActiveSection('transactions'), icon: 'TX', badge: 23 },
    { id: 'analytics', label: 'Analytics Dashboard', href: `/analytics/wallet/${walletId}`, icon: 'ANL', external: true },
    { id: 'settings', label: 'Wallet Settings', onClick: () => setActiveSection('settings'), icon: 'CFG' },
    { id: 'export', label: 'Export Data', onClick: () => console.log('Export wallet data'), icon: 'EXP', disabled: true },
  ];

  const renderContent = () => {
    switch (activeSection) {
      case 'contracts':
        return (
          <div className="wallet-detail__content">
            <h2>Related Smart Contracts</h2>
            <p>Contracts associated with this wallet...</p>
          </div>
        );
      case 'transactions':
        return (
          <div className="wallet-detail__content">
            <h2>Transaction History</h2>
            <p>Recent transactions for this wallet...</p>
          </div>
        );
      case 'settings':
        return (
          <div className="wallet-detail__content">
            <h2>Wallet Settings</h2>
            <p>Configure wallet preferences and security settings...</p>
          </div>
        );
      default:
        return (
          <div className="wallet-detail__content">
            <h2>Wallet Overview</h2>
            <div style={{ display: 'grid', gap: '1rem', marginTop: '1rem' }}>
              <div style={{ padding: '1rem', border: '1px solid #333', borderRadius: '0.5rem' }}>
                <h3>Balance</h3>
                <p style={{ fontSize: '1.5rem', fontWeight: 'bold', color: '#7de2d1' }}>1,234.56 XLM</p>
              </div>
              <div style={{ padding: '1rem', border: '1px solid #333', borderRadius: '0.5rem' }}>
                <h3>Status</h3>
                <StatusPill tone="success" label="Active" />
              </div>
            </div>
          </div>
        );
    }
  };

  return (
    <main style={{ padding: '2rem', maxWidth: '1200px', margin: '0 auto' }} aria-labelledby="wallet-details-heading">
      <header style={{ marginBottom: '2rem' }}>
        <h1 id="wallet-details-heading">Wallet Details</h1>
        <p style={{ color: '#a8b5c8', fontFamily: 'monospace' }}>{walletId}</p>
      </header>

      <section style={{ marginBottom: '2rem' }} aria-label="Wallet quick navigation">
        <h2 style={{ marginBottom: '1rem', fontSize: '0.875rem', textTransform: 'uppercase', letterSpacing: '0.05em' }}>
          Quick Navigation
        </h2>
        <QuickPivotLinks links={pivotLinks} activeId={activeSection} testId="wallet-detail-pivot-links" />
      </section>

      <section aria-label="Wallet comparison snapshot" style={{ marginBottom: '1.25rem' }}>
        <h2 style={{ marginBottom: '0.75rem' }}>Balance delta comparison</h2>
        <WalletBalanceDeltaCards
          left={{ id: 'current', label: 'Current wallet', currentBalance: 1234.56, previousBalance: 1180.2 }}
          right={{ id: 'comparison', label: 'Comparison wallet', currentBalance: 1088.12, previousBalance: 1114.55 }}
          testId="wallet-balance-delta-cards"
        />
      </section>

      <section aria-label="Wallet detail content">{renderContent()}</section>

      <div style={{ marginTop: '1.5rem' }}>
        <PinnedWalletActionTray
          actions={[
            { id: 'refresh-session', label: 'Refresh session', onClick: () => console.log('refresh wallet session') },
            { id: 'repeat-last-transfer', label: 'Repeat last transfer', onClick: () => console.log('repeat last transfer') },
            { id: 'reconnect-wallet', label: 'Reconnect wallet', onClick: () => console.log('reconnect wallet') },
          ]}
        />
      </div>
    </main>
  );
};

export default WalletDetail;
